mod db;
mod state;

use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
pub use db::DatabaseBuilder;
use neo4rs::{Path, query};
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
pub struct PathResponse {
    paths: Vec<Vec<Node>>,
    source_friendly_name: String,
    target_friendly_name: String,
}

#[derive(Deserialize)]
pub struct PathRequest {
    from: String,
    to: String,
}

pub async fn paths(
    State(state): State<Arc<AppState>>,
    req: Query<PathRequest>,
) -> (StatusCode, Json<PathResponse>) {
    let query = query("MATCH path=allShortestPaths((:Key {fingerprint: $from})-[*]-(:Key {fingerprint: $to})) RETURN path")
        .param("from", req.from.as_str())
        .param("to", req.to.as_str());

    let mut results = state
        .db
        .execute(query)
        .await
        .expect("Error while reading paths from DB.");

    let mut paths = Vec::new();

    while let Ok(Some(row)) = results.next().await {
        let path: Path = row.get("path").expect("Path not part of query results");
        let nodes = path
            .nodes()
            .into_iter()
            .flat_map(|n| n.get::<String>("fingerprint"))
            .map(|fp| Node {
                id: "".into(),
                title: fp,
                url: "".into(),
                description: "".into(),
                thumbnail: "".into(),
            })
            .collect();

        paths.push(nodes);
    }

    let response = PathResponse {
        paths,
        source_friendly_name: todo!("Transform into spaced hex"),
        target_friendly_name: todo!("Transform into spaced hex"),
    };
    (StatusCode::OK, response.into())
}
