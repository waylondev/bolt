use crate::domain::entities::{HttpRequest, LoadTestConfig, LoadTestResult, LoadTestState};
use crate::domain::services::{HttpClient, HttpClientError};
use crate::domain::value_objects::{Headers, Url};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;
use tokio::time::Duration;

#[async_trait]
pub trait LoadTestUseCase: Send + Sync {
    async fn execute(
        &self,
        config: LoadTestConfig,
        progress_callback: impl Fn(LoadTestState, LoadTestResult) + Send + 'static,
    ) -> Result<LoadTestResult, HttpClientError>;
}

pub struct LoadTestUseCaseImpl<C: HttpClient> {
    client: C,
}

impl<C: HttpClient> LoadTestUseCaseImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: HttpClient> LoadTestUseCase for LoadTestUseCaseImpl<C> {
    async fn execute(
        &self,
        config: LoadTestConfig,
        progress_callback: impl Fn(LoadTestState, LoadTestResult) + Send + 'static,
    ) -> Result<LoadTestResult, HttpClientError> {
        let mut result = LoadTestResult::default();

        let (sender, mut receiver) = mpsc::channel::<(u16, Duration)>(10000);

        let config = Arc::new(config);
        let request_url = config.url.clone();
        let request_method = config.method.clone();
        let duration_secs = config.duration_secs;

        let worker = tokio::spawn(async move {
            let request = HttpRequest {
                id: uuid::Uuid::new_v4(),
                url: Url::parse(&request_url).unwrap(),
                method: request_method.parse().unwrap(),
                headers: Headers::new(),
                body: None,
                timeout: Duration::from_secs(30),
            };

            let start_time = Instant::now();
            let mut handles = Vec::new();

            for _ in 0..config.concurrent_users {
                let sender = sender.clone();
                let request = request.clone();

                let handle = tokio::spawn(async move {
                    let mut interval = tokio::time::interval(Duration::from_millis(100));
                    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

                    loop {
                        interval.tick().await;
                        if Instant::now().duration_since(start_time)
                            > Duration::from_secs(duration_secs)
                        {
                            break;
                        }

                        let start = Instant::now();
                        let response = crate::infrastructure::http::ReqwestClient::new()
                            .unwrap()
                            .execute(&request)
                            .await;

                        match response {
                            Ok(resp) => {
                                if sender.send((resp.status, start.elapsed())).await.is_err() {
                                    break;
                                }
                            }
                            Err(_) => {
                                if sender.send((0, start.elapsed())).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                let _ = handle.await;
            }
        });

        while let Some((status, duration)) = receiver.recv().await {
            result.add_response(status, duration);
            result.calculate_qps();
            progress_callback(LoadTestState::Running, result.clone());
        }

        let _ = worker.await;

        result.calculate_qps();
        progress_callback(LoadTestState::Completed, result.clone());

        Ok(result)
    }
}
