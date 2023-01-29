use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
struct RootAPIResponse {
    author: String,
    version: String,
}

pub async fn root() -> impl IntoResponse {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or("0.0.0".to_string());

    Json(RootAPIResponse {
        author: "Gilles De Mey".to_string(),
        version,
    })
}
