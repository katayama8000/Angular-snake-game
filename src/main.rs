#![allow(unused)]

pub use self::error::{Error, Result};
use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service, Route},
    Router,
};

use serde::Deserialize;
use tower_cookies::{CookieManager, CookieManagerLayer};
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    let routers_all = Router::new()
        .merge(routes_health_check())
        .merge(web::routes_login::routes())
        .route_layer(middleware::from_fn(web::mw_auth::mw_requier_auth))
        .layer(CookieManagerLayer::new())
        .fallback_service(route_static());
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routers_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_health_check() -> Router {
    Router::new().route("/", get("healthy"))
}
