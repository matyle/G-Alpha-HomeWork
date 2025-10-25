use crate::ethereum::types::{Balance, TokenInfo, TokenPrice};
use anyhow::{anyhow, bail, Context, Result};
use ethers::{
    contract::abigen,
    middleware::Middleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{transaction::eip2718::TypedTransaction, Address, Bytes, TransactionRequest, U256},
};
use once_cell::sync::Lazy;
use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;
use std::{
    collections::HashMap,
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::try_join;
use tracing::info;

abigen!(
    IERC20,
    r#"[
        {"type": "function", "name": "balanceOf", "inputs": [{"name": "account", "type": "address"}], "outputs": [{"name": "", "type": "uint256"}], "stateMutability": "view"},
        {"type": "function", "name": "decimals", "inputs": [], "outputs": [{"name": "", "type": "uint8"}], "stateMutability": "view"},
        {"type": "function", "name": "symbol", "inputs": [], "outputs": [{"name": "", "type": "string"}], "stateMutability": "view"},
        {"type": "function", "name": "name", "inputs": [], "outputs": [{"name": "", "type": "string"}], "stateMutability": "view"}
    ]"#
);

abigen!(
    UniswapV2Router,
    r#"[
        {"type": "function", "name": "getAmountsOut", "inputs": [{"name": "amountIn", "type": "uint256"}, {"name": "path", "type": "address[]"}], "outputs": [{"name": "amounts", "type": "uint256[]"}], "stateMutability": "view"},
        {"type": "function", "name": "swapExactTokensForTokens", "inputs": [{"name": "amountIn", "type": "uint256"}, {"name": "amountOutMin", "type": "uint256"}, {"name": "path", "type": "address[]"}, {"name": "to", "type": "address"}, {"name": "deadline", "type": "uint256"}], "outputs": [{"name": "amounts", "type": "uint256[]"}], "stateMutability": "nonpayable"}
    ]"#
);

abigen!(
    UniswapV2Factory,
    r#"[
        {"type": "function", "name": "getPair", "inputs": [{"name": "tokenA", "type": "address"}, {"name": "tokenB", "type": "address"}], "outputs": [{"name": "", "type": "address"}], "stateMutability": "view"}
    ]"#
);

abigen!(
    UniswapV3Quoter,
    r#"[
        {"type": "function", "name": "quoteExactInputSingle", "inputs": [{"name": "tokenIn", "type": "address"}, {"name": "tokenOut", "type": "address"}, {"name": "fee", "type": "uint24"}, {"name": "amountIn", "type": "uint256"}, {"name": "sqrtPriceLimitX96", "type": "uint160"}], "outputs": [{"name": "amountOut", "type": "uint256"}], "stateMutability": "nonpayable"}
    ]"#
);

abigen!(
    UniswapV3Router,
    r#"[
        {"type": "function", "name": "exactInputSingle", "inputs": [{"name": "tokenIn", "type": "address"}, {"name": "tokenOut", "type": "address"}, {"name": "fee", "type": "uint24"}, {"name": "recipient", "type": "address"}, {"name": "deadline", "type": "uint256"}, {"name": "amountIn", "type": "uint256"}, {"name": "amountOutMinimum", "type": "uint256"}, {"name": "sqrtPriceLimitX96", "type": "uint160"}], "outputs": [{"name": "amountOut", "type": "uint256"}], "stateMutability": "payable"}
    ]"#
);

#[allow(dead_code)]
static WETH_ADDRESS: Lazy<Address> = Lazy::new(|| {
    Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").expect("invalid WETH address")
});
#[allow(dead_code)]
static USDC_ADDRESS: Lazy<Address> = Lazy::new(|| {
    Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").expect("invalid USDC address")
});
#[allow(dead_code)]
static UNISWAP_V2_ROUTER: Lazy<Address> = Lazy::new(|| {
    Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").expect("invalid V2 router")
});
#[allow(dead_code)]
static UNISWAP_V2_FACTORY: Lazy<Address> = Lazy::new(|| {
    Address::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").expect("invalid V2 factory")
});
#[allow(dead_code)]
static UNISWAP_V3_QUOTER: Lazy<Address> = Lazy::new(|| {
    Address::from_str("0xb27308f9F90D607463bb33eA1BeBb41C27CE5AB6").expect("invalid V3 quoter")
});
#[allow(dead_code)]
static UNISWAP_V3_ROUTER: Lazy<Address> = Lazy::new(|| {
    Address::from_str("0xE592427A0AEce92De3Edee1F18E0157C05861564").expect("invalid V3 router")
});

#[allow(dead_code)]
static TOKEN_SYMBOLS: Lazy<HashMap<&'static str, Address>> = Lazy::new(|| {
    HashMap::from([
        ("USDC", *USDC_ADDRESS),
        (
            "USDT",
            Address::from_str("0xdAC17F958D2ee523a2206206994597C13D831ec7").unwrap(),
        ),
        (
            "DAI",
            Address::from_str("0x6B175474E89094C44Da98b954EedeAC495271d0F").unwrap(),
        ),
        ("WETH", *WETH_ADDRESS),
        (
            "WBTC",
            Address::from_str("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599").unwrap(),
        ),
    ])
});

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum SwapProtocol {
    UniswapV2,
    UniswapV3,
}

impl SwapProtocol {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            SwapProtocol::UniswapV2 => "UniswapV2",
            SwapProtocol::UniswapV3 => "UniswapV3",
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SwapQuote {
    pub protocol: SwapProtocol,
    pub router: Address,
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: U256,
    pub amount_out: U256,
    pub path: Vec<Address>,
    pub fee: Option<u32>,
    pub price_impact_pct: Decimal,
}

#[allow(dead_code)]
pub struct EthereumClient {
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
    chain_id: u64,
}

#[allow(dead_code)]
impl EthereumClient {
    pub async fn new(rpc_url: String, private_key: String) -> Result<Self> {
        let provider = Provider::<Http>::try_from(&rpc_url)
            .context("创建 Provider 失败，请检查 ETHEREUM_RPC_URL")?;

        let chain_id = provider
            .get_chainid()
            .await
            .context("获取链 ID 失败，请确保 RPC 可用")?
            .as_u64();

        let wallet = LocalWallet::from_str(&private_key)
            .context("解析 PRIVATE_KEY 失败，请检查格式和 0x 前缀")?
            .with_chain_id(chain_id);

        info!(
            wallet = %format_address(wallet.address()),
            chain_id,
            rpc = %rpc_url,
            "Ethereum client initialized"
        );

        Ok(Self {
            provider: Arc::new(provider),
            wallet,
            chain_id,
        })
    }

    pub async fn get_eth_balance(&self, address: &str) -> Result<Balance> {
        let address = Address::from_str(address)?;
        let balance_wei = self.provider.get_balance(address, None).await?;
        let balance_eth = units_to_decimal(balance_wei, 18)?;
        
        Ok(Balance {
            address: format_address(address),
            token_address: None,
            symbol: "ETH".to_string(),
            balance: balance_eth,
            decimals: 18,
            formatted_balance: format!("{:.6} ETH", balance_eth),
        })
    }

    pub async fn get_erc20_balance(&self, address: &str, token_address: &str) -> Result<Balance> {
        let address = Address::from_str(address)?;
        let token_address = Address::from_str(token_address)?;
        let token_info = self.get_token_info(token_address).await?;
        let erc20 = IERC20::new(token_address, self.provider.clone());

        let balance_raw = erc20
            .balance_of(address)
            .call()
            .await
            .context("调用 balanceOf 失败")?;

        let balance = units_to_decimal(balance_raw, token_info.decimals)?;
        let symbol = token_info.symbol.clone();

        Ok(Balance {
            address: format_address(address),
            token_address: Some(format_address(token_address)),
            symbol,
            balance,
            decimals: token_info.decimals,
            formatted_balance: format!("{:.6} {}", balance, token_info.symbol),
        })
    }

    pub async fn get_token_info(&self, token_address: Address) -> Result<TokenInfo> {
        if token_address == Address::zero() {
            return Ok(TokenInfo {
                address: format_address(Address::zero()),
                symbol: "ETH".to_string(),
                name: "Ethereum".to_string(),
                decimals: 18,
                is_native: true,
            });
        }

        let erc20 = IERC20::new(token_address, self.provider.clone());

        let symbol_call = erc20.symbol();
        let name_call = erc20.name();
        let decimals_call = erc20.decimals();

        let (symbol, name, decimals) = try_join!(
            symbol_call.call(),
            name_call.call(),
            decimals_call.call()
        )
        .map_err(|err| anyhow!("查询 ERC20 元数据失败: {err}"))?;

        Ok(TokenInfo {
            address: format_address(token_address),
            symbol,
            name,
            decimals,
            is_native: false,
        })
    }

    pub async fn get_token_price(
        &self,
        token_address: Option<&str>,
        symbol: Option<&str>,
        quote_currency: &str,
    ) -> Result<TokenPrice> {
        let quote_currency = quote_currency.to_uppercase();
        let token_address = match (token_address, symbol) {
            (Some(addr), _) => Address::from_str(addr).context("解析 token_address 失败")?,
            (None, Some(sym)) => {
                let addr = self
                    .resolve_token_address(sym)
                    .ok_or_else(|| anyhow!("未知代币符号: {sym}"))?;
                addr
            }
            _ => bail!("需要提供 token_address 或 symbol"),
        };

        let token_info = self.get_token_info(token_address).await?;

        let price = match quote_currency.as_str() {
            "ETH" => self.get_price_in_eth(token_address, &token_info).await?,
            "USD" => self.get_price_in_usd(token_address, &token_info).await?,
            other => bail!("暂不支持的报价币种: {other}"),
        };

        Ok(TokenPrice {
            token_address: Some(format_address(token_address)),
            symbol: token_info.symbol,
            price,
            quote_currency,
            timestamp: current_timestamp()?,
        })
    }

    pub async fn quote_best_swap(
        &self,
        token_in: Address,
        token_in_decimals: u8,
        token_out: Address,
        token_out_decimals: u8,
        amount_in: U256,
    ) -> Result<SwapQuote> {
        if token_in == token_out {
            bail!("输入与输出代币相同，无需交换");
        }

        let mut candidates: Vec<SwapQuote> = Vec::new();

        if let Some(v3_quote) = self
            .quote_uniswap_v3(
                token_in,
                token_in_decimals,
                token_out,
                token_out_decimals,
                amount_in,
            )
            .await?
        {
            candidates.push(v3_quote);
        }

        if let Some(v2_quote) = self
            .quote_uniswap_v2(
                token_in,
                token_in_decimals,
                token_out,
                token_out_decimals,
                amount_in,
            )
            .await?
        {
            candidates.push(v2_quote);
        }

        candidates
            .into_iter()
            .max_by(|a, b| a.amount_out.cmp(&b.amount_out))
            .ok_or_else(|| anyhow!("未能在 Uniswap V2/V3 上找到可用报价"))
    }

    pub async fn build_uniswap_v2_swap_tx(
        &self,
        amount_in: U256,
        amount_out_min: U256,
        path: Vec<Address>,
        recipient: Address,
        deadline_secs: u64,
    ) -> Result<TypedTransaction> {
        let deadline = self.deadline_after(deadline_secs)?;
        let router = UniswapV2Router::new(*UNISWAP_V2_ROUTER, self.provider.clone());
        let call = router.swap_exact_tokens_for_tokens(
            amount_in,
            amount_out_min,
            path,
            recipient,
            deadline,
        );
        let calldata = call
            .calldata()
            .ok_or_else(|| anyhow!("构造 V2 swap calldata 失败"))?
            .clone();

        let mut tx: TypedTransaction = TransactionRequest::new()
            .from(self.wallet.address())
            .to(*UNISWAP_V2_ROUTER)
            .data(calldata)
            .value(U256::zero())
            .into();

        tx.set_chain_id(self.chain_id);
        Ok(tx)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn build_uniswap_v3_swap_tx(
        &self,
        token_in: Address,
        token_out: Address,
        fee: u32,
        amount_in: U256,
        amount_out_min: U256,
        recipient: Address,
        deadline_secs: u64,
    ) -> Result<TypedTransaction> {
        let deadline = self.deadline_after(deadline_secs)?;
        let router = UniswapV3Router::new(*UNISWAP_V3_ROUTER, self.provider.clone());
        let call = router.exact_input_single(
            token_in,
            token_out,
            fee,
            recipient,
            deadline,
            amount_in,
            amount_out_min,
            U256::zero(),
        );
        let calldata = call
            .calldata()
            .ok_or_else(|| anyhow!("构造 V3 swap calldata 失败"))?
            .clone();

        let mut tx: TypedTransaction = TransactionRequest::new()
            .from(self.wallet.address())
            .to(*UNISWAP_V3_ROUTER)
            .data(calldata)
            .value(U256::zero())
            .into();

        tx.set_chain_id(self.chain_id);
        Ok(tx)
    }

    pub async fn sign_transaction(&self, mut tx: TypedTransaction) -> Result<Bytes> {
        if tx.from().is_none() {
            tx.set_from(self.wallet.address());
        }

        if tx.chain_id().is_none() {
            tx.set_chain_id(self.chain_id);
        }

        if tx.gas().is_none() {
            let gas = self.provider.estimate_gas(&tx, None).await?;
            tx.set_gas(gas);
        }

        if tx.gas_price().is_none() {
            let gas_price = self.provider.get_gas_price().await?;
            tx.set_gas_price(gas_price);
        }

        if tx.nonce().is_none() {
            let nonce = self
                .provider
                .get_transaction_count(self.wallet.address(), None)
                .await?;
            tx.set_nonce(nonce);
        }

        let signature = self.wallet.sign_transaction(&tx).await?;
        Ok(tx.rlp_signed(&signature))
    }

    pub fn provider(&self) -> Arc<Provider<Http>> {
        self.provider.clone()
    }

    pub fn wallet_address(&self) -> Address {
        self.wallet.address()
    }

    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    async fn get_price_in_eth(&self, token: Address, token_info: &TokenInfo) -> Result<Decimal> {
        if token == *WETH_ADDRESS {
            return Ok(dec!(1));
        }

        let amount_in = decimal_to_units(dec!(1), token_info.decimals)?;
        let quote = self
            .quote_best_swap(token, token_info.decimals, *WETH_ADDRESS, 18, amount_in)
            .await?;

        units_to_decimal(quote.amount_out, 18)
    }

    async fn get_price_in_usd(&self, token: Address, token_info: &TokenInfo) -> Result<Decimal> {
        let price_in_eth = self.get_price_in_eth(token, token_info).await?;
        let eth_price_usd = self.get_eth_price_in_usd().await?;
        Ok(price_in_eth * eth_price_usd)
    }

    async fn get_eth_price_in_usd(&self) -> Result<Decimal> {
        let amount_in = U256::exp10(18);
        let quote = self
            .quote_best_swap(*WETH_ADDRESS, 18, *USDC_ADDRESS, 6, amount_in)
            .await?;

        units_to_decimal(quote.amount_out, 6)
    }

    async fn quote_uniswap_v2(
        &self,
        token_in: Address,
        token_in_decimals: u8,
        token_out: Address,
        token_out_decimals: u8,
        amount_in: U256,
    ) -> Result<Option<SwapQuote>> {
        let router = UniswapV2Router::new(*UNISWAP_V2_ROUTER, self.provider.clone());
        let mut best: Option<SwapQuote> = None;

        for path in self.v2_candidate_paths(token_in, token_out).await? {
            let amounts = match router.get_amounts_out(amount_in, path.clone()).call().await {
                Ok(amounts) => amounts,
                Err(err) => {
                    if is_bad_path_error(&err) {
                        continue;
                    }
                    return Err(err.into());
                }
            };

            if let Some(amount_out) = amounts.last() {
                let price_impact = self
                    .estimate_price_impact_v2(
                        token_in_decimals,
                        token_out_decimals,
                        amount_in,
                        *amount_out,
                        path.clone(),
                    )
                    .await?;

                let quote = SwapQuote {
                    protocol: SwapProtocol::UniswapV2,
                    router: *UNISWAP_V2_ROUTER,
                    token_in,
                    token_out,
                    amount_in,
                    amount_out: *amount_out,
                    path: path.clone(),
                    fee: None,
                    price_impact_pct: price_impact,
                };

                if best
                    .as_ref()
                    .is_none_or(|q| quote.amount_out > q.amount_out)
                {
                    best = Some(quote);
                }
            }
        }

        Ok(best)
    }

    async fn quote_uniswap_v3(
        &self,
        token_in: Address,
        token_in_decimals: u8,
        token_out: Address,
        token_out_decimals: u8,
        amount_in: U256,
    ) -> Result<Option<SwapQuote>> {
        let quoter = UniswapV3Quoter::new(*UNISWAP_V3_QUOTER, self.provider.clone());
        let mut best: Option<SwapQuote> = None;
        for &fee in &[500_u32, 3000_u32, 10000_u32] {
            match quoter
                .quote_exact_input_single(token_in, token_out, fee, amount_in, U256::zero())
                .call()
                .await
            {
                Ok(amount_out) if !amount_out.is_zero() => {
                    let price_impact = self
                        .estimate_price_impact_v3(
                            token_in_decimals,
                            token_out_decimals,
                            amount_in,
                            amount_out,
                            token_in,
                            token_out,
                            fee,
                        )
                        .await?;

                    let quote = SwapQuote {
                        protocol: SwapProtocol::UniswapV3,
                        router: *UNISWAP_V3_ROUTER,
                        token_in,
                        token_out,
                        amount_in,
                        amount_out,
                        path: vec![token_in, token_out],
                        fee: Some(fee),
                        price_impact_pct: price_impact,
                    };

                    if best.as_ref().is_none_or(|q| amount_out > q.amount_out) {
                        best = Some(quote);
                    }
                }
                Ok(_) => continue,
                Err(_) => continue,
            }
        }

        Ok(best)
    }

    async fn v2_candidate_paths(
        &self,
        token_in: Address,
        token_out: Address,
    ) -> Result<Vec<Vec<Address>>> {
        let mut paths = Vec::new();

        if self.v2_pair_exists(token_in, token_out).await? {
            paths.push(vec![token_in, token_out]);
        }

        if token_in != *WETH_ADDRESS 
            && token_out != *WETH_ADDRESS
            && self.v2_pair_exists(token_in, *WETH_ADDRESS).await?
            && self.v2_pair_exists(*WETH_ADDRESS, token_out).await?
        {
            paths.push(vec![token_in, *WETH_ADDRESS, token_out]);
        }

        Ok(paths)
    }

    async fn v2_pair_exists(&self, token_a: Address, token_b: Address) -> Result<bool> {
        if token_a == token_b {
            return Ok(false);
        }
        let factory = UniswapV2Factory::new(*UNISWAP_V2_FACTORY, self.provider.clone());
        let pair = factory.get_pair(token_a, token_b).call().await?;
        Ok(pair != Address::zero())
    }

    async fn estimate_price_impact_v2(
        &self,
        token_in_decimals: u8,
        token_out_decimals: u8,
        amount_in: U256,
        amount_out: U256,
        path: Vec<Address>,
    ) -> Result<Decimal> {
        let sample_in = sample_amount(amount_in);
        if sample_in == amount_in || sample_in.is_zero() {
            return Ok(Decimal::ZERO);
        }

        let router = UniswapV2Router::new(*UNISWAP_V2_ROUTER, self.provider.clone());
        let sample_out = router
            .get_amounts_out(sample_in, path)
            .call()
            .await
            .unwrap_or_default();

        let sample_last = match sample_out.last() {
            Some(value) => *value,
            None => return Ok(Decimal::ZERO),
        };

        let spot_price = units_to_decimal(sample_last, token_out_decimals)?
            / units_to_decimal(sample_in, token_in_decimals)?;
        if spot_price.is_zero() {
            return Ok(Decimal::ZERO);
        }

        let executed_price = units_to_decimal(amount_out, token_out_decimals)?
            / units_to_decimal(amount_in, token_in_decimals)?;

        Ok(((spot_price - executed_price) / spot_price).abs() * dec!(100))
    }

    #[allow(clippy::too_many_arguments)]
    async fn estimate_price_impact_v3(
        &self,
        token_in_decimals: u8,
        token_out_decimals: u8,
        amount_in: U256,
        amount_out: U256,
        token_in: Address,
        token_out: Address,
        fee: u32,
    ) -> Result<Decimal> {
        let sample_in = sample_amount(amount_in);
        if sample_in == amount_in || sample_in.is_zero() {
            return Ok(Decimal::ZERO);
        }

        let quoter = UniswapV3Quoter::new(*UNISWAP_V3_QUOTER, self.provider.clone());
        let sample_out = quoter
            .quote_exact_input_single(token_in, token_out, fee, sample_in, U256::zero())
            .call()
            .await
            .unwrap_or(U256::zero());

        if sample_out.is_zero() {
            return Ok(Decimal::ZERO);
        }

        let spot_price = units_to_decimal(sample_out, token_out_decimals)?
            / units_to_decimal(sample_in, token_in_decimals)?;
        if spot_price.is_zero() {
            return Ok(Decimal::ZERO);
        }

        let executed_price = units_to_decimal(amount_out, token_out_decimals)?
            / units_to_decimal(amount_in, token_in_decimals)?;

        Ok(((spot_price - executed_price) / spot_price).abs() * dec!(100))
    }

    fn deadline_after(&self, seconds: u64) -> Result<U256> {
        let deadline = SystemTime::now()
            .checked_add(Duration::from_secs(seconds))
            .ok_or_else(|| anyhow!("deadline overflow"))?
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        Ok(U256::from(deadline))
    }

    fn resolve_token_address(&self, symbol: &str) -> Option<Address> {
        let key = symbol.to_ascii_uppercase();
        TOKEN_SYMBOLS.get(key.as_str()).copied()
    }
}

fn format_address(address: Address) -> String {
    format!("0x{:x}", address)
}

fn sample_amount(amount: U256) -> U256 {
    let candidate = amount
        .checked_div(U256::from(100))
        .unwrap_or_else(U256::zero);
    if candidate.is_zero() {
        U256::one()
    } else {
        candidate
    }
}

pub(crate) fn units_to_decimal(value: U256, decimals: u8) -> Result<Decimal> {
    let value_str = value.to_string();
    let base = Decimal::from_str(&value_str)
        .map_err(|_| anyhow!("数值 {value} 超出 rust_decimal 支持范围"))?;
    let scale = Decimal::from_u128(10u128.pow(decimals as u32))
        .ok_or_else(|| anyhow!("数值超出支持范围"))?;
    Ok(base / scale)
}

pub(crate) fn decimal_to_units(amount: Decimal, decimals: u8) -> Result<U256> {
    if amount < Decimal::ZERO {
        bail!("数量不能为负");
    }

    let scale = Decimal::from_u128(10u128.pow(decimals as u32))
        .ok_or_else(|| anyhow!("数值超出支持范围"))?;
    let scaled = (amount * scale).round_dp(0).trunc();
    let scaled_str = scaled.to_string();
    U256::from_dec_str(scaled_str.as_str()).map_err(|err| anyhow!("转换数量失败: {err}"))
}

fn current_timestamp() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}

fn is_bad_path_error(error: &ethers::contract::ContractError<ethers::providers::Provider<ethers::providers::Http>>) -> bool {
    error
        .to_string()
        .contains("UniswapV2Library: INSUFFICIENT_LIQUIDITY")
}
