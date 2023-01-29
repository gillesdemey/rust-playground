use sqlx::SqlitePool;

pub async fn migrate(pool: &SqlitePool) {
    return sqlx::migrate!("db/migrations").run(pool).await.unwrap();
}
