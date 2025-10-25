use anyhow::{Context, Result};
use std::time::Duration;
use tokio::time::timeout;
use tracing::{info, warn};

/// 免费公共 RPC 提供商列表
pub const FREE_RPC_PROVIDERS: &[&str] = &[
    "https://eth.llamarpc.com",           // LlamaRPC - 推荐
    "https://rpc.ankr.com/eth",           // Ankr
    "https://ethereum.publicnode.com",    // PublicNode
    "https://cloudflare-eth.com",         // Cloudflare
    "https://rpc.flashbots.net",         // Flashbots
    "https://ethereum-rpc.publicnode.com", // PublicNode 备用
];

/// RPC 提供商信息
#[derive(Debug, Clone)]
pub struct RpcProvider {
    pub url: String,
    pub name: String,
    pub description: String,
    pub rate_limit: Option<u32>, // 每分钟请求限制
}

impl RpcProvider {
    pub fn new(url: &str, name: &str, description: &str, rate_limit: Option<u32>) -> Self {
        Self {
            url: url.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            rate_limit,
        }
    }
}

/// 获取推荐的 RPC 提供商列表
pub fn get_recommended_providers() -> Vec<RpcProvider> {
    vec![
        RpcProvider::new(
            "https://eth.llamarpc.com",
            "LlamaRPC",
            "稳定快速，推荐使用",
            Some(1000),
        ),
        RpcProvider::new(
            "https://rpc.ankr.com/eth",
            "Ankr",
            "多链支持，稳定可靠",
            Some(500),
        ),
        RpcProvider::new(
            "https://ethereum.publicnode.com",
            "PublicNode",
            "完全免费，无限制",
            None,
        ),
        RpcProvider::new(
            "https://cloudflare-eth.com",
            "Cloudflare",
            "由 Cloudflare 提供，稳定",
            Some(1000),
        ),
    ]
}

/// 测试 RPC 连接是否可用
pub async fn test_rpc_connection(rpc_url: &str) -> Result<bool> {
    use ethers::providers::{Http, Provider};
    use ethers::middleware::Middleware;

    let provider = match Provider::<Http>::try_from(rpc_url) {
        Ok(p) => p,
        Err(_) => return Ok(false),
    };

    // 设置 5 秒超时
    match timeout(Duration::from_secs(5), provider.get_chainid()).await {
        Ok(Ok(_)) => {
            info!("✅ RPC 连接成功: {}", rpc_url);
            Ok(true)
        }
        Ok(Err(_)) => {
            warn!("❌ RPC 连接失败: {}", rpc_url);
            Ok(false)
        }
        Err(_) => {
            warn!("⏰ RPC 连接超时: {}", rpc_url);
            Ok(false)
        }
    }
}

/// 自动选择可用的 RPC 提供商
pub async fn auto_select_rpc() -> Result<String> {
    info!("🔍 自动选择可用的 RPC 提供商...");

    for provider in get_recommended_providers() {
        info!("测试 RPC: {} ({})", provider.name, provider.url);
        
        if test_rpc_connection(&provider.url).await? {
            info!("✅ 选择 RPC: {} - {}", provider.name, provider.description);
            return Ok(provider.url);
        }
    }

    Err(anyhow::anyhow!("所有 RPC 提供商都不可用"))
}

/// 获取最佳 RPC URL
pub async fn get_best_rpc_url() -> Result<String> {
    // 首先检查环境变量
    if let Ok(env_rpc) = std::env::var("ETHEREUM_RPC_URL") {
        info!("使用环境变量中的 RPC: {}", env_rpc);
        
        // 测试环境变量中的 RPC 是否可用
        if test_rpc_connection(&env_rpc).await? {
            return Ok(env_rpc);
        } else {
            warn!("环境变量中的 RPC 不可用，尝试自动选择...");
        }
    }

    // 自动选择可用的 RPC
    auto_select_rpc().await
}

/// 显示 RPC 提供商信息
pub fn print_rpc_info() {
    println!("🌐 可用的免费 RPC 提供商:");
    println!("================================");
    
    for provider in get_recommended_providers() {
        println!("📡 {} ({})", provider.name, provider.url);
        println!("   {}", provider.description);
        if let Some(limit) = provider.rate_limit {
            println!("   限制: {} 请求/分钟", limit);
        } else {
            println!("   限制: 无限制");
        }
        println!();
    }
    
    println!("💡 使用方法:");
    println!("   export ETHEREUM_RPC_URL=\"https://eth.llamarpc.com\"");
    println!("   cargo run");
    println!();
}
