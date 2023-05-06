use std::net::SocketAddr;

use axum::{Router, routing::{Route, self}, response::IntoResponse};
use mongodb::Database;
use tracing_subscriber::prelude::*;

use crate::{error::Error, user, utils::response::ApiResponse, config::Config, db::DB};

use super::quote;

pub async fn init() -> Result<Router, Error> {
    let api_routes = Router::new()
        .route("/healthchecker", routing::get(healthchecker))
        .merge(quote::router())
        .merge(user::router());
    let app = Router::new()
        .nest_service("/api", api_routes);
    Ok(app)
}

pub async fn serve(host: SocketAddr, config: Config, db: DB) -> Result<(), super::error::Error> {
    let app = init().await?;

    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn healthchecker() -> impl IntoResponse {
    ApiResponse::<String>::success("I'm healthy!".into(), None, None)
        .send()
}