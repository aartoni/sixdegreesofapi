use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use neo4rs::{Path, query};
use serde::{Deserialize, Serialize};

use crate::state::AppState;

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
    source: String,
    target: String,
}

pub async fn paths(
    State(state): State<Arc<AppState>>,
    req: Query<PathRequest>,
) -> (StatusCode, Json<PathResponse>) {
    let query = query("MATCH path=allShortestPaths((:Key {fingerprint: $source})-[*]-(:Key {fingerprint: $target})) RETURN path")
        .param("source", req.source.as_str())
        .param("target", req.target.as_str());

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
                id: String::new(),
                title: fp,
                url: String::new(),
                description: String::new(),
                thumbnail: String::new(),
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
