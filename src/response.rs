use axum::{response::IntoResponse, Json};
use serde::Serialize;

const SUCCESS_MSG: &str = "success";


/// API Response
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    // 响应码, 0: 请求成功, 其他表示错误码
    pub code: i32,  
    // 响应消息
    pub message: String,
    // 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>
}


/// ApiResponse -> Axum Response
impl<T: Serialize> IntoResponse for ApiResponse<T>{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl ApiResponse<()> {
    // 响应成功，无数据
    pub fn ok() -> Self{
        Self::new(ResponseCode::Success, SUCCESS_MSG, None)
    }
    pub fn error(code: ResponseCode, message: impl Into<String>) -> Self {
        Self::new(code, message, None)
    }
}

impl<T> ApiResponse<T> where 
    T: Serialize
{
    pub fn success(data: T) -> Self {
        Self::new(ResponseCode::Success, SUCCESS_MSG, Some(data))
    }
    pub fn ok_with_msg(message: impl Into<String>) -> Self{
        Self::new(ResponseCode::Success, message, None)
    }

    fn new(code: ResponseCode, message: impl Into<String>, data: Option<T>) -> Self{
        Self{
            code: code as i32,
            message: message.into(),
            data
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum ResponseCode{
    Success = 0,
    Error = 1,
}
