use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadTestResult {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub status_2xx: u64,
    pub status_4xx: u64,
    pub status_5xx: u64,
    pub total_duration: Duration,
    pub min_latency: Duration,
    pub max_latency: Duration,
    pub avg_latency: Duration,
    pub p50_latency: Duration,
    pub p90_latency: Duration,
    pub p95_latency: Duration,
    pub p99_latency: Duration,
    pub qps: f64,
}

impl Default for LoadTestResult {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            status_2xx: 0,
            status_4xx: 0,
            status_5xx: 0,
            total_duration: Duration::ZERO,
            min_latency: Duration::MAX,
            max_latency: Duration::ZERO,
            avg_latency: Duration::ZERO,
            p50_latency: Duration::ZERO,
            p90_latency: Duration::ZERO,
            p95_latency: Duration::ZERO,
            p99_latency: Duration::ZERO,
            qps: 0.0,
        }
    }
}

impl LoadTestResult {
    pub fn add_response(&mut self, status: u16, duration: Duration) {
        self.total_requests += 1;
        self.total_duration += duration;

        if status < 200 {
            self.failed_requests += 1;
        } else if status < 300 {
            self.successful_requests += 1;
            self.status_2xx += 1;
        } else if status < 400 {
            self.status_2xx += 1;
        } else if status < 500 {
            self.status_4xx += 1;
        } else {
            self.status_5xx += 1;
        }

        if duration < self.min_latency {
            self.min_latency = duration;
        }
        if duration > self.max_latency {
            self.max_latency = duration;
        }
    }

    pub fn calculate_qps(&mut self) {
        if self.total_duration.as_secs_f64() > 0.0 {
            self.qps = self.total_requests as f64 / self.total_duration.as_secs_f64();
        }
        if self.total_requests > 0 {
            self.avg_latency = self.total_duration / self.total_requests as u32;
        }
    }
}

#[derive(Debug, Clone)]
pub struct LoadTestConfig {
    pub url: String,
    pub method: String,
    pub concurrent_users: usize,
    pub duration_secs: u64,
    pub warmup_secs: u64,
    pub ramp_up_secs: u64,
}

impl Default for LoadTestConfig {
    fn default() -> Self {
        Self {
            url: "https://httpbin.org/get".to_string(),
            method: "GET".to_string(),
            concurrent_users: 10,
            duration_secs: 30,
            warmup_secs: 5,
            ramp_up_secs: 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadTestState {
    NotStarted,
    WarmingUp,
    RampingUp,
    Running,
    Completed,
    Failed,
}
