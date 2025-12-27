use crate::domain::entities::{HttpRequest, HttpResponse};
use crate::domain::services::{HttpClient, HttpClientError};
use crate::domain::value_objects::{Body, ContentType, Headers};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    pub fn new() -> Result<Self, HttpClientError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        Ok(Self { client })
    }
}

#[async_trait::async_trait]
impl HttpClient for ReqwestClient {
    async fn execute(&self, request: &HttpRequest) -> Result<HttpResponse, HttpClientError> {
        let start_time = Instant::now();

        let mut req_builder = match request.method {
            crate::domain::HttpMethod::GET => self.client.get(request.url.as_str()),
            crate::domain::HttpMethod::POST => self.client.post(request.url.as_str()),
            crate::domain::HttpMethod::PUT => self.client.put(request.url.as_str()),
            crate::domain::HttpMethod::DELETE => self.client.delete(request.url.as_str()),
            crate::domain::HttpMethod::PATCH => self.client.patch(request.url.as_str()),
            crate::domain::HttpMethod::HEAD => self.client.head(request.url.as_str()),
            crate::domain::HttpMethod::OPTIONS => self
                .client
                .request(http::Method::OPTIONS, request.url.as_str()),
        };

        for (key, value) in request.headers.iter() {
            req_builder = req_builder.header(key, value);
        }

        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.to_string());
        }

        let request = req_builder
            .build()
            .map_err(|e| HttpClientError::SerializationError(e.to_string()))?;
        let response = self.client.execute(request).await?;
        let duration = start_time.elapsed();
        let status = response.status().as_u16();

        let mut headers_map: HashMap<String, String> = HashMap::new();
        for (k, v) in response.headers().iter() {
            headers_map.insert(k.to_string(), v.to_str().unwrap_or("").to_string());
        }

        let body = response
            .text()
            .await
            .ok()
            .map(|s| Body::from_string(s, ContentType::Text));

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

    async fn close(&self) -> Result<(), HttpClientError> {
        Ok(())
    }
}

impl Default for ReqwestClient {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
