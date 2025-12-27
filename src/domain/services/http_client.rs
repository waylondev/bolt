//! HTTP客户端服务接口
//!
//! 定义HTTP客户端的领域服务接口

use crate::domain::entities::HttpRequest;
use async_trait::async_trait;

/// HTTP客户端服务接口
#[async_trait]
pub trait HttpClient: Send + Sync {
    /// 执行HTTP请求
    async fn execute(
        &self,
        request: &HttpRequest,
    ) -> Result<crate::domain::entities::HttpResponse, super::HttpClientError>;
    
    /// 关闭客户端连接
    async fn close(&self) -> Result<(), super::HttpClientError>;
}