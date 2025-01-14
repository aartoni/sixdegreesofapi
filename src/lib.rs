mod db;
mod state;

use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
pub use db::DatabaseBuilder;
use neo4rs::query;
use serde::{Deserialize, Serialize};
pub use state::AppState;

#[derive(Serialize)]
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

#[derive(Deserialize)]
pub struct ShortestPathRequest {
    from: String,
    to: String,
}

pub async fn paths(
    State(state): State<Arc<AppState>>,
    req: Query<ShortestPathRequest>,
) -> (StatusCode, Json<ShortestPathResponse>) {
    let query = query("MATCH path=allShortestPaths((:Key {fingerprint: $from})-[*]-(:Key {fingerprint: $to})) RETURN path, length(path) as distance")
        .param("from", req.from.as_str())
        .param("to", req.to.as_str());

    let _results = state
        .db
        .execute(query)
        .await
        .expect("Error while reading paths from DB.");
    todo!()
}
