use std::sync::Arc;

use axum::{Router, routing, Json, response::IntoResponse, extract::{State, Query}};
use serde::Serialize;

use crate::context::ServerContext;

use self::repository::Repository;

pub mod model;
pub mod repository;
mod schema;

pub struct Service {
    repo: Repository
}

impl Service {
    pub fn new() -> Self {
        Service {
            repo: Repository {  }
        }
    }
}

pub fn router() -> Router<Arc<ServerContext>> {
    Router::new()
        .route("/quotes", routing::get(index).post(store))
}



pub async fn index(
    opts: Option<Query<String>>,
    server_context: State<Arc<ServerContext>>
) -> impl IntoResponse {
    let response = Json(serde_json::json!({}));
    response
}

pub async fn store(
    server_context: State<Arc<ServerContext>>
) -> impl IntoResponse {
    let response = Json(serde_json::json!({}));
    response
}