use sqlx::SqlitePool;

use crate::models::User;

pub async fn get_all_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    return sqlx::query_as("SELECT * FROM users").fetch_all(pool).await;
}
