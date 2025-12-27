//! 性能指标模块
//!
//! 定义性能监控和统计相关的数据结构

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

/// 性能指标
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    /// 总请求数
    pub total_requests: AtomicUsize,
    /// 成功请求数
    pub successful_requests: AtomicUsize,
    /// 失败请求数
    pub failed_requests: AtomicUsize,
    /// 总发送字节数
    pub total_bytes_sent: AtomicUsize,
    /// 总接收字节数
    pub total_bytes_received: AtomicUsize,
    /// 平均响应时间（纳秒）
    pub total_response_time_ns: AtomicUsize,
}

impl PerformanceMetrics {
    /// 增加请求统计
    pub fn increment_request(
        &self, 
        success: bool, 
        bytes_sent: usize, 
        bytes_received: usize,
        response_time_ns: u64
    ) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        if success {
            self.successful_requests.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_requests.fetch_add(1, Ordering::Relaxed);
        }
        self.total_bytes_sent.fetch_add(bytes_sent, Ordering::Relaxed);
        self.total_bytes_received.fetch_add(bytes_received, Ordering::Relaxed);
        self.total_response_time_ns.fetch_add(response_time_ns as usize, Ordering::Relaxed);
    }

    /// 计算QPS（每秒查询数）
    pub fn qps(&self, duration: Duration) -> f64 {
        let total = self.total_requests.load(Ordering::Relaxed) as f64;
        if duration.as_secs_f64() > 0.0 {
            total / duration.as_secs_f64()
        } else {
            0.0
        }
    }

    /// 计算成功率
    pub fn success_rate(&self) -> f64 {
        let total = self.total_requests.load(Ordering::Relaxed) as f64;
        let success = self.successful_requests.load(Ordering::Relaxed) as f64;
        
        if total > 0.0 {
            success / total * 100.0
        } else {
            0.0
        }
    }

    /// 计算平均响应时间
    pub fn average_response_time(&self) -> Duration {
        let total_requests = self.total_requests.load(Ordering::Relaxed) as u64;
        let total_time_ns = self.total_response_time_ns.load(Ordering::Relaxed) as u64;
        
        if total_requests > 0 {
            Duration::from_nanos(total_time_ns / total_requests)
        } else {
            Duration::ZERO
        }
    }

    /// 计算吞吐量（字节/秒）
    pub fn throughput(&self, duration: Duration) -> f64 {
        let total_bytes = self.total_bytes_received.load(Ordering::Relaxed) as f64;
        if duration.as_secs_f64() > 0.0 {
            total_bytes / duration.as_secs_f64()
        } else {
            0.0
        }
    }

    /// 重置所有指标
    pub fn reset(&self) {
        self.total_requests.store(0, Ordering::Relaxed);
        self.successful_requests.store(0, Ordering::Relaxed);
        self.failed_requests.store(0, Ordering::Relaxed);
        self.total_bytes_sent.store(0, Ordering::Relaxed);
        self.total_bytes_received.store(0, Ordering::Relaxed);
        self.total_response_time_ns.store(0, Ordering::Relaxed);
    }

    /// 获取当前统计的快照
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            total_requests: self.total_requests.load(Ordering::Relaxed),
            successful_requests: self.successful_requests.load(Ordering::Relaxed),
            failed_requests: self.failed_requests.load(Ordering::Relaxed),
            total_bytes_sent: self.total_bytes_sent.load(Ordering::Relaxed),
            total_bytes_received: self.total_bytes_received.load(Ordering::Relaxed),
            total_response_time_ns: self.total_response_time_ns.load(Ordering::Relaxed),
        }
    }
}

/// 指标快照（用于线程安全读取）
#[derive(Debug, Clone, Default)]
pub struct MetricsSnapshot {
    pub total_requests: usize,
    pub successful_requests: usize,
    pub failed_requests: usize,
    pub total_bytes_sent: usize,
    pub total_bytes_received: usize,
    pub total_response_time_ns: usize,
}

impl MetricsSnapshot {
    /// 计算成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_requests > 0 {
            self.successful_requests as f64 / self.total_requests as f64 * 100.0
        } else {
            0.0
        }
    }

    /// 计算平均响应时间
    pub fn average_response_time(&self) -> Duration {
        if self.total_requests > 0 {
            Duration::from_nanos(self.total_response_time_ns as u64 / self.total_requests as u64)
        } else {
            Duration::ZERO
        }
    }
}