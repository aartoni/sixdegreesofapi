use std::{env, sync::Arc};

use axum::{Router, routing::get};
use sixdegreesofapi::{AppState, DatabaseBuilder, routes::paths};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
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
        .route("/paths", get(paths))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
