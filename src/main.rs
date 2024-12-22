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
    let addr: SocketAddr = format!("127.0.0.1:15001")
        .parse()
        .expect("can not parse address and port");
    let listen = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind server to"))
        .unwrap();
    info!("server started successfully listening on {}", addr);
    axum::serve(listen, app)
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();
}

/// 等待信号停止服务
async fn graceful_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect shutdown signal handler");
    info!("server stopped successfully");
    // todo do clean handler
}
