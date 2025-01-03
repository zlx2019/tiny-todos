#![allow(unused_variables)]
#![allow(dead_code)]

use axum::{
    body::Bytes, extract::{Path, Query, Request, State}, http::{HeaderMap, Method}, response::IntoResponse, Json
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::atomic::Ordering};
use tracing::info;

use crate::{
    error::ApiError, extractors::{request_body::{RequestBody, ValidateRequestBody}, request_path::RequestPath}, response::ApiResponse, state::AppState, types::{BodyParams, Pagination, PathParams, QueryParams}, validations::form_validate::ValidateForm
};

/// Home
pub async fn index() -> Result<impl IntoResponse, ApiError> {
    Ok(ApiResponse::ok("Hello"))
}

/// 标准API
pub async fn std_api() -> Result<impl IntoResponse, ApiError> {
    if true {
        Ok(ApiResponse::ok("Hello Ok."))
        // or
        // ApiResponse::ok("Hello Ok").into()
    } else {
        Err(ApiError::MethodNotAllowed)
        // or
        // ApiError::SysError.into()
    }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoReq {
    pub title: String,
    pub desc: String,
    pub status: u8,
}
/// 从 Request 中提取 Body 映射为 Json<CrateTodoReq>
pub async fn create_handler(Json(req): Json<CreateTodoReq>) -> impl IntoResponse {
    info!("{:?}", req);
}
pub async fn update_handler() -> impl IntoResponse {}
pub async fn delete_handler(Path(id): Path<usize>) -> impl IntoResponse {
    info!("Delete todo id: {}", id);
}

pub async fn get_handler(Query(id): Query<usize>) -> impl IntoResponse {
    info!("Get by id: {}", id);
}

pub async fn list_handler(Query(req): Query<HashMap<String, String>>) -> impl IntoResponse {
    info!("List Handler req {:?}", req);
}

/// ========================常用的提取参数方式========================

/// 从 Request Path 中提取参数
pub async fn extraction_path(Path(id): Path<usize>) -> impl IntoResponse {
    info!("extraction path id: {}", id);
    ApiResponse::ok(id)
}

/// 从 Request Path 中提取多个参数
pub async fn extraction_path_many(RequestPath(req): RequestPath<PathParams>) -> impl IntoResponse {
    info!("extraction path custom: {:?}", req);
    ApiResponse::ok(req)
}

/// 从 Request Query 中提取参数, 映射为 QueryParams 实体
pub async fn extraction_query(Query(param): Query<QueryParams>) -> impl IntoResponse {
    info!("extraction query params: {:?}", param);
    ApiResponse::ok(param)
}

/// Query 参数校验
pub async fn extension_query_valida(ValidateForm(req): ValidateForm<QueryParams>) -> impl IntoResponse{
    info!("{:?}", req);
    ApiResponse::ok(req)
}

/// 同时提取 path 和 query 多部分参数
pub async fn extraction_path_and_query(
    Path(id): Path<usize>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    info!(
        "extraction path and query, id: {}, pagintion: {:?}",
        id, pagination
    );
    ApiResponse::ok(pagination)
}

/// 从 Request Body 提取参数，映射为 CreateTodoReq 实体
pub async fn extraction_body(Json(req): Json<BodyParams>) -> impl IntoResponse {
    info!("extraction body req: {:?}", req);
    ApiResponse::ok(req)
}

/// 使用自定义 Request Body 提取器
pub async fn extraction_body_custom(
    RequestBody(req): RequestBody<BodyParams>,
) -> impl IntoResponse {
    info!("extraction body by custom: {:?}", req);
    ApiResponse::ok(req)
}

/// 使用自定义 RequestBydo
pub async fn extraction_body_validate(ValidateRequestBody(req): ValidateRequestBody<BodyParams>) -> impl IntoResponse{
    info!("extraction body by validate: {:?}", req);
    ApiResponse::ok(req)
}

/// 提取请求头信息
pub async fn extraction_headers(headers: HeaderMap) -> impl IntoResponse {
    // info!("extraction headers: {:?}", headers);
    let maps: HashMap<String, String> = headers
        .iter()
        .filter_map(|(key, value)| {
            value
                .to_str()
                .ok()
                .map(|v| (key.to_string(), v.to_string()))
        })
        .collect();
    ApiResponse::ok(maps)
}

/// 提取整个HttpReqeust 请求信息
pub async fn extraction_request(request: Request) -> impl IntoResponse {
    info!("extraction request {:?}", request);
    ApiResponse::ok(request.method().to_string())
}

/// 以字符串形式提取 RequestBody
pub async fn extraction_body_str(body: String) -> impl IntoResponse {
    info!("extraction body String: {}", body);
    ApiResponse::ok(body)
}

/// 以字节流形式提取 RequestBody  并确保它是有效的utf-8
pub async fn extraction_body_bytes(body_bytes: Bytes) -> Result<impl IntoResponse, ApiError> {
    let res = String::from_utf8(body_bytes.to_vec()).map_err(|e| format!("invalid UTF-8: {}", e));
    match res {
        Ok(body) => Ok(ApiResponse::ok(body)),
        Err(err_msg) => Err(ApiError::SysError),
    }
}

/// 提取全局状态
pub async fn extraction_state_counter(State(state): State<AppState>) -> impl IntoResponse {
    // 访问统计递增1
    let counter = state.counter.fetch_add(1, Ordering::SeqCst);
    ApiResponse::ok(counter)
}

/// 从请求中提取数据的顺序
///     - Request Body 是异步流，只能读取一次，所以一定要放在最后提取。
///     - Request Method & Request Headers 可以放在任意位置提取，但一定在 Request Body 之前。
///     - State 提取器也要在 Body 提取之前。
pub async fn extraction_order(
    method: Method,
    headers: HeaderMap,
    State(state): State<AppState>,
    body: String,
) -> impl IntoResponse {
    ApiResponse::empty()
}
