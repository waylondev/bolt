//! 工具函数模块

use std::time::Duration;

/// 格式化持续时间
pub fn format_duration(duration: &Duration) -> String {
    if duration.as_secs() > 0 {
        format!("{:.2}s", duration.as_secs_f64())
    } else if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{}μs", duration.as_micros())
    }
}

/// 计算 QPS (每秒查询数)
pub fn calculate_qps(total_requests: u64, duration: &Duration) -> f64 {
    if duration.as_secs() > 0 {
        total_requests as f64 / duration.as_secs_f64()
    } else {
        0.0
    }
}

/// 验证 URL 格式
pub fn validate_url(url: &str) -> Result<(), String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(())
    } else {
        Err("URL 必须以 http:// 或 https:// 开头".to_string())
    }
}