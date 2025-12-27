use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Timeout exceeded")]
    Timeout,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("SSL error: {0}")]
    SslError(String),

    #[error("Too many redirects")]
    TooManyRedirects,

    #[error("Request cancelled")]
    Cancelled,

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<reqwest::Error> for HttpClientError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            Self::Timeout
        } else if e.is_redirect() {
            Self::TooManyRedirects
        } else if e.is_connect() {
            Self::ConnectionFailed(e.to_string())
        } else if e.is_body() || e.is_decode() {
            Self::SerializationError(e.to_string())
        } else {
            Self::Unknown(e.to_string())
        }
    }
}

impl From<hyper::Error> for HttpClientError {
    fn from(e: hyper::Error) -> Self {
        Self::ConnectionFailed(e.to_string())
    }
}
