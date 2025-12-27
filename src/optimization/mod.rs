//! 性能优化模块
//!
//! 针对高并发场景的性能优化策略
//!
//! # 性能优化策略
//!
//! 1. **连接池优化**: 复用 TCP 连接，减少连接建立开销
//! 2. **HTTP/2 多路复用**: 单连接并行请求，减少 TCP 握手
//! 3. **零拷贝传输**: 减少内存拷贝开销
//! 4. **批量统计**: 减少锁竞争
//! 5. **自适应并发**: 根据系统负载动态调整

pub mod config;
pub mod metrics;
pub mod client_factory;

pub use config::PerformanceConfig;
pub use metrics::{PerformanceMetrics, MetricsSnapshot};
pub use client_factory::{ReqwestClientFactory, ConnectionPoolStats, ConnectionPoolManager};