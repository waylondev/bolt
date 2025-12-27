//! 数据传输对象模块
//!
//! 定义服务层使用的数据传输对象

/// HTTP请求选项配置
#[derive(Debug, Clone)]
pub struct RequestOptions {
    /// 是否跟随重定向
    pub follow_redirects: bool,
    /// 重定向限制次数
    pub redirect_limit: u32,
    /// 超时时间（秒）
    pub timeout_seconds: u64,
    /// 是否验证SSL证书
    pub verify_ssl: bool,
}

impl Default for RequestOptions {
    fn default() -> Self {
        Self {
            follow_redirects: true,
            redirect_limit: 10,
            timeout_seconds: 30,
            verify_ssl: true,
        }
    }
}

impl RequestOptions {
    /// 创建严格的安全配置
    pub fn strict_security() -> Self {
        Self {
            follow_redirects: false,
            redirect_limit: 0,
            timeout_seconds: 60,
            verify_ssl: true,
        }
    }
    
    /// 创建宽松的测试配置
    pub fn lenient_testing() -> Self {
        Self {
            follow_redirects: true,
            redirect_limit: 20,
            timeout_seconds: 10,
            verify_ssl: false,
        }
    }
}