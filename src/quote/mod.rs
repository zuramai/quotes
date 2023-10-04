use std::sync::Arc;

use axum::{Router, routing, response::IntoResponse, extract::{State, Query}, http::{StatusCode, request, Request}, body::Body, error_handling::HandleErrorLayer};
use serde::Serialize;

use crate::{context::ServerContext, utils::{response::ApiResponse, request::Json}, quote::model::Quote, error::Error};

use self::{repository::Repository, schema::CreateQuoteRequest};

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
            ApiResponse::<Vec<Quote>>::error("Internal Server Error".into(), None)
        });
    if let Err(e) = quotes {
        return e;
    }
    tracing::info!("Quotes: {:#?}", quotes.as_ref().unwrap());
    let response = ApiResponse::<Vec<Quote>>::success(
                "Success get quotes".to_string(), 
                Some(quotes.unwrap()), 
                None
        );
    response
}

pub async fn store(
    server_context: State<Arc<ServerContext>>,
    Json(body): Json<CreateQuoteRequest>,
) -> Result<impl IntoResponse,impl IntoResponse> {
    let quote: Result<Quote, Error> = server_context.0.quote_service.repo.insert_one(server_context.db.clone(), body).await;
    if let Err(e) = quote {
        return Err(e);
    }

    let response = ApiResponse::<Quote>::success(
                "Quote created".to_string(), 
                Some(quote.unwrap()), 
                None
            );
    return Ok(response);
}