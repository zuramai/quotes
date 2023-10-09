use std::sync::Arc;

use axum::{Router, routing, response::IntoResponse, extract::{State, Query}};
use crate::{context::ServerContext, utils::{response::ApiResponse, request::Json},  error::Error, db::DB, extractors::required_authentication::RequiredAuthentication};
use self::{schema::{CreateQuoteRequest, CreateAuthorRequest}, model::quote::Quote, repository::{QuoteRepository, get::QuotePagination}};
use serde::Deserialize;

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
        .route("/my-quotes", routing::get(my_quotes))
}

pub async fn index(
    query: Query<QuotePagination>,
    server_context: State<Arc<ServerContext>>
) -> impl IntoResponse {
    tracing::info!("Get all quotes request");
    let quotes = server_context.0.quote_service.repo.get_quotes(Some(query.0), None)
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
pub async fn my_quotes(
    query: Query<QuotePagination>,
    RequiredAuthentication(user): RequiredAuthentication,
    server_context: State<Arc<ServerContext>>
) -> impl IntoResponse {
    tracing::info!("Get my quotes request user_id: {}", user.id);
    let quotes = server_context.0.quote_service.repo.get_quotes_by_user_id(user.id, Some(query.0))
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
    RequiredAuthentication(user): RequiredAuthentication,
    Json(mut body): Json<CreateQuoteRequest>
) -> Result<impl IntoResponse,impl IntoResponse> {
    let _data = Option::Some(2);

    // Insert author if not exists
    if body.author_id.is_none() && body.author_name.is_some() {
        // Create new author
        let author = server_context.quote_service.repo.insert_author(
            CreateAuthorRequest {name: body.author_name.to_owned().unwrap()}
        ).await?;
        body.author_id = Some(author.id);
    }
    
    // Insert the quote
    let quote: Result<i32, Error> = server_context.0.quote_service.repo.insert_quote(body.clone(), user.id).await;
    if let Err(e) = quote {
        return Err(e);
    }

    // Insert the quote tags
    let _tags = server_context.0.quote_service.repo.insert_quote_tags(quote.unwrap(), body.tags.unwrap()).await?;

    let response = ApiResponse::<Quote>::success(
                "Quote created".to_string(), 
                None, 
                None
            );
    return Ok(response);
}