use std::{env, sync::Arc};

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sixdegreesofapi::{AppState, DatabaseBuilder};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv()?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let port = env::var("PORT")?;
    let url = format!("0.0.0.0:{port}");
    let db = DatabaseBuilder::from_env()?.build().await?;
    let shared_state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/", get(async || "Hello, World!"))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
