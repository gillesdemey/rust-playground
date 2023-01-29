use sqlx::{Connection, SqliteConnection};

pub async fn migrate() {
    let mut conn = SqliteConnection::connect("db/playground.db").await.unwrap();
    return sqlx::migrate!("db/migrations")
        .run(&mut conn)
        .await
        .unwrap();
}
