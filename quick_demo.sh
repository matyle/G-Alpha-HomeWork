#!/bin/bash

echo "🎯 Ethereum MCP 快速演示"
echo "================================"
echo ""

# 设置环境变量
export ETHEREUM_RPC_URL="https://eth.llamarpc.com"
export PRIVATE_KEY="0xee47965684a23f4c2c4447ad7ff164cc0f7539cfcd313700fb353d25ea479e1a"

echo "📡 RPC: $ETHEREUM_RPC_URL"
echo "🔑 私钥: ${PRIVATE_KEY:0:10}..."
echo ""

echo "🚀 启动服务器（按 Ctrl+C 停止）..."
echo ""

# 运行服务器
cargo run
