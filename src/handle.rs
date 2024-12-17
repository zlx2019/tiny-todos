#![allow(unused_variables)]
#![allow(dead_code)]

use std::{collections::HashMap, sync::atomic::Ordering};
use axum::{body::Bytes, extract::{rejection::JsonRejection, Path, Query, Request, State}, http::{HeaderMap, Method}, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{error::ApiError, reader::RequestBody, response::ApiResponse, state::AppState, types::{BodyParams, DisasmRequest, Pagination, QueryParams}};

/// Home
pub async fn index() -> &'static str {
    "Hello World!"
}

/// Cloudflare Api
pub async fn cloudflaer_disasm(Json(payload): Json<DisasmRequest>) -> impl IntoResponse{
    ApiResponse::success(payload)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoReq{
    pub title: String,
    pub desc: String,
    pub status: u8
}
/// 从 Request 中提取 Body 映射为 Json<CrateTodoReq>
pub async fn create_handler(Json(req): Json<CreateTodoReq>) -> impl IntoResponse {
    info!("{:?}", req);
}
pub async fn update_handler() -> impl IntoResponse {
    
}
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
pub async fn extraction_path(Path(id): Path<usize>) -> impl IntoResponse{
    info!("extraction path id: {}", id);
    ApiResponse::success(id)
}



/// 从 Request Query 中提取参数, 映射为 QueryParams 实体
pub async fn extraction_query(Query(param): Query<QueryParams>) -> impl IntoResponse{
    info!("extraction query params: {:?}", param);
    ApiResponse::success(param)
}

/// 同时提取 path 和 query 多部分参数
pub async fn extraction_path_and_query(Path(id): Path<usize>, pagination: Option<Query<Pagination>>) -> impl IntoResponse{
    let Query(pagination) = pagination.unwrap_or_default();
    info!("extraction path and query, id: {}, pagintion: {:?}", id, pagination);
    ApiResponse::success(pagination)
}


/// 从 Request Body 提取参数，映射为 CreateTodoReq 实体
pub async fn extraction_body(Json(req): Json<BodyParams>) -> impl IntoResponse{
    info!("extraction body req: {:?}", req);
    ApiResponse::success(req)
}

/// 使用自定义 Request Body 提取器
pub async fn extraction_body_custom(RequestBody(req): RequestBody<BodyParams>) -> impl IntoResponse{
    info!("extraction body by custom: {:?}", req);
    ApiResponse::success(req)
}

/// 从 Request Body 提取参数，处理提取错误情况
pub async fn extraction_body_err(req: Result<Json<BodyParams>, JsonRejection>) -> Result<impl IntoResponse, ApiError>{
    match req {
        Ok(Json(payload)) => {
            // 提取成功
            info!("extraction body req: {:?}", payload);
            return Ok(ApiResponse::success(payload));
        },
        Err(e) => {
            let error = match e {
                JsonRejection::MissingJsonContentType(ct) => {
                    info!("{:?}", ct);
                    ApiError::RequestUnsupportedMediaType(ct.to_string())
                },
                JsonRejection::JsonDataError(_) | JsonRejection::JsonSyntaxError(_) => ApiError::RequestParamError,
                _ => ApiError::SysError,
            };
            Err(error)
        }
    }
}


/// 提取请求头信息
pub async fn extraction_headers(headers: HeaderMap) -> impl IntoResponse{
    info!("extraction headers: {:?}", headers);
    let maps: HashMap<String, String> = headers
        .iter()
        .filter_map(|(key, value)| {
            value.to_str().ok().map(|v| (key.to_string(), v.to_string()))
        })
        .collect();
    ApiResponse::success(maps)
}


/// 提取整个HttpReqeust 请求信息
pub async fn extraction_request(request: Request) -> impl IntoResponse{
    info!("extraction request {:?}", request);
    ApiResponse::success(request.method().to_string())
}

/// 以字符串形式提取 RequestBody
pub async fn extraction_body_str(body: String) -> impl IntoResponse{
    info!("extraction body String: {}", body);
    ApiResponse::success(body)
}

/// 以字节流形式提取 RequestBody  并确保它是有效的utf-8
pub async fn extraction_body_bytes(body_bytes: Bytes) -> Result<impl IntoResponse, ApiError>{
    let res =  String::from_utf8(body_bytes.to_vec()).map_err(|e| format!("invalid UTF-8: {}",e));
    match res {
        Ok(body) => Ok(ApiResponse::success(body)),
        Err(err_msg) => Err(ApiError::SysError),
    }
}

/// 提取全局状态
pub async fn extraction_state_counter(State(state): State<AppState>) -> impl IntoResponse{
    // 访问统计递增1
    let counter = state.counter.fetch_add(1, Ordering::SeqCst);
    ApiResponse::success(counter)
}


/// 从请求中提取数据的顺序
///     - Request Body 是异步流，只能读取一次，所以一定要放在最后提取。
///     - Request Method & Request Headers 可以放在任意位置提取，但一定在 Request Body 之前。
///     - State 提取器也要在 Body 提取之前。
pub async fn extraction_order(
    method: Method,
    headers: HeaderMap,
    State(state): State<AppState>,
    body: String
) -> impl IntoResponse{
    ApiResponse::ok()
}