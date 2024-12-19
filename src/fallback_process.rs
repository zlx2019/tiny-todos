use axum::{http::StatusCode, response::IntoResponse};

use crate::error::ApiError;

/// 405 Method Not Allowed Handler
pub async fn method_not_allowed_fallback() -> impl IntoResponse {
    (StatusCode::METHOD_NOT_ALLOWED, ApiError::MethodNotAllowed)
}

/// 404 Not Found Handler
pub async fn not_found_handler() -> impl IntoResponse {
    ApiError::RouteNotFound
}
