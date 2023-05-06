use axum::{Router, routing, Json, response::IntoResponse};
use serde::Serialize;

use self::repository::Repository;

pub mod model;
pub mod repository;

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

pub fn router() -> Router {
    Router::new()
        .route("/quotes", routing::get(index))
        .route("/quotes", routing::post(store))
}

#[derive(Serialize)]
pub struct QuoteList {

}

pub async fn index(
) -> impl IntoResponse {
    let response = Json(serde_json::json!(QuoteList {}));
    response
}

pub async fn store(
) -> impl IntoResponse {
    let response = Json(serde_json::json!(QuoteList {}));
    response
}