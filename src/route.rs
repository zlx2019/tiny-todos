#![allow(unused_variables)]
#![allow(dead_code)]


use std::time::Duration;

use axum::{
    http::{Request, Response}, middleware::from_fn, routing::{delete, get, patch, post}, Router
};
use tower::ServiceBuilder;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use tracing::Span;

use crate::{
    fallback_process::{method_not_allowed_fallback, not_found_handler}, handle::{
        create_handler, delete_handler, extension_query_valida, extraction_body, extraction_body_bytes, extraction_body_custom, extraction_body_str, extraction_headers, extraction_path, extraction_path_and_query, extraction_path_many, extraction_query, extraction_request, extraction_state_counter, index, list_handler, update_handler
    }, middlewares, state::AppState
};

pub fn routers() -> axum::Router {
    // global state
    let state = AppState::default();
    // request trace
    let request_trace = TraceLayer::new_for_http()
        .on_request(| req: &Request<_>, _: &Span |{
            // tracing::info!("[Request] {} | {} | [{:?}]", req.method(), req.uri(), req.version());
        })
        .on_response(|res: &Response<_>, latency: Duration, _: &Span| {
            // tracing::info!("[Response] status={}, latency={}ms",res.status().as_u16(),latency.as_millis());
        }
    );
    // register middlewares
    Router::new()
        .merge(api_route())
        .merge(api_example_route(state))
        .merge(fallback_route())
        .layer(ServiceBuilder::new()
                .layer(CorsLayer::new().allow_methods(Any).allow_headers(Any).allow_origin(Any))
                .layer(from_fn(middlewares::authenticator::authorization))
                .layer(request_trace))
}

/// 业务api
pub fn api_route() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/todos", get(list_handler))
        .route("/todos/:id", get(update_handler))
        .route("/todos", post(create_handler))
        .route("/todos/:id", delete(delete_handler))
        .route("/todos/:id", patch(update_handler))
}

/// 示例api
pub fn api_example_route(state: AppState) -> Router {
    Router::new()
        .route("/extraction/path/:id", get(extraction_path))
        .route("/extraction/path/many/:id/:name", get(extraction_path_many))
        .route("/extraction/query/many", get(extraction_query))
        .route("/extraction/query/valida", get(extension_query_valida))
        .route("/extraction/path/query/:id", get(extraction_path_and_query))
        .route("/extraction/body", post(extraction_body))
        .route("/extraction/body/custom", post(extraction_body_custom))
        .route("/extraction/headers", get(extraction_headers))
        .route("/extraction/request", get(extraction_request))
        .route("/extraction/body/string", get(extraction_body_str))
        .route("/extraction/body/bytes", get(extraction_body_bytes))
        .route("/extraction/state/counter", get(extraction_state_counter))
        .with_state(state)
}


/// 非正常处理回调
pub fn fallback_route() -> Router {
    Router::new()
        .method_not_allowed_fallback(method_not_allowed_fallback)
        .fallback(not_found_handler)
}
