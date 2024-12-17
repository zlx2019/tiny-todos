#![allow(unused_variables)]
#![allow(dead_code)]

use std::net::SocketAddr;

use anyhow::Context;
use tiny_todos::{logger, route};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    logger::logger_init();
    let app = route::routers();
    let addr = SocketAddr::from(([127, 0, 0, 1], 15001));
    let listen = TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind server to"))
        .unwrap();
    info!("Server listening on {}", addr);
    axum::serve(listen, app).await.unwrap();
}