use std::sync::Arc;

use axum::{Router, routing, response::IntoResponse, extract::{State, Query}, http::{StatusCode, request, Request}, body::Body, error_handling::HandleErrorLayer};
use serde::Serialize;
use tracing::info;

use crate::{context::ServerContext, utils::{response::ApiResponse, request::Json},  error::Error, services::quote::schema::QuoteList, db::DB};

use self::{schema::{CreateQuoteRequest, CreateAuthorRequest}, model::quote::Quote, repository::QuoteRepository};

pub mod model;
pub mod repository;
mod schema;

pub struct Service {
    pub repo: QuoteRepository
}

impl Service {
    pub fn new(db: Arc<DB>) -> Self {
        Service {
            repo: QuoteRepository {
                db
            }
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
    let mut quotes = server_context.0.quote_service.repo.get_quotes()
        .await
        .map_err(|err| {
            tracing::error!("Error: {}", err);
            ApiResponse::<Vec<Quote>>::error("Internal Server Error".into(), None)
        });
    if quotes.is_err() {
        return quotes.unwrap_err()
    }

    let response = ApiResponse::success(
            "Success get quotes".to_string(), 
            Some(quotes.unwrap()), 
            None
    );

    return response
}

pub async fn store(
    server_context: State<Arc<ServerContext>>,
    Json(mut body): Json<CreateQuoteRequest>,
) -> Result<impl IntoResponse,impl IntoResponse> {
    let data = Option::Some(2);

    // Insert author if not exists
    if body.author_id.is_none() && body.author_name.is_some() {
        // Create new author
        let author = server_context.quote_service.repo.insert_author(
            server_context.db.clone(), 
            CreateAuthorRequest {name: body.author_name.to_owned().unwrap()}
        ).await?;
        body.author_id = Some(author.id);
    }
    
    // Insert the quote
    let quote: Result<i32, Error> = server_context.0.quote_service.repo.insert_quote(server_context.db.clone(), body.clone()).await;
    if let Err(e) = quote {
        return Err(e);
    }

    // Insert the quote tags
    let tags = server_context.0.quote_service.repo.insert_quote_tags(server_context.db.clone(), quote.unwrap(), body.tags.unwrap()).await?;

    let response = ApiResponse::<Quote>::success(
                "Quote created".to_string(), 
                None, 
                None
            );
    return Ok(response);
}