use crate::domain::entities::{HttpRequest, HttpResponse};
use crate::domain::services::{HttpClient, HttpClientError};
use crate::domain::value_objects::{Body as DomainBody, ContentType, Headers};
use std::time::Instant;

#[derive(Clone)]
pub struct HyperClient {
    client: reqwest::Client,
    timeout: std::time::Duration,
}

impl HyperClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .pool_max_idle_per_host(1000)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create reqwest client");

        Self {
            client,
            timeout: std::time::Duration::from_secs(30),
        }
    }

    async fn execute_inner(&self, request: &HttpRequest) -> Result<HttpResponse, HttpClientError> {
        let start_time = Instant::now();

        // 构建请求
        let mut req_builder = self.client
            .request(request.method.as_reqwest_method(), request.url.as_str());

        // 添加headers
        for (key, value) in request.headers.iter() {
            req_builder = req_builder.header(key, value);
        }

        // 添加body
        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.to_string());
        }

        // 发送请求
        let response = req_builder
            .send()
            .await
            .map_err(|e| HttpClientError::ConnectionFailed(e.to_string()))?;

        let duration = start_time.elapsed();
        let status = response.status().as_u16();

        // 处理headers
        let mut headers_map = std::collections::HashMap::new();
        for (key, value) in response.headers() {
            headers_map.insert(key.to_string(), value.to_str().unwrap_or("").to_string());
        }

        // 处理body
        let body_bytes = response
            .bytes()
            .await
            .map_err(|e| HttpClientError::IoError(e.to_string()))?;

        let body_str = String::from_utf8_lossy(&body_bytes).to_string();
        let body = if body_str.is_empty() {
            None
        } else {
            Some(DomainBody::from_string(body_str, ContentType::Text))
        };

        let timestamp = chrono::DateTime::from_timestamp(chrono::Utc::now().timestamp(), 0)
            .unwrap_or_else(|| chrono::DateTime::from_timestamp(0, 0).unwrap());

        Ok(HttpResponse {
            status,
            headers: Headers::from(headers_map),
            body,
            duration,
            timestamp,
        })
    }
}

#[async_trait::async_trait]
impl HttpClient for HyperClient {
    async fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, HttpClientError> {
        self.execute_inner(request).await
    }

    async fn close(&self) -> Result<(), HttpClientError> {
        Ok(())
    }
}

impl Default for HyperClient {
    fn default() -> Self {
        Self::new()
    }
}
