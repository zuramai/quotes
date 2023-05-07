use std::sync::Arc;

use axum::{Router, routing, Json, response::IntoResponse, extract::{State, Query}, http::StatusCode};
use serde::Serialize;

use crate::{context::ServerContext, utils::response::ApiResponse, quote::model::Quote};

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
            ApiResponse::<String>::error("Internal Server Error".into(), None).send()
        });
    if let Err(e) = quotes {
        return e;
    }

    let response = ApiResponse::<Vec<Quote>>::success(
                "Success get quotes".to_string(), 
                Some(quotes.unwrap()), 
                None
        );
    response.send()
}

pub async fn store(
    server_context: State<Arc<ServerContext>>
) -> impl IntoResponse {
    let response = Json(serde_json::json!({}));
    response
}