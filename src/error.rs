#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::fmt::Display;

use crate::response::ApiResponse;
use axum::{extract::rejection::FormRejection, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

/// Api Error
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Error")]
    SysError,
    #[error("{0}")]
    BusinessError(String),

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

    #[error("{0}")]
    RequestUnsupportedMediaType(String),

    // Request params error
    #[error("{0}")]
    RequestParamError(String),
    // RequestBody error
    #[error("{0}")]
    RequestBodyError(String),
    

    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

/// 实现 ApiError -> Axum Response 的转换
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let mut message = self.to_string();
        let (http_status) = match self {
            ApiError::SysError | ApiError::BusinessError(_) => (StatusCode::INTERNAL_SERVER_ERROR),
            ApiError::RouteNotFound => (StatusCode::NOT_FOUND),
            ApiError::RequestUnsupportedMediaType(_) => (StatusCode::UNSUPPORTED_MEDIA_TYPE),
            ApiError::MethodNotAllowed => (StatusCode::METHOD_NOT_ALLOWED),
            ApiError::RequestBodyError(_)
            | ApiError::RequestParamError(_) => (StatusCode::BAD_REQUEST),
            ApiError::Unauthorized
            | ApiError::TokenSyntaxError
            | ApiError::TokenExpired
            | ApiError::TokenInvalid => (StatusCode::UNAUTHORIZED),
            ApiError::ValidationError(errors) => {
                message = format!("Input validation error: [{errors}]").replace('\n', ", ");
                StatusCode::BAD_REQUEST
            },
            ApiError::AxumFormRejection(e) => {
                message = e.body_text();
                (StatusCode::BAD_REQUEST)
            }
        };
        (
            http_status,
            Json(ApiResponse::<()>::error_with_msg(message)),
        )
            .into_response()
    }
}

/// Json parse fail info
#[derive(Debug, Serialize)]
pub struct JsonParseLocation {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl Display for JsonParseLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "line: {}, column: {}", self.line, self.column)
    }
}
