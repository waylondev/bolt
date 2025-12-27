//! 领域服务模块
//!
//! 定义领域层的服务接口和业务逻辑

mod errors;
mod http_client;
mod dto;

pub use errors::HttpClientError;
pub use http_client::HttpClient;
pub use dto::RequestOptions;