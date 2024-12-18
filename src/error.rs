#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_parens)]

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;
use crate::response::ApiResponse;


/// Api Error
#[derive(Error, Debug, Serialize)]
pub enum ApiError{
    #[error("Error")]
    SysError,
    #[error("Route not found")]
    RouteNotFound,
    #[error("Method not allowed")]
    MethodNotAllowed,

    #[error("Unauthorized")]
    Unauthorized,
    #[error("Token syntax error")]
    TokenSyntaxError,
    #[error("Token invalid")]
    TokenInvalid,
    #[error("Token expired")]
    TokenExpired,


    #[error("Invalid parameter: {0}")]
    ValidationError(String),


    // Content-Type 错误
    #[error("{0}")]
    RequestUnsupportedMediaType(String),
    // 请求参数错误
    #[error("Request parameter error")]
    RequestParamError,
}

/// 实现 ApiError -> Axum Response 的转换
impl IntoResponse for ApiError{
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();
        let (http_status) = match self {
            ApiError::SysError => (StatusCode::INTERNAL_SERVER_ERROR),
            ApiError::RouteNotFound => (StatusCode::NOT_FOUND),
            ApiError::ValidationError(_) => (StatusCode::BAD_REQUEST),
            ApiError::RequestUnsupportedMediaType(_) => (StatusCode::UNSUPPORTED_MEDIA_TYPE),
            ApiError::RequestParamError => (StatusCode::BAD_REQUEST),
            ApiError::MethodNotAllowed => (StatusCode::METHOD_NOT_ALLOWED),
            ApiError::Unauthorized | 
                ApiError::TokenSyntaxError | 
                ApiError::TokenExpired | 
                ApiError::TokenInvalid => (StatusCode::UNAUTHORIZED),
        };
        (http_status, Json(ApiResponse::<()>::error_with_msg(message))).into_response()
    }
}