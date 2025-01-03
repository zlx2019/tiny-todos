#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_parens)]

use std::collections::HashMap;

use crate::response::ApiResponse;
use axum::{
    extract::rejection::{FormRejection, JsonRejection, PathRejection},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use thiserror::Error;
use tracing::{error, info};
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

    /// JSON & Path 参数解析错误转换
    #[error(transparent)]
    RequestPathError(#[from] PathRejection),
    #[error(transparent)]
    RequestBodyJsonError(#[from] JsonRejection),

    // Axum from valida error convert
    // #[error(transparent)]
    #[error("Parameter validation failed")]
    ValidationError(#[from] ValidationErrors),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl ApiError {
    /// 获取 ApiError 响应的HTTP状态码和响应消息
    pub fn get_msg_states(&self) -> (StatusCode, String) {
        match self {
            // 应用错误
            ApiError::SysError | ApiError::BusinessError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            ApiError::RouteNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::MethodNotAllowed => (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string()),
            ApiError::RequestUnsupportedMediaType(_) => {
                (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string())
            }

            // 认证错误
            ApiError::Unauthorized
            | ApiError::TokenSyntaxError
            | ApiError::TokenExpired
            | ApiError::TokenInvalid => (StatusCode::UNAUTHORIZED, self.to_string()),

            // 参数校验错误
            ApiError::ValidationError(errors) => {
                let formatted_msg = format!("[{errors}]").replace('\n', ", ");
                info!("{}", formatted_msg);
                (StatusCode::BAD_REQUEST, self.to_string())
            }

            // 参数解析错误
            ApiError::AxumFormRejection(form_err) => {
                (StatusCode::BAD_REQUEST, form_err.body_text())
            }
            ApiError::RequestBodyJsonError(json_err) => {
                (StatusCode::BAD_REQUEST, json_err.body_text())
            }
            ApiError::RequestPathError(path_err) => (StatusCode::BAD_REQUEST, path_err.body_text()),
        }
    }
}

/// 实现 ApiError -> Axum Response 的转换
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // 获取错误对应的响应码和响应消息
        let (status_code, message) = self.get_msg_states();
        match &self {
            ApiError::ValidationError(errors) => {
                let mut error_map = HashMap::new();
                for (field, field_errors) in errors.field_errors() {

                    let messages: Vec<String> = field_errors
                        .iter()
                        .map(|err| {
                            err.message
                                .as_ref()
                                .map(|msg| msg.to_string())
                                .unwrap_or_else(|| err.code.to_string())
                        }).collect();
                    error_map.insert(field.to_string(), messages);
                }
                let s = errors.to_string();
                (status_code, Json(ApiResponse::error_with_data(message, error_map))).into_response()
            }
            _ => {
                (status_code, Json(ApiResponse::<()>::error_with_msg(message))).into_response()
            }
        }
    }
}

/// 将 ApiError 转换为 Err(ApiError)
impl<T> From<ApiError> for Result<ApiResponse<T>, ApiError> {
    fn from(value: ApiError) -> Self {
        Err(value)
    }
}
