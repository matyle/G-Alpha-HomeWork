use crate::ethereum::client::EthereumClient;
use anyhow::Result;
use serde_json;
use tracing::info;

#[allow(dead_code)]
pub async fn get_token_price(
    client: &EthereumClient,
    token_address: Option<&str>,
    symbol: Option<&str>,
    quote_currency: &str,
) -> Result<String> {
    info!("查询代币价格 - address: {:?}, symbol: {:?}, quote currency: {}", token_address, symbol, quote_currency);

    let price = client
        .get_token_price(token_address, symbol, quote_currency)
        .await?;
    let result = serde_json::to_string_pretty(&price)?;

    info!("代币价格查询完成: {}", result);
    Ok(result)
}
