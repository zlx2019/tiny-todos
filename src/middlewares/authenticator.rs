#![allow(unused_variables)]
#![allow(dead_code)]


use axum::{
    body::Body, http::{self, Request, StatusCode}, middleware::Next, response::Response
};

use crate::error::ApiError;

const TOKEN_PREFIX: &str = "Bearer ";

/// authorization 认证中间件
pub async fn authorization(req: Request<Body>, next: Next) -> Result<Response, (StatusCode, ApiError)> {
    // get user token
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, ApiError::Unauthorized))?;
    let token = token.strip_prefix(TOKEN_PREFIX)
        .ok_or((StatusCode::UNAUTHORIZED, ApiError::TokenSyntaxError))?;
    // TODO token check
    tracing::debug!("Authorization token: {}", token);
    Ok(next.run(req).await)
}

