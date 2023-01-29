use axum::{routing::get, Router};
use sqlx::SqlitePool;
use std::{env, net::SocketAddr};
use tracing_subscriber;

mod controllers;
mod database;
mod routes;

#[tokio::main]
async fn main() {
    // start tracing subscriber
    tracing_subscriber::fmt::init();

    // create DB connection pool
    let db_url = env::var("DATABASE_URL").expect("no valid DATABASE_URL set");
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
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or("127.0.0.1:7878".to_string());
    let socket_addr: SocketAddr = listen_addr.parse().expect("bad listen address");

    axum::Server::bind(&socket_addr)
        .serve(server.into_make_service())
        .await
        .unwrap();
}
