use crate::ethereum::client::EthereumClient;
use anyhow::Result;
use serde_json;
use tracing::info;

#[allow(dead_code)]
pub async fn get_balance(
    client: &EthereumClient,
    address: &str,
    token_address: Option<&str>,
) -> Result<String> {
    info!("查询余额 - address: {}, token: {:?}", address, token_address);

    let balance = if let Some(token_addr) = token_address {
        client.get_erc20_balance(address, token_addr).await?
    } else {
        client.get_eth_balance(address).await?
    };

    let result = serde_json::to_string_pretty(&balance)?;
    info!("余额查询完成: {}", result);

    Ok(result)
}
