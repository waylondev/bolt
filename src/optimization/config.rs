//! 性能配置模块
//!
//! 定义性能优化相关的配置参数

/// 性能配置
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// 最大连接池大小
    pub max_pool_size: usize,
    /// 每个主机的最大连接数
    pub max_idle_per_host: usize,
    /// TCP nodelay (禁用 Nagle 算法)
    pub tcp_nodelay: bool,
    /// HTTP/2 最大并发流
    pub http2_max_concurrent_streams: u32,
    /// 批量统计的缓冲区大小
    pub stats_buffer_size: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_pool_size: 10000,
            max_idle_per_host: 1000,
            tcp_nodelay: true,
            http2_max_concurrent_streams: 100,
            stats_buffer_size: 1000,
        }
    }
}

impl PerformanceConfig {
    /// 创建高性能配置
    pub fn high_performance() -> Self {
        Self {
            max_pool_size: 50000,
            max_idle_per_host: 5000,
            tcp_nodelay: true,
            http2_max_concurrent_streams: 1000,
            stats_buffer_size: 5000,
        }
    }
    
    /// 创建平衡配置
    pub fn balanced() -> Self {
        Self::default()
    }
    
    /// 创建资源节约配置
    pub fn resource_saving() -> Self {
        Self {
            max_pool_size: 1000,
            max_idle_per_host: 100,
            tcp_nodelay: true,
            http2_max_concurrent_streams: 10,
            stats_buffer_size: 100,
        }
    }
}