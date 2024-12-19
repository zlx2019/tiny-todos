#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_parens)]


use crate::response::ApiResponse;
use axum::{extract::rejection::{FormRejection, JsonRejection, PathRejection}, http::StatusCode, response::IntoResponse, Json};
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

    /// JSON & Path 参数解析错误转换
    #[error(transparent)]
    RequestPathError(#[from] PathRejection),
    #[error(transparent)]
    RequestBodyJsonError(#[from] JsonRejection),

    // Axum from valida error convert 
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

/// 实现 ApiError -> Axum Response 的转换
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        // TODO OPT
        let (status_code, message) = match self {
            // 应用错误
            ApiError::SysError | 
            ApiError::BusinessError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            },
            ApiError::RouteNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::MethodNotAllowed => (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string()),
            ApiError::RequestUnsupportedMediaType(_) => (StatusCode::UNSUPPORTED_MEDIA_TYPE, self.to_string()),

            // 认证错误
            ApiError::Unauthorized |  
            ApiError::TokenSyntaxError |  
            ApiError::TokenExpired |  
            ApiError::TokenInvalid => {  
                (StatusCode::UNAUTHORIZED, self.to_string())  
            },

            // 参数校验错误  
            ApiError::ValidationError(errors) => {  
                let formatted_msg = format!("Input validation error: [{errors}]").replace('\n', ", ");  
                (StatusCode::BAD_REQUEST, formatted_msg)  
            }

            // 参数解析错误
            ApiError::AxumFormRejection(form_err) => {  
                (StatusCode::BAD_REQUEST, form_err.body_text())  
            }  
            ApiError::RequestBodyJsonError(json_err) => {  
                (StatusCode::BAD_REQUEST, json_err.body_text())  
            }  
            ApiError::RequestPathError(path_err) => {  
                (StatusCode::BAD_REQUEST, path_err.body_text())  
            }  
        };
        (status_code,Json(ApiResponse::<()>::error_with_msg(message)),).into_response()
    }
}
