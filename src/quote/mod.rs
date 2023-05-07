use std::sync::Arc;

use axum::{Router, routing, Json, response::IntoResponse, extract::{State, Query}, http::StatusCode};
use serde::Serialize;

use crate::{context::ServerContext, utils::response::ApiResponse};

use self::repository::Repository;

pub mod model;
pub mod repository;
mod schema;

pub struct Service {
    pub repo: Repository
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
    tracing::info!("Get all quotes request");
    let quotes = server_context.0.quote_service.repo.get_quotes(server_context.db.clone())
        .await
        .map_err(|err| {
            tracing::error!("Error: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR)
        });

    let response = ApiResponse::<String>::error("arst".into(), Some(StatusCode::INTERNAL_SERVER_ERROR));
    response.send()
}

pub async fn store(
    server_context: State<Arc<ServerContext>>
) -> impl IntoResponse {
    let response = Json(serde_json::json!({}));
    response
}