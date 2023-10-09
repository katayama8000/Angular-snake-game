use axum::{routing::get, Router};
use std::net::SocketAddr;

mod web;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(web::routes_login::routes())
        .route("/", get(|| async { "health check" }));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
