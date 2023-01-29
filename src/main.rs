use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tracing::error;
use tracing_subscriber;

use crate::controllers::user;

mod controllers;
mod database;
mod models;

#[tokio::main]
async fn main() {
    // start tracing subscriber
    tracing_subscriber::fmt::init();

    // create DB connection pool
    let pool = SqlitePool::connect("db/playground.db")
        .await
        .expect("cannot connect to DB");

    database::migrate(&pool).await;

    let server = Router::new()
        .route("/", get(root))
        .route("/users", get(all_users))
        // add pool to the app state
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    axum::Server::bind(&addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct RootAPIResponse {
    author: String,
    version: String,
}

async fn root() -> impl IntoResponse {
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or("0.0.0".to_string());

    Json(RootAPIResponse {
        author: "Gilles De Mey".to_string(),
        version,
    })
}

async fn all_users(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match user::get_all_users(&pool).await {
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
