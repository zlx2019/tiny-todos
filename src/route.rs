use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::handle::{cloudflaer_disasm, create_handler, delete_handler, handler_404, index, list_handler, ok_empty, update_handler};

/// API 路由
pub fn routers() -> axum::Router {
    
    Router::new()
        .route("/", get(index))
        .route("/api/disasm", post(cloudflaer_disasm))


        .route("/tests/empty", get(ok_empty))


        .route("/todos", get(list_handler))
        .route("/todos/:id", get(update_handler))
        .route("/todos", post(create_handler))
        .route("/todos/:id", delete(delete_handler))
        .route("/todos/:id", patch(update_handler))
        .fallback(handler_404)
}
