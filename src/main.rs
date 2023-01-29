use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tracing::error;
use tracing_subscriber;

use crate::controllers::user;

mod controllers;
mod database;
mod models;

#[tokio::main]
#[tracing::instrument(ret)]
async fn main() {
    tracing_subscriber::fmt::init();

    database::migrate().await;

    let router = Router::new()
        .route("/", get(root))
        .route("/users", get(all_users));

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct ApiRoot {
    version: String,
}

async fn root() -> impl IntoResponse {
    Json(ApiRoot {
        version: String::from("0.1.0"),
    })
}

async fn all_users() -> impl IntoResponse {
    match user::get_all_users().await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(err) => {
            error!("{}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                StatusCode::INTERNAL_SERVER_ERROR.to_string(),
            )
                .into_response()
        }
    }
}
