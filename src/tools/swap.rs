use crate::ethereum::client::EthereumClient;
use crate::ethereum::types::SwapResult;
use anyhow::{anyhow, bail, Context, Result};
use ethers::{
    middleware::Middleware,
    types::Address,
};
use rust_decimal::{prelude::FromPrimitive, Decimal};
use rust_decimal_macros::dec;
use serde_json;
use std::str::FromStr;
use tracing::info;

#[allow(dead_code)]
const DEFAULT_DEADLINE_SECS: u64 = 15 * 60;

#[allow(dead_code)]
pub async fn swap_tokens(
    client: &EthereumClient,
    from_token: &str,
    to_token: &str,
    amount: &str,
    slippage_tolerance: f64,
) -> Result<String> {
    info!(
        "模拟代币兑换 - from: {} to: {}, amount: {}, slippage: {}%",
        from_token, to_token, amount, slippage_tolerance
    );

    let from_token = Address::from_str(from_token).context("解析 from_token 失败")?;
    let to_token = Address::from_str(to_token).context("解析 to_token 失败")?;

    let input_amount = Decimal::from_str(amount).context("解析兑换数量失败")?;
    if input_amount <= Decimal::ZERO {
        bail!("兑换数量必须大于 0");
    }

    let from_info = client.get_token_info(from_token).await?;
    let to_info = client.get_token_info(to_token).await?;

    let amount_in = crate::ethereum::client::decimal_to_units(input_amount, from_info.decimals)?;
    let quote = client
        .quote_best_swap(
            from_token,
            from_info.decimals,
            to_token,
            to_info.decimals,
            amount_in,
        )
        .await?;

    let output_amount =
        crate::ethereum::client::units_to_decimal(quote.amount_out, to_info.decimals)?;
    let slippage = Decimal::from_f64(slippage_tolerance).ok_or_else(|| anyhow!("解析滑点失败"))?;
    if slippage < Decimal::ZERO {
        bail!("滑点不能为负");
    }

    let gas_estimate = estimate_gas(client, &quote).await?;
    let gas_price_raw = client
        .provider()
        .get_gas_price()
        .await
        .context("获取 gas price 失败")?;
    let gas_price = crate::ethereum::client::units_to_decimal(gas_price_raw, 9)?; // Convert from wei to gwei
    let total_cost = gas_price * Decimal::from_u64(gas_estimate).unwrap_or(dec!(0));

    let minimum_output = output_amount * (Decimal::ONE - slippage / dec!(100));

    let (protocol, router_address, fee_tier, transaction_data) = match quote.protocol {
        crate::ethereum::client::SwapProtocol::UniswapV2 => {
            let min_out_units =
                crate::ethereum::client::decimal_to_units(minimum_output, to_info.decimals)?;
            let tx = client
                .build_uniswap_v2_swap_tx(
                    amount_in,
                    min_out_units,
                    quote.path.clone(),
                    client.wallet_address(),
                    DEFAULT_DEADLINE_SECS,
                )
                .await?;
            let signed = client.sign_transaction(tx).await?;
            (
                quote.protocol.as_str().to_string(),
                quote.router,
                None,
                format!("0x{}", hex::encode(signed)),
            )
        }
        crate::ethereum::client::SwapProtocol::UniswapV3 => {
            let fee = quote.fee.ok_or_else(|| anyhow!("V3 报价缺少费率信息"))?;
            let min_out_units =
                crate::ethereum::client::decimal_to_units(minimum_output, to_info.decimals)?;
            let tx = client
                .build_uniswap_v3_swap_tx(
                    from_token,
                    to_token,
                    fee,
                    amount_in,
                    min_out_units,
                    client.wallet_address(),
                    DEFAULT_DEADLINE_SECS,
                )
                .await?;
            let signed = client.sign_transaction(tx).await?;
            (
                quote.protocol.as_str().to_string(),
                quote.router,
                Some(fee),
                format!("0x{}", hex::encode(signed)),
            )
        }
    };

    let swap_result = SwapResult {
        from_token: format_address(from_token),
        to_token: format_address(to_token),
        input_amount,
        output_amount,
        price_impact: quote.price_impact_pct,
        gas_estimate,
        gas_price,
        total_cost,
        slippage_tolerance: slippage,
        minimum_output,
        protocol,
        fee_tier,
        router_address: format_address(router_address),
        path: quote
            .path
            .iter()
            .map(|addr| format_address(*addr))
            .collect(),
        transaction_data,
    };

    let result = serde_json::to_string_pretty(&swap_result)?;
    info!("兑换模拟完成: {}", result);

    Ok(result)
}

#[allow(dead_code)]
fn format_address(addr: Address) -> String {
    format!("0x{:x}", addr)
}

#[allow(dead_code)]
async fn estimate_gas(
    client: &EthereumClient,
    quote: &crate::ethereum::client::SwapQuote,
) -> Result<u64> {
    let tx = match quote.protocol {
        crate::ethereum::client::SwapProtocol::UniswapV2 => {
            client
                .build_uniswap_v2_swap_tx(
                    quote.amount_in,
                    quote.amount_out,
                    quote.path.clone(),
                    client.wallet_address(),
                    DEFAULT_DEADLINE_SECS,
                )
                .await?
        }
        crate::ethereum::client::SwapProtocol::UniswapV3 => {
            let fee = quote.fee.ok_or_else(|| anyhow!("V3 报价缺少费率信息"))?;
            client
                .build_uniswap_v3_swap_tx(
                    quote.token_in,
                    quote.token_out,
                    fee,
                    quote.amount_in,
                    quote.amount_out,
                    client.wallet_address(),
                    DEFAULT_DEADLINE_SECS,
                )
                .await?
        }
    };

    let gas = client
        .provider()
        .estimate_gas(&tx, None)
        .await
        .context("估算 gas 失败")?;

    Ok(gas.as_u64())
}
