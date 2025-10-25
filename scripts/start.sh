#!/bin/bash

# Ethereum MCP Server startup script

echo "Starting Ethereum MCP Server..."

# Check environment variables
if [ -z "$ETHEREUM_RPC_URL" ]; then
    echo "Error: Please set ETHEREUM_RPC_URL environment variable"
    echo "Example: export ETHEREUM_RPC_URL=\"https://eth.llamarpc.com\""
    exit 1
fi

if [ -z "$PRIVATE_KEY" ]; then
    echo "Error: Please set PRIVATE_KEY environment variable"
    echo "Example: export PRIVATE_KEY=\"0xYOUR_PRIVATE_KEY\""
    exit 1
fi

# Build project
echo "Building project..."
cargo build --release

# Run server
echo "Starting server..."
cargo run --release
