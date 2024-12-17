use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use tracing::info;

use crate::{error::ApiError, response::ApiResponse, types::DisasmRequest};

/// Home
pub async fn index() -> &'static str {
    "Hello World!"
}

pub async fn ok_empty() -> impl IntoResponse {
    ApiResponse::ok()
}

/// Cloudflare Api
pub async fn cloudflaer_disasm(Json(payload): Json<DisasmRequest>) -> impl IntoResponse{
    ApiResponse::success(payload)
}

/// 404 handler
pub async fn handler_404() -> impl IntoResponse {
    ApiError::RouteNotFound
}



#[derive(Debug, Deserialize)]
pub struct CreateTodoReq{
    pub title: String,
    pub desc: String,
    pub status: u8
}
pub async fn create_handler(Json(req): Json<CreateTodoReq>) -> impl IntoResponse {
    info!("{:?}", req);
}

pub async fn update_handler() -> impl IntoResponse {
    
}

pub async fn delete_handler() -> impl IntoResponse {
    
}

pub async fn get_handler() -> impl IntoResponse {
    
}

pub async fn list_handler() -> impl IntoResponse {
    
}
