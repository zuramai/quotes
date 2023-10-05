use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing::{Route, self}, response::IntoResponse, error_handling::HandleErrorLayer, http::StatusCode};
use crate::{error::Error, utils::response::ApiResponse, config::Config, db::DB, context::ServerContext, services::{quote, user}};
use tower::{ServiceBuilder};


pub async fn init<S>(db: DB, config: Config) -> Result<Router<S>, Error> {
    let api_routes = Router::new()
        .merge(quote::router())
        .merge(user::router());

    let app = Router::new()
        .route("/healthchecker", routing::get(healthchecker))
        .route("/", routing::get(welcome))
        .nest("/api", api_routes)
        .layer(
            ServiceBuilder::new()
        )
        .with_state(Arc::new(ServerContext {
            db: Arc::new(db),
            config: Arc::new(config),
            quote_service: Arc::new(quote::Service::new())
        }));
    Ok(app)
}

pub async fn welcome() -> impl IntoResponse {
    ApiResponse::<String>::success("Welcome to Quotes!".into(), None, None)
}

pub async fn serve(host: SocketAddr, config: Config, db: DB) -> Result<(), super::error::Error> {
    let app = init(db, config).await?;

    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn healthchecker() -> impl IntoResponse {
    ApiResponse::<String>::success("I'm healthy!".into(), None, None)
}