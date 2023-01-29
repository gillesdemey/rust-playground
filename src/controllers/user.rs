use sqlx::{Connection, SqliteConnection};

use crate::models::User;

pub async fn get_all_users() -> Result<Vec<User>, sqlx::Error> {
    let mut conn = SqliteConnection::connect("db/playground.db").await.unwrap();

    return sqlx::query_as("SELECT * FROM users")
        .fetch_all(&mut conn)
        .await;
}
