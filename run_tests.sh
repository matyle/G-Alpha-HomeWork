#!/bin/bash

# 运行 Ethereum MCP 测试

echo "🧪 运行 Ethereum MCP 测试"
echo "================================"
echo ""

# 设置测试环境变量
export ETHEREUM_RPC_URL="https://eth.llamarpc.com"
export PRIVATE_KEY="0x1234567890123456789012345678901234567890123456789012345678901234"

echo "📡 RPC: $ETHEREUM_RPC_URL"
echo "🔑 私钥: ${PRIVATE_KEY:0:10}..."
echo ""

# 运行测试
echo "🚀 开始运行测试..."
cargo test -- --nocapture

echo ""
echo "✅ 测试完成！"
