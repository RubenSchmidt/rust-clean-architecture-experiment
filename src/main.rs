use std::{sync::Arc, time::Duration};

use app::AppImpl;
use dotenv::dotenv;

mod app;
mod domain;
mod ports;
mod postgres_repository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let level = match std::env::var("LOG_LEVEL").unwrap_or_else(|_| "DEBUG".to_string()) {
        ref s if s == "DEBUG" => tracing::Level::DEBUG,
        ref s if s == "INFO" => tracing::Level::INFO,
        ref s if s == "WARN" => tracing::Level::WARN,
        ref s if s == "ERROR" => tracing::Level::ERROR,
        _ => tracing::Level::ERROR,
    };

    //tracing
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();

    let db_connection_str = std::env::var("DATABASE_URL")?.to_string();
    //pool with 3 sec timeout

    let pool = sqlx::postgres::PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await?;

    // check args if we are migrating or running the server
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "migrate" {
            sqlx::migrate!().run(&pool).await?;
            return Ok(());
        }
    }

    let repo = postgres_repository::PgRepo::new(pool);
    let app_state = Arc::new(AppImpl::new(Box::new(repo)));
    let router = ports::http::router(app_state);

    let adr = std::env::var("LISTEN_ADDR")?.to_string();
    axum::Server::bind(&adr.parse()?)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}
