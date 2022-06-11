

pub mod configuration;
pub mod handler;
pub mod error;
pub mod response;

use std::net::SocketAddr;

use axum::{Router, routing::get};

pub use crate::response::Response;

/// 定义自己的 Result
type Result<T> = std::result::Result<T, error::AppError>;

pub async fn run(addr: &SocketAddr) {
    let app = Router::new()
        .route("/", get(handler::usage));
    
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}