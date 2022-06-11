

pub mod configuration;
pub mod handler;

use std::net::SocketAddr;

use axum::{Router, routing::get};

pub async fn run(addr: &SocketAddr) {
    let app = Router::new()
        .route("/", get(handler::usage));
    
    axum::Server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}