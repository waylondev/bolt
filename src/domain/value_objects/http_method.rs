use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

impl HttpMethod {
    /// 转换为 reqwest::Method
    pub fn as_reqwest_method(&self) -> reqwest::Method {
        match self {
            Self::GET => reqwest::Method::GET,
            Self::POST => reqwest::Method::POST,
            Self::PUT => reqwest::Method::PUT,
            Self::DELETE => reqwest::Method::DELETE,
            Self::PATCH => reqwest::Method::PATCH,
            Self::HEAD => reqwest::Method::HEAD,
            Self::OPTIONS => reqwest::Method::OPTIONS,
        }
    }
    
    /// 从 reqwest::Method 转换
    pub fn from_reqwest(method: &reqwest::Method) -> Option<Self> {
        match *method {
            reqwest::Method::GET => Some(Self::GET),
            reqwest::Method::POST => Some(Self::POST),
            reqwest::Method::PUT => Some(Self::PUT),
            reqwest::Method::DELETE => Some(Self::DELETE),
            reqwest::Method::PATCH => Some(Self::PATCH),
            reqwest::Method::HEAD => Some(Self::HEAD),
            reqwest::Method::OPTIONS => Some(Self::OPTIONS),
            _ => None,
        }
    }
    
    /// 检查是否为幂等方法
    pub fn is_idempotent(&self) -> bool {
        matches!(self, Self::GET | Self::HEAD | Self::PUT | Self::DELETE | Self::OPTIONS)
    }
    
    /// 检查是否支持请求体
    pub fn supports_body(&self) -> bool {
        matches!(self, Self::POST | Self::PUT | Self::PATCH)
    }
}

impl From<HttpMethod> for reqwest::Method {
    fn from(method: HttpMethod) -> Self {
        method.as_reqwest_method()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum HttpMethod {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}


impl FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            _ => Err(format!("Invalid HTTP method: {}", s)),
        }
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
            Self::PATCH => "PATCH",
            Self::HEAD => "HEAD",
            Self::OPTIONS => "OPTIONS",
        };
        write!(f, "{}", method_str)
    }
}
