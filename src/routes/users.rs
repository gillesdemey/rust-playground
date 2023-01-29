use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sqlx::SqlitePool;
use tracing::error;

use crate::controllers;

pub async fn get_all(State(pool): State<SqlitePool>) -> impl IntoResponse {
    match controllers::user::get_all_users(&pool).await {
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
