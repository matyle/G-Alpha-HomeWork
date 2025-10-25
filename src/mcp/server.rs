use crate::ethereum::client::EthereumClient;
use crate::mcp::types::{Content, MCPRequest, MCPResponse, Tool, ToolCall, ToolResult};
use crate::tools::{get_balance, get_token_price, swap_tokens};
use anyhow::Result;
use serde_json::json;
use tracing::{error, info};

#[allow(dead_code)]
pub struct MCPServer {
    ethereum_client: EthereumClient,
    tools: Vec<Tool>,
}

#[allow(dead_code)]
impl MCPServer {
    pub async fn new(rpc_url: String, private_key: String) -> Result<Self> {
        let ethereum_client = EthereumClient::new(rpc_url, private_key).await?;

        let tools = vec![
            Tool {
                name: "get_balance".to_string(),
                description: "Query ETH and ERC20 token balances for Ethereum addresses"
                    .to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "address": {
                            "type": "string",
                            "description": "Ethereum address"
                        },
                        "token_address": {
                            "type": "string",
                            "description": "ERC20 token contract address (optional, queries ETH balance if not provided)"
                        }
                    },
                    "required": ["address"]
                }),
            },
            Tool {
                name: "get_token_price".to_string(),
                description: "Get current token price in USD or ETH".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "token_address": {
                            "type": "string",
                            "description": "Token contract address"
                        },
                        "symbol": {
                            "type": "string",
                            "description": "Token symbol (e.g., USDC, WETH)"
                        },
                        "quote_currency": {
                            "type": "string",
                            "description": "Quote currency (USD or ETH)",
                            "default": "USD"
                        }
                    }
                }),
            },
            Tool {
                name: "swap_tokens".to_string(),
                description: "Simulate token swap (does not execute actual transaction)"
                    .to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "from_token": {
                            "type": "string",
                            "description": "Source token address"
                        },
                        "to_token": {
                            "type": "string",
                            "description": "Destination token address"
                        },
                        "amount": {
                            "type": "string",
                            "description": "Swap amount (in source token's smallest unit)"
                        },
                        "slippage_tolerance": {
                            "type": "number",
                            "description": "Slippage tolerance (percentage, e.g., 0.5 means 0.5%)",
                            "default": 0.5
                        }
                    },
                    "required": ["from_token", "to_token", "amount"]
                }),
            },
        ];

        Ok(Self {
            ethereum_client,
            tools,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        info!("MCP 服务器已启动，等待请求...");

        // 这里应该实现实际的 MCP 协议处理
        // 为了演示，我们使用简单的 JSON-RPC 处理
        // 在实际实现中，这里应该从 stdin 或网络连接读取 MCP 请求
        // 现在我们先实现工具调用逻辑

        Ok(())
    }

    pub async fn handle_request(&self, request: MCPRequest) -> MCPResponse {
        match request.method.as_str() {
            "tools/list" => {
                let result = json!({
                    "tools": self.tools
                });
                MCPResponse::success(request.id, result)
            }
            "tools/call" => {
                if let Some(params) = request.params {
                    if let Ok(tool_call) = serde_json::from_value::<ToolCall>(params) {
                        match self.handle_tool_call(tool_call).await {
                            Ok(result) => {
                                let result_json = json!({
                                    "content": result.content,
                                    "isError": result.is_error
                                });
                                MCPResponse::success(request.id, result_json)
                            }
                            Err(e) => {
                                error!("工具调用错误: {}", e);
                                MCPResponse::error(
                                    request.id,
                                    -32603,
                                    format!("工具调用失败: {}", e),
                                )
                            }
                        }
                    } else {
                        MCPResponse::error(request.id, -32602, "无效的参数格式".to_string())
                    }
                } else {
                    MCPResponse::error(request.id, -32602, "缺少参数".to_string())
                }
            }
            _ => MCPResponse::error(request.id, -32601, "未知方法".to_string()),
        }
    }

    async fn handle_tool_call(&self, tool_call: ToolCall) -> Result<ToolResult> {
        match tool_call.name.as_str() {
            "get_balance" => {
                let address = tool_call
                    .arguments
                    .get("address")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("缺少 address 参数"))?;

                let token_address = tool_call
                    .arguments
                    .get("token_address")
                    .and_then(|v| v.as_str());

                let balance = get_balance(&self.ethereum_client, address, token_address).await?;

                Ok(ToolResult {
                    content: vec![Content {
                        content_type: "text".to_string(),
                        text: format!("余额查询结果: {}", balance),
                    }],
                    is_error: false,
                })
            }
            "get_token_price" => {
                let token_address = tool_call
                    .arguments
                    .get("token_address")
                    .and_then(|v| v.as_str());
                let symbol = tool_call.arguments.get("symbol").and_then(|v| v.as_str());
                let quote_currency = tool_call
                    .arguments
                    .get("quote_currency")
                    .and_then(|v| v.as_str())
                    .unwrap_or("USD");

                let price =
                    get_token_price(&self.ethereum_client, token_address, symbol, quote_currency)
                        .await?;

                Ok(ToolResult {
                    content: vec![Content {
                        content_type: "text".to_string(),
                        text: format!("代币价格: {}", price),
                    }],
                    is_error: false,
                })
            }
            "swap_tokens" => {
                let from_token = tool_call
                    .arguments
                    .get("from_token")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("缺少 from_token 参数"))?;
                let to_token = tool_call
                    .arguments
                    .get("to_token")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("缺少 to_token 参数"))?;
                let amount = tool_call
                    .arguments
                    .get("amount")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("缺少 amount 参数"))?;
                let slippage_tolerance = tool_call
                    .arguments
                    .get("slippage_tolerance")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.5);

                let swap_result = swap_tokens(
                    &self.ethereum_client,
                    from_token,
                    to_token,
                    amount,
                    slippage_tolerance,
                )
                .await?;

                Ok(ToolResult {
                    content: vec![Content {
                        content_type: "text".to_string(),
                        text: format!("交换模拟结果: {}", swap_result),
                    }],
                    is_error: false,
                })
            }
            _ => Err(anyhow::anyhow!("未知工具: {}", tool_call.name)),
        }
    }
}
