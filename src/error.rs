
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;
use crate::response::{ApiResponse, ResponseCode};


/// Api Error
#[derive(Error, Debug, Serialize)]
pub enum ApiError{
    #[error("system error")]
    SysError,
    #[error("api not found")]
    RouteNotFound,
    #[error("invalid parameter: {0}")]
    ValidationError(String),
}

/// 实现 ApiError -> Axum Response 的转换
impl IntoResponse for ApiError{
    fn into_response(self) -> axum::response::Response {
        let message = self.to_string();
        let (http_status, code) = match self {
            ApiError::SysError => (StatusCode::INTERNAL_SERVER_ERROR, ResponseCode::Error),
            ApiError::RouteNotFound => (StatusCode::NOT_FOUND, ResponseCode::Error),
            ApiError::ValidationError(_) => (StatusCode::BAD_REQUEST, ResponseCode::Error),
        };
        (http_status, Json(ApiResponse::<()>::error(code, message))).into_response()
    }
}