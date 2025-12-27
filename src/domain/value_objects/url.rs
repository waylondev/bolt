use std::fmt;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Url(String);

#[derive(Debug, Error)]
pub enum UrlError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
    #[error("Missing scheme")]
    MissingScheme,
    #[error("Unsupported scheme: {0}")]
    UnsupportedScheme(String),
}

impl Url {
    pub fn parse(s: &str) -> Result<Self, UrlError> {
        let parsed = url::Url::parse(s).map_err(|e| UrlError::InvalidUrl(e.to_string()))?;

        match parsed.scheme() {
            "http" | "https" => Ok(Self(s.to_string())),
            scheme => Err(UrlError::UnsupportedScheme(scheme.to_string())),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Url {
    fn default() -> Self {
        Self("https://httpbin.org/get".to_string())
    }
}
