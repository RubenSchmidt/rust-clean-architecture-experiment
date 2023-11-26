use std::{sync::Arc, str::FromStr};

use app::AppImpl;
use dotenv::dotenv;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, ConnectOptions};

mod app;
mod domain;
mod ports;
mod sqlite_repository;
 
#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let db_connection_str  = std::env::var("DATABASE_URL")?.to_string(); 

    // check args if we are migrating or running the server
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        if args[1] == "migrate" {
            let mut conn = SqliteConnectOptions::from_str(&db_connection_str)?
                .create_if_missing(true)
                .connect()
                .await?;

            sqlx::migrate!().run(&mut conn).await.unwrap();
            return Ok(());
        }
    } 

    // set up connection pool 
   let pool = SqlitePoolOptions::new()
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");
    
    let repo = sqlite_repository::SqliteRepo::new(pool);

    let app_state = Arc::new(AppImpl::new(Box::new(repo)));

    let router = ports::http::router(app_state);

    let adr = std::env::var("LISTEN_ADDR").unwrap().to_string();
    axum::Server::bind(&adr.parse().unwrap())
        .serve(router.into_make_service())
        .await?;
    Ok(())

}
