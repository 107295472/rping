use serde::{Deserialize, Serialize};
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct Pingdb {
    pub name: String,
    pub host: Vec<Sdata>,
}

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct Sdata {
    pub name: String,
    pub avg: String,
    pub curtime: String,
    pub min: i32,
    pub max: i32,
}
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct AlctAPIModel {
    pub token: Option<String>,
    pub method_adress: String,
    pub method_domain: String,
    pub menthod: String,
    pub data: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct IpResult {
    ip: String,
    timeout: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct CommandResult {
    name: String,
    ipv4: Vec<IpResult>,
    ipv6: Vec<IpResult>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocalToken {
    pub token: Option<String>,
    pub refresh_token: Option<String>,
    pub expiry_in: Option<i64>,
    pub last_time: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
}
