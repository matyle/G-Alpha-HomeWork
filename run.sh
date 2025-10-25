#!/bin/bash

# Ethereum MCP Server 启动脚本

echo "🚀 启动 Ethereum MCP Server"
echo "================================"

export PRIVATE_KEY="0xee47965684a23f4c2c4447ad7ff164cc0f7539cfcd313700fb353d25ea479e1a"

# 检查是否设置了私钥
if [ -z "$PRIVATE_KEY" ]; then
    echo "❌ 错误：未设置 PRIVATE_KEY 环境变量"
    echo ""
    echo "请按以下方式之一设置私钥："
    echo ""
    echo "方法 1 - 设置环境变量："
    echo "  export PRIVATE_KEY=0x你的私钥"
    echo "  cargo run"
    echo ""
    echo "方法 2 - 直接运行："
    echo "  PRIVATE_KEY=0x你的私钥 cargo run"
    echo ""
    echo "方法 3 - 使用配置文件："
    echo "  编辑 config.toml 文件，然后运行："
    echo "  cargo run"
    echo ""
    echo "⚠️  注意：请确保私钥以 0x 开头且长度为 66 个字符"
    echo "⚠️  私钥用于签名交易，请确保安全存储"
    exit 1
fi

# 验证私钥格式
if [[ ! $PRIVATE_KEY =~ ^0x[0-9a-fA-F]{64}$ ]]; then
    echo "❌ 错误：私钥格式不正确"
    echo "请确保私钥以 0x 开头且长度为 66 个字符（包括 0x 前缀）"
    exit 1
fi

# 设置默认 RPC URL（如果未设置）
if [ -z "$ETHEREUM_RPC_URL" ]; then
    export ETHEREUM_RPC_URL="https://eth.llamarpc.com"
    echo "📡 使用默认 RPC: $ETHEREUM_RPC_URL"
else
    echo "📡 使用 RPC: $ETHEREUM_RPC_URL"
fi

echo "🔑 私钥已设置（前8位: ${PRIVATE_KEY:0:10}...）"
echo ""

# 编译并运行
echo "🔨 编译项目..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ 编译成功"
    echo "🚀 启动服务器..."
    echo ""
    cargo run --release
else
    echo "❌ 编译失败"
    exit 1
fi
