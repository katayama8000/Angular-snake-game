use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/api/login", get("api_login"))
}
