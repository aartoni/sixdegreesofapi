use std::{env, sync::Arc};

use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::get,
};
use sixdegreesofapi::{AppState, DatabaseBuilder, routes::paths, util::shutdown_signal};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(feature = "dotenvy")]
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let port = env::var("PORT")?;
    let url = format!("0.0.0.0:{port}");
    let db = DatabaseBuilder::from_env()?.build().await?;
    let shared_state = Arc::new(AppState { db });

    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST])
        .allow_origin(env::var("ORIGIN")?.parse::<HeaderValue>().unwrap());

    let app = Router::new()
        .route("/", get(async || "Hello, World!"))
        .route("/paths", get(paths))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer)
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
