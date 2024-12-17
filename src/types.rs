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


/// 分页参数
#[derive(Debug, Deserialize, Serialize)]
pub struct Pagination {
    // 页码
    pub page: usize,
    // 页容量
    pub per_page: usize
}
impl Default for Pagination{
    fn default() -> Self {
        Self { page: 1, per_page: 10 }
    }
}



/// Request Query 参数体
#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams{
    pub id: usize,
    pub name: String
}



/// Request Body 参数体
#[derive(Debug, Deserialize, Serialize)]
pub struct BodyParams{
    pub id: usize,
    pub name: String,
    pub lock: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>
}