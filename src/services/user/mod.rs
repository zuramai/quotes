use std::sync::Arc;

use axum::{Router, routing, Json, response::IntoResponse};
use serde::Serialize;

use crate::context::ServerContext;

use self::repository::Repository;

pub mod model;
pub mod repository;

pub struct Service {
    repo: Repository
}

pub fn router() -> Router<Arc<ServerContext>> {
    Router::new()
        .route("/users", routing::get(index))
        .route("/users", routing::post(store))
}

#[derive(Serialize)]
pub struct QuoteList {

}

pub async fn index(
) -> impl IntoResponse {
    let response = Json(serde_json::json!({
        "message": "User list"
    }));
    response
}

pub async fn store(
) -> impl IntoResponse {
    let response = Json(serde_json::json!(QuoteList {}));
    response
}