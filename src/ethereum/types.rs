use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub address: String,
    pub token_address: Option<String>,
    pub symbol: String,
    pub balance: Decimal,
    pub decimals: u8,
    pub formatted_balance: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPrice {
    pub token_address: Option<String>,
    pub symbol: String,
    pub price: Decimal,
    pub quote_currency: String,
    pub timestamp: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapResult {
    pub from_token: String,
    pub to_token: String,
    pub input_amount: Decimal,
    pub output_amount: Decimal,
    pub price_impact: Decimal,
    pub gas_estimate: u64,
    pub gas_price: Decimal,
    pub total_cost: Decimal,
    pub slippage_tolerance: Decimal,
    pub minimum_output: Decimal,
    pub protocol: String,
    pub fee_tier: Option<u32>,
    pub router_address: String,
    pub path: Vec<String>,
    pub transaction_data: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: u8,
    pub is_native: bool,
}
