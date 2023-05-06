use axum::{Router, routing::{Route, self}, response::IntoResponse};
use tracing_subscriber::prelude::*;

use crate::{error::Error, user, utils::response::ApiResponse};

use super::quote;

pub async fn app() -> Result<Router, Error> {
    let api_routes = Router::new()
        .route("/healthchecker", routing::get(healthchecker))
        .merge(quote::router())
        .merge(user::router());
    let app = Router::new()
        .nest_service("/api", api_routes);
    Ok(app)
}

pub async fn healthchecker() -> impl IntoResponse {
    ApiResponse::<String>::success("I'm healthy!".into(), None, None)
        .send()
}