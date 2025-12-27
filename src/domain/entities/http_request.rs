use crate::domain::value_objects::{Body, Headers, HttpMethod, Url};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub id: uuid::Uuid,
    pub url: Url,
    pub method: HttpMethod,
    pub headers: Headers,
    pub body: Option<Body>,
    pub timeout: Duration,
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            url: Url::parse("https://httpbin.org/get").unwrap(),
            method: HttpMethod::GET,
            headers: Headers::new(),
            body: None,
            timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Headers,
    pub body: Option<Body>,
    pub duration: Duration,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl HttpResponse {
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }
}
