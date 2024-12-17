use serde::{Deserialize, Serialize};


/// 请求参数，必填
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DisasmRequest{
    pub proxy_scheme: String,
    pub proxy_host: String,
    pub proxy_port: String,
    pub proxy_username: String,
    pub proxy_password: String,
    pub url: String
}


