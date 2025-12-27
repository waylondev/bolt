use crate::application::{
    ApiDebugUseCase, ApiDebugUseCaseImpl, LoadTestUseCase, LoadTestUseCaseImpl,
};
use crate::domain::entities::{LoadTestConfig, LoadTestResult, LoadTestState};
use crate::infrastructure::http::ReqwestClient;

pub struct CliHandler {
    api_debug_use_case: ApiDebugUseCaseImpl<ReqwestClient>,
    load_test_use_case: LoadTestUseCaseImpl<ReqwestClient>,
}

impl CliHandler {
    pub fn new() -> Result<Self, anyhow::Error> {
        let client = ReqwestClient::new()?;
        Ok(Self {
            api_debug_use_case: ApiDebugUseCaseImpl::new(client.clone()),
            load_test_use_case: LoadTestUseCaseImpl::new(client),
        })
    }

    pub async fn handle_debug(&self, url: &str, method: &str) -> Result<(), anyhow::Error> {
        println!("\n🔍 API 调试: {} {}\n", method, url);

        match self.api_debug_use_case.execute(url, method).await {
            Ok(result) => {
                println!("✅ 状态: {}", result.response.status);
                println!("⏱️  耗时: {:.2?}", result.response.duration);
                println!("\n📋 响应头:");
                for (key, value) in result.response.headers.iter() {
                    println!("  {}: {}", key, value);
                }

                if let Some(body) = &result.formatted_body {
                    println!("\n📄 响应体:\n{}", body);
                }

                Ok(())
            }
            Err(e) => {
                println!("❌ 错误: {}", e);
                Err(anyhow::anyhow!(e.to_string()))
            }
        }
    }

    pub async fn handle_load_test(
        &self,
        url: &str,
        method: &str,
        concurrent: usize,
        duration: u64,
    ) -> Result<(), anyhow::Error> {
        println!(
            "\n🚀 负载测试: {} {} (并发: {}, 时长: {}秒)\n",
            method, url, concurrent, duration
        );

        let config = LoadTestConfig {
            url: url.to_string(),
            method: method.to_string(),
            concurrent_users: concurrent,
            duration_secs: duration,
            warmup_secs: 0,
            ramp_up_secs: 0,
        };

        let start_time = std::time::Instant::now();
        let start_time_for_move = start_time;

        match self
            .load_test_use_case
            .execute(config, move |state, result| {
                print_progress(state, result, start_time_for_move);
            })
            .await
        {
            Ok(result) => {
                println!("\n📊 测试完成!\n");
                print_summary(&result);
                Ok(())
            }
            Err(e) => {
                println!("❌ 错误: {}", e);
                Err(anyhow::anyhow!(e.to_string()))
            }
        }
    }
}

impl Default for CliHandler {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

fn print_progress(state: LoadTestState, result: LoadTestResult, start_time: std::time::Instant) {
    let _ = start_time;
    let success_rate = if result.total_requests > 0 {
        result.successful_requests as f64 / result.total_requests as f64 * 100.0
    } else {
        0.0
    };

    print!(
        "\r🔄 状态: {:?} | 请求: {} | QPS: {:.1} | 成功率: {:.1}% | 延迟: {:.2?}",
        state, result.total_requests, result.qps, success_rate, result.avg_latency,
    );
    use std::io::Write;
    let _ = std::io::stdout().flush();
}

fn print_summary(result: &LoadTestResult) {
    println!("┌─────────────────────────────────────────────┐");
    println!("│              负载测试结果摘要               │");
    println!("├─────────────────────────────────────────────┤");
    println!("│ 总请求数:        {:>25} │", result.total_requests);
    println!("│ 成功请求:        {:>25} │", result.successful_requests);
    println!("│ 失败请求:        {:>25} │", result.failed_requests);
    println!("├─────────────────────────────────────────────┤");
    println!("│ 2xx 响应:        {:>25} │", result.status_2xx);
    println!("│ 4xx 响应:        {:>25} │", result.status_4xx);
    println!("│ 5xx 响应:        {:>25} │", result.status_5xx);
    println!("├─────────────────────────────────────────────┤");
    println!("│ 平均延迟:        {:>25?} │", result.avg_latency);
    println!("│ 最小延迟:        {:>25?} │", result.min_latency);
    println!("│ 最大延迟:        {:>25?} │", result.max_latency);
    println!("│ P95 延迟:        {:>25?} │", result.p95_latency);
    println!("│ P99 延迟:        {:>25?} │", result.p99_latency);
    println!("├─────────────────────────────────────────────┤");
    println!("│ QPS:             {:>25.1} │", result.qps);
    println!("│ 总耗时:          {:>25?} │", result.total_duration);
    println!("└─────────────────────────────────────────────┘");
}
