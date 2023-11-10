use axum::prelude::*;
use std::net::SocketAddr;
mod routes;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/",get(|| async { "Welcome Axum!!" }))
        
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

