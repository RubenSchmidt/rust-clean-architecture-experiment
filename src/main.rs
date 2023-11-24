use std::{sync::Arc, time::Duration};

use app::AppImpl;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

mod app;
mod domain;
mod ports;
mod postgres_repository;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let level = match std::env::var("LOG_LEVEL").unwrap_or_else(|_| "DEBUG".to_string()) {
        ref s if s == "DEBUG" => tracing::Level::DEBUG,
        ref s if s == "INFO" => tracing::Level::INFO,
        ref s if s == "WARN" => tracing::Level::WARN,
        ref s if s == "ERROR" => tracing::Level::ERROR,
        _ => tracing::Level::ERROR,
    };

    //tracing
    tracing_subscriber::fmt().with_max_level(level).init();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap().to_string();
    // set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    sqlx::migrate!().run(&pool).await.unwrap();

    let repo = postgres_repository::PgRepo::new(pool);

    let app_state = Arc::new(AppImpl::new(Box::new(repo)));

    let router = ports::http::router(app_state);

    let port = std::env::var("PORT").unwrap().to_string();
    let adr = format!("127.0.0.1:{}", port);

    axum::Server::bind(&adr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
