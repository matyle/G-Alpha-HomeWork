# Ethereum MCP Server 使用指南

## 🚀 快速开始

### 方法 1: 简单启动（推荐）

```bash
# 使用简单启动脚本（不需要私钥）
./start_simple.sh
```

### 方法 2: 完整功能

```bash
# 设置私钥（用于实际交易）
export PRIVATE_KEY=0x你的私钥

# 设置 RPC URL（可选，自动选择）
export ETHEREUM_RPC_URL=https://eth.llamarpc.com

# 运行服务器
cargo run
```

### 方法 3: 生成测试私钥

```bash
# 生成新的测试私钥
./generate_key.sh

# 使用生成的私钥
export PRIVATE_KEY=$(cat .private_key)
cargo run
```

## 🧪 测试功能

### 运行功能测试

```bash
# 设置私钥后运行测试
PRIVATE_KEY=0x你的私钥 cargo run --bin test_functions
```

### 测试内容

1. **ETH 余额查询** - 查询指定地址的 ETH 余额
2. **代币价格查询** - 通过 Uniswap 获取实时代币价格
3. **代币交换模拟** - 模拟 USDC -> WETH 交换
4. **ERC20 余额查询** - 查询指定地址的 ERC20 代币余额

## 📋 功能说明

### 支持的代币

- **ETH** - 以太坊原生代币
- **USDC** - USD Coin (0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48)
- **WETH** - Wrapped ETH (0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2)
- **USDT** - Tether USD (0xdAC17F958D2ee523a2206206994597C13D831ec7)
- **DAI** - Dai Stablecoin (0x6B175474E89094C44Da98b954EedeAC495271d0F)
- **WBTC** - Wrapped Bitcoin (0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599)

### 支持的交换协议

- **Uniswap V2** - 经典 AMM 协议
- **Uniswap V3** - 集中流动性协议

## 🔧 配置选项

### 环境变量

| 变量名 | 描述 | 默认值 | 必需 |
|--------|------|--------|------|
| `PRIVATE_KEY` | 以太坊私钥（带 0x 前缀） | - | ✅ |
| `ETHEREUM_RPC_URL` | 以太坊 RPC 端点 | 自动选择 | ❌ |

### 🌐 免费 RPC 提供商

#### 推荐提供商

1. **LlamaRPC** (推荐)
   ```bash
   export ETHEREUM_RPC_URL="https://eth.llamarpc.com"
   ```
   - ✅ 稳定快速
   - ✅ 每分钟 1000 次请求
   - ✅ 支持所有以太坊功能

2. **Ankr**
   ```bash
   export ETHEREUM_RPC_URL="https://rpc.ankr.com/eth"
   ```
   - ✅ 多链支持
   - ✅ 每分钟 500 次请求
   - ✅ 稳定可靠

3. **PublicNode**
   ```bash
   export ETHEREUM_RPC_URL="https://ethereum.publicnode.com"
   ```
   - ✅ 完全免费
   - ✅ 无请求限制
   - ⚠️ 可能较慢

4. **Cloudflare**
   ```bash
   export ETHEREUM_RPC_URL="https://cloudflare-eth.com"
   ```
   - ✅ 企业级稳定性
   - ✅ 每分钟 1000 次请求
   - ✅ 由 Cloudflare 提供

#### 自动选择 RPC

如果不设置 `ETHEREUM_RPC_URL`，程序会自动测试并选择最佳可用的 RPC：

```bash
# 自动选择最佳 RPC
cargo run
```

#### 付费专业 RPC

- **Infura**: `https://mainnet.infura.io/v3/YOUR_PROJECT_ID`
- **Alchemy**: `https://eth-mainnet.alchemyapi.io/v2/YOUR_API_KEY`
- **QuickNode**: `https://your-endpoint.quiknode.pro/YOUR_API_KEY/`

## 🔑 私钥使用说明

### 为什么需要私钥？

私钥用于以下功能：
- ✅ **交易签名** - 对 swap 交易进行数字签名
- ✅ **钱包地址** - 从私钥推导出钱包地址
- ✅ **Gas 估算** - 计算交易费用

### 不需要私钥的功能

以下功能**不需要**私钥：
- ✅ **查询余额** - 只需要地址
- ✅ **获取价格** - 只需要 RPC 连接
- ✅ **模拟交换** - 只需要 RPC 连接

### 如何获取私钥？

#### 1. 使用测试私钥（推荐）
```bash
# 项目提供的测试私钥（无实际价值）
export PRIVATE_KEY="0x1234567890123456789012345678901234567890123456789012345678901234"
```

#### 2. 生成新的测试私钥
```bash
# 生成新的测试私钥
./generate_key.sh
```

#### 3. 从现有钱包获取
- **MetaMask**: 账户详情 → 导出私钥
- **硬件钱包**: 通过钱包软件导出
- **其他钱包**: 查看钱包的私钥导出功能

## ⚠️ 安全注意事项

1. **测试私钥**: 项目提供的私钥仅用于测试，无实际价值
2. **私钥安全**: 生产环境请使用硬件钱包
3. **版本控制**: 不要将真实私钥提交到代码仓库
4. **权限控制**: 私钥用于签名交易，请谨慎使用

## 🐛 故障排除

### 常见错误

1. **"Please set PRIVATE_KEY environment variable"**
   - 解决：设置 `PRIVATE_KEY` 环境变量

2. **"私钥格式错误"**
   - 解决：确保私钥以 `0x` 开头且长度为 66 个字符

3. **"创建 Provider 失败"**
   - 解决：检查 RPC URL 是否正确且可访问

4. **"查询 ERC20 元数据失败"**
   - 解决：检查代币地址是否正确

### 调试模式

```bash
# 启用详细日志
RUST_LOG=debug cargo run
```

## 📊 示例输出

### 余额查询
```json
{
  "address": "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6",
  "token_address": null,
  "symbol": "ETH",
  "balance": "1000.123456",
  "decimals": 18,
  "formatted_balance": "1000.123456 ETH"
}
```

### 价格查询
```json
{
  "token_address": null,
  "symbol": "USDC",
  "price": "1.000000",
  "quote_currency": "USD",
  "timestamp": 1703123456
}
```

### 交换模拟
```json
{
  "from_token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
  "to_token": "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2",
  "input_amount": "1.000000",
  "output_amount": "0.000400",
  "price_impact": "0.1",
  "gas_estimate": 150000,
  "gas_price": "20.000000",
  "total_cost": "0.003000",
  "slippage_tolerance": "0.5",
  "minimum_output": "0.000398",
  "protocol": "UniswapV2",
  "fee": null,
  "router_address": "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D",
  "path": [
    "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
  ],
  "transaction_data": "0x..."
}
```
