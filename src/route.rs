use axum::{
    routing::{delete, get, patch, post},
    Router,
};

use crate::{fallback_process::{method_not_allowed_fallback, not_found_handler}, handle::{cloudflaer_disasm, create_handler, delete_handler, extraction_body, extraction_body_bytes, extraction_body_custom, extraction_body_err, extraction_body_str, extraction_headers, extraction_path, extraction_path_and_query, extraction_query, extraction_request, extraction_state_counter, index, list_handler, update_handler}, state::AppState};

/// API 路由
pub fn routers() -> axum::Router {
    let state = AppState::default();
    Router::new()
        .route("/", get(index))
        .route("/api/disasm", post(cloudflaer_disasm))


        .route("/todos", get(list_handler))
        .route("/todos/:id", get(update_handler))
        .route("/todos", post(create_handler))
        .route("/todos/:id", delete(delete_handler))
        .route("/todos/:id", patch(update_handler))


        .route("/extraction/path/:id", get(extraction_path))
        .route("/extraction/query", get(extraction_query))
        .route("/extraction/path/query/:id", get(extraction_path_and_query))
        
        .route("/extraction/body", post(extraction_body))
        .route("/extraction/body/err", post(extraction_body_err))
        .route("/extraction/body/custom", post(extraction_body_custom))
        .route("/extraction/headers", get(extraction_headers))
        .route("/extraction/request", get(extraction_request))
        .route("/extraction/body/string", get(extraction_body_str))
        .route("/extraction/body/bytes", get(extraction_body_bytes))
        .route("/extraction/state/counter", get(extraction_state_counter))
        
        .method_not_allowed_fallback(method_not_allowed_fallback)
        .fallback(not_found_handler)
        .with_state(state)
}
