use axum::{response::IntoResponse, Json};
use serde::Serialize;

use crate::error::ApiError;

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
    pub data: Option<T>,
}

/// ApiResponse -> Axum Response
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl ApiResponse<()> {
    pub fn empty() -> Self {
        Self::new(ResponseCode::Success.into(), SUCCESS_MSG, None)
    }
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn ok(data: T) -> Self {
        Self::new(ResponseCode::Success.into(), SUCCESS_MSG, Some(data))
    }
    pub fn ok_with_msg(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::Success.into(), message, None)
    }
    pub fn error_with_msg(message: impl Into<String>) -> Self {
        Self::new(ResponseCode::Error.into(), message.into(), None)
    }
    pub fn error(error: ApiError) -> Self {
        Self::new(ResponseCode::Error.into(), error.to_string(), None)
    }
    pub fn error_with_data(message: impl Into<String>, data: T) -> Self {
        Self::new(ResponseCode::Error.into(), message, Some(data))
    }
    fn new(code: i32, message: impl Into<String>, data: Option<T>) -> Self {
        Self {
            code: code,
            message: message.into(),
            data,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResponseCode {
    Success = 0,
    Error = 1,
}

impl From<ResponseCode> for i32 {
    fn from(value: ResponseCode) -> Self {
        match value {
            ResponseCode::Success => 0,
            ResponseCode::Error => 1,
        }
    }
}


/// ApiResponse<T> 转换为 Ok(ApiResponse<T>)
impl<T, E> From<ApiResponse<T>> for Result<ApiResponse<T>, E>{
    fn from(value: ApiResponse<T>) -> Self {
        Ok(value)
    }
}