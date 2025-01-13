mod db;
mod state;

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
pub use db::DatabaseBuilder;
pub use state::AppState;

pub struct Node {
    id: String,
    title: String,
    url: String,
    description: String,
    thumbnail: String,
}

#[derive(Serialize)]
pub struct ShortestPathResponse {
    paths: Vec<Vec<Node>>,
    source_friendly_name: String,
    target_friendly_name: String,
}

pub async fn paths(State(state): State<Arc<AppState>>) -> (StatusCode, Json<ShortestPathResponse>) {
    todo!()
}
