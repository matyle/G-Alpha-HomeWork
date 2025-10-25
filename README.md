# Ethereum MCP Server

A comprehensive Ethereum MCP (Model Context Protocol) server that provides real-time blockchain data access, token price queries, and swap simulation capabilities.

## 🚀 Features

- **Real-time Balance Queries**: Query ETH and ERC20 token balances from any address
- **Live Price Data**: Get real-time token prices via Uniswap V2/V3 integration
- **Swap Simulation**: Simulate token swaps with gas estimation and price impact analysis
- **Multi-RPC Support**: Automatic failover between multiple free RPC providers
- **Wallet Management**: Secure private key handling for transaction signing
- **MCP Protocol**: Full Model Context Protocol implementation

## 📋 Prerequisites

- Rust 1.70+ 
- Git
- Internet connection for RPC access

## 🛠️ Installation

### 1. Clone the Repository

```bash
git clone https://github.com/yourusername/ethereum-mcp-server.git
cd ethereum-mcp-server
```

### 2. Build the Project

```bash
cargo build --release
```

### 3. Run Tests

```bash
cargo test
```

## 🚀 Quick Start

### Method 1: Simple Start (Recommended)

```bash
# Run with default settings (no private key required)
cargo run
```

This will:
- Automatically select the best available RPC provider
- Use a test private key (no real value)
- Enable all query and simulation features

### Method 2: Full Functionality

```bash
# Set your private key for actual transactions
export PRIVATE_KEY=0x你的私钥

# Optionally set a specific RPC URL
export ETHEREUM_RPC_URL=https://eth.llamarpc.com

# Run the server
cargo run
```

### Method 3: Generate Test Private Key

```bash
# Generate a new test private key
./generate_key.sh

# Use the generated key
export PRIVATE_KEY=$(cat .private_key)
cargo run
```

## 🌐 Supported RPC Providers

The server automatically tests and selects from these free RPC providers:

| Provider | URL | Rate Limit | Status |
|----------|-----|------------|--------|
| LlamaRPC | `https://eth.llamarpc.com` | 1000/min | ✅ Recommended |
| Ankr | `https://rpc.ankr.com/eth` | 500/min | ✅ Stable |
| PublicNode | `https://ethereum.publicnode.com` | Unlimited | ✅ Free |
| Cloudflare | `https://cloudflare-eth.com` | 1000/min | ✅ Enterprise |

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `PRIVATE_KEY` | Ethereum private key (with 0x prefix) | Test key | ❌ |
| `ETHEREUM_RPC_URL` | Ethereum RPC endpoint | Auto-select | ❌ |

### Private Key Usage

**Functions requiring private key:**
- ✅ Transaction signing
- ✅ Wallet address derivation
- ✅ Gas estimation

**Functions NOT requiring private key:**
- ✅ Balance queries
- ✅ Price queries
- ✅ Swap simulation

## 📊 Supported Tokens

- **ETH** - Ethereum native token
- **USDC** - USD Coin (0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48)
- **WETH** - Wrapped ETH (0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2)
- **USDT** - Tether USD (0xdAC17F958D2ee523a2206206994597C13D831ec7)
- **DAI** - Dai Stablecoin (0x6B175474E89094C44Da98b954EedeAC495271d0F)
- **WBTC** - Wrapped Bitcoin (0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599)

## 🔄 Supported Swap Protocols

- **Uniswap V2** - Classic AMM protocol
- **Uniswap V3** - Concentrated liquidity protocol

## 📖 API Examples

### Balance Query

```json
{
  "method": "get_balance",
  "params": {
    "address": "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6",
    "token_address": null
  }
}
```

### Price Query

```json
{
  "method": "get_token_price",
  "params": {
    "token_address": null,
    "symbol": "USDC",
    "quote_currency": "USD"
  }
}
```

### Swap Simulation

```json
{
  "method": "swap_tokens",
  "params": {
    "from_token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    "to_token": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
    "amount": "1000000",
    "slippage_tolerance": 0.5
  }
}
```

## 🧪 Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_get_eth_balance

# Run with output
cargo test -- --nocapture
```

## 🛡️ Security

### Test Environment
- Use the provided test private key (no real value)
- All transactions are simulated

### Production Environment
- Use hardware wallets (Ledger, Trezor)
- Store private keys securely offline
- Never commit real private keys to version control

## 🐛 Troubleshooting

### Common Issues

1. **"Please set PRIVATE_KEY environment variable"**
   ```bash
   export PRIVATE_KEY=0x你的私钥
   ```

2. **"RPC connection failed"**
   - Check internet connection
   - Try a different RPC provider
   - The server will auto-select available RPCs

3. **"Private key format error"**
   - Ensure key starts with `0x`
   - Ensure key is 66 characters long (including `0x`)

### Debug Mode

```bash
# Enable verbose logging
RUST_LOG=debug cargo run
```

## 📁 Project Structure

```
src/
├── main.rs              # Application entry point
├── ethereum/
│   ├── client.rs        # Ethereum client implementation
│   ├── rpc.rs          # RPC provider management
│   └── types.rs        # Data structures
├── mcp/
│   ├── server.rs       # MCP server implementation
│   └── types.rs        # MCP protocol types
└── tools/
    ├── balance.rs      # Balance query tools
    ├── price.rs        # Price query tools
    └── swap.rs         # Swap simulation tools
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Ethers.rs](https://github.com/gakonst/ethers-rs) - Ethereum library for Rust
- [Uniswap](https://uniswap.org/) - Decentralized exchange protocol
- [MCP](https://modelcontextprotocol.io/) - Model Context Protocol

