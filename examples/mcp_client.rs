use serde_json::json;

/// Example MCP client demonstrating how to interact with Ethereum MCP server
fn main() {
    println!("Ethereum MCP Server Example Client");
    println!("===================================");

    // Example 1: Query ETH balance
    println!("\n1. Query ETH Balance Example:");
    let balance_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": "get_balance",
            "arguments": {
                "address": "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6"
            }
        }
    });
    println!(
        "Request: {}",
        serde_json::to_string_pretty(&balance_request).unwrap()
    );

    // Example 2: Query ERC20 token balance
    println!("\n2. Query USDC Token Balance Example:");
    let token_balance_request = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": "get_balance",
            "arguments": {
                "address": "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6",
                "token_address": "0xA0b86a33E6441c8C06DdD5d8c4c4E4c4E4c4E4c4E"
            }
        }
    });
    println!(
        "Request: {}",
        serde_json::to_string_pretty(&token_balance_request).unwrap()
    );

    // Example 3: Query token price
    println!("\n3. Query Token Price Example:");
    let price_request = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "get_token_price",
            "arguments": {
                "symbol": "USDC",
                "quote_currency": "USD"
            }
        }
    });
    println!(
        "Request: {}",
        serde_json::to_string_pretty(&price_request).unwrap()
    );

    // Example 4: Token swap simulation
    println!("\n4. Token Swap Simulation Example:");
    let swap_request = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "swap_tokens",
            "arguments": {
                "from_token": "0xA0b86a33E6441c8C06DdD5d8c4c4E4c4E4c4E4c4E",
                "to_token": "0xB0b86a33E6441c8C06DdD5d8c4c4E4c4E4c4E4c4E",
                "amount": "1000000",
                "slippage_tolerance": 0.5
            }
        }
    });
    println!(
        "Request: {}",
        serde_json::to_string_pretty(&swap_request).unwrap()
    );

    // Example 5: List available tools
    println!("\n5. List Available Tools Example:");
    let tools_request = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "tools/list"
    });
    println!(
        "Request: {}",
        serde_json::to_string_pretty(&tools_request).unwrap()
    );

    println!("\n===================================");
    println!("To run the actual MCP server, execute:");
    println!("export ETHEREUM_RPC_URL=\"https://eth.llamarpc.com\"");
    println!("export PRIVATE_KEY=\"0xYOUR_PRIVATE_KEY\"");
    println!("cargo run");
}
