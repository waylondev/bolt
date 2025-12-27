use crate::domain::entities::{HttpRequest, HttpResponse};
use crate::domain::services::{HttpClient, HttpClientError};
use crate::domain::value_objects::{ContentType, Headers, Url};
use async_trait::async_trait;

#[async_trait]
pub trait ApiDebugUseCase: Send + Sync {
    async fn execute(&self, url: &str, method: &str) -> Result<ApiDebugResult, HttpClientError>;
}

#[derive(Debug)]
pub struct ApiDebugResult {
    pub response: HttpResponse,
    pub formatted_body: Option<String>,
}

pub struct ApiDebugUseCaseImpl<C: HttpClient> {
    client: C,
}

impl<C: HttpClient> ApiDebugUseCaseImpl<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }
}

#[async_trait]
impl<C: HttpClient> ApiDebugUseCase for ApiDebugUseCaseImpl<C> {
    async fn execute(&self, url: &str, method: &str) -> Result<ApiDebugResult, HttpClientError> {
        let request = HttpRequest {
            id: uuid::Uuid::new_v4(),
            url: Url::parse(url).map_err(|e| HttpClientError::InvalidUrl(e.to_string()))?,
            method: method.parse().map_err(HttpClientError::InvalidUrl)?,
            headers: Headers::new(),
            body: None,
            timeout: std::time::Duration::from_secs(30),
        };

        let response = self.client.execute(&request).await?;

        let formatted_body = response
            .body
            .as_ref()
            .map(|b| format_response_body(b.content(), b.content_type()));

        Ok(ApiDebugResult {
            response,
            formatted_body,
        })
    }
}

fn format_response_body(body: &str, content_type: ContentType) -> String {
    match content_type {
        ContentType::Json => serde_json::from_str::<serde_json::Value>(body)
            .map(|v| serde_json::to_string_pretty(&v).unwrap_or(body.to_string()))
            .unwrap_or_else(|_| body.to_string()),
        _ => body.to_string(),
    }
}
