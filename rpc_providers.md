# 免费以太坊 RPC 提供商

## 🚀 推荐提供商

### 1. LlamaRPC (推荐)
```bash
export ETHEREUM_RPC_URL="https://eth.llamarpc.com"
```
- ✅ 稳定快速
- ✅ 免费
- ✅ 每分钟 1000 次请求
- ✅ 支持所有以太坊功能

### 2. Ankr
```bash
export ETHEREUM_RPC_URL="https://rpc.ankr.com/eth"
```
- ✅ 多链支持
- ✅ 免费
- ✅ 每分钟 500 次请求
- ✅ 支持所有以太坊功能

### 3. PublicNode
```bash
export ETHEREUM_RPC_URL="https://ethereum.publicnode.com"
```
- ✅ 完全免费
- ✅ 无请求限制
- ⚠️ 可能较慢

### 4. Cloudflare
```bash
export ETHEREUM_RPC_URL="https://cloudflare-eth.com"
```
- ✅ 由 Cloudflare 提供
- ✅ 稳定可靠
- ✅ 每分钟 1000 次请求

## 🔧 使用方法

### 方法 1: 环境变量
```bash
export ETHEREUM_RPC_URL="https://eth.llamarpc.com"
cargo run
```

### 方法 2: 直接运行
```bash
ETHEREUM_RPC_URL="https://eth.llamarpc.com" cargo run
```

### 方法 3: 配置文件
编辑 `config.toml` 文件：
```toml
[ethereum]
rpc_url = "https://eth.llamarpc.com"
```

## 📊 价格获取功能

项目支持通过以下方式获取链上价格：

1. **ETH 价格**: 通过 Uniswap V2/V3 获取 ETH/USDC 价格
2. **ERC20 代币价格**: 通过 Uniswap 获取代币/ETH 价格
3. **多协议支持**: 自动选择 Uniswap V2 或 V3 的最佳价格

## 🧪 测试价格获取

```bash
# 使用 LlamaRPC
ETHEREUM_RPC_URL="https://eth.llamarpc.com" PRIVATE_KEY="0x你的私钥" cargo run

# 使用 Ankr
ETHEREUM_RPC_URL="https://rpc.ankr.com/eth" PRIVATE_KEY="0x你的私钥" cargo run

# 使用 PublicNode
ETHEREUM_RPC_URL="https://ethereum.publicnode.com" PRIVATE_KEY="0x你的私钥" cargo run
```

## ⚡ 性能对比

| 提供商 | 速度 | 稳定性 | 限制 | 推荐度 |
|--------|------|--------|------|--------|
| LlamaRPC | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 1000/分钟 | ⭐⭐⭐⭐⭐ |
| Ankr | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | 500/分钟 | ⭐⭐⭐⭐ |
| PublicNode | ⭐⭐⭐ | ⭐⭐⭐ | 无限制 | ⭐⭐⭐ |
| Cloudflare | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 1000/分钟 | ⭐⭐⭐⭐ |

## 🔍 故障排除

如果某个 RPC 不可用，可以尝试其他提供商：

```bash
# 备用 RPC 列表
export ETHEREUM_RPC_URL="https://eth.llamarpc.com"  # 主要
export ETHEREUM_RPC_URL="https://rpc.ankr.com/eth"  # 备用1
export ETHEREUM_RPC_URL="https://ethereum.publicnode.com"  # 备用2
export ETHEREUM_RPC_URL="https://cloudflare-eth.com"  # 备用3
```
