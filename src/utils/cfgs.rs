use serde::Deserialize;
/// 配置文件
#[derive(Debug, Deserialize)]
pub struct Configs {
    pub system: System,
}

/// server 配置文件
#[derive(Debug, Deserialize)]
pub struct Server {
    /// 服务器名称
    pub name: String,
    /// 服务器(IP地址:端口)
    /// `0.0.0.0:3000`
    pub address: String,
    /// 服务器ssl
    pub ssl: bool,
    /// 响应数据gzip
    pub content_gzip: bool,
    /// 缓存时间
    pub cache_time: u64,
    /// 缓存方式
    pub cache_method: u32,
    /// api 前缀  例如："/api_v1"
    pub api_prefix: String,
}

/// system 系统配置
#[derive(Debug, Deserialize)]
pub struct System {
    /// 域名
    pub pint_host: Vec<String>,
    pub t: u64, //秒
    pub send_host: String,
    pub client_name: String,
}
