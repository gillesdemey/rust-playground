use axum::{routing::get, Router};
use sqlx::SqlitePool;
use std::{env, net::SocketAddr};

mod controllers;
mod database;
mod routes;

const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:7878";
const DEFAULT_DB_PATH: &str = "sqlite://db/playground.db";

#[tokio::main]
async fn main() {
    // start tracing subscriber
    tracing_subscriber::fmt::init();

    // create DB connection pool
    let db_url = env::var("DATABASE_URL").unwrap_or(DEFAULT_DB_PATH.to_string());
    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("cannot connect to database");

    // migrate the database
    database::migrate(&pool).await;

    // create and listen on the server
    let server = create_server(pool);
    run_server(server).await;
}

fn create_server(db_pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(routes::root::root))
        .route("/users", get(routes::users::get_all))
        .with_state(db_pool)
}

async fn run_server(server: Router) {
    let socket_addr: SocketAddr = env::var("LISTEN_ADDR")
        .unwrap_or(DEFAULT_LISTEN_ADDR.to_string())
        .parse()
        .expect("bad listen address");

    axum::Server::bind(&socket_addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
