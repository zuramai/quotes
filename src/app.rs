use axum::{Router, routing::Route};
use tracing_subscriber::prelude::*;

use crate::{error::Error, user};

use super::quote;

pub async fn app() -> Result<Router, Error> {
    tracing_subscriber::fmt().init();
    let api_routes = Router::new()
        .merge(quote::router())
        .merge(user::router());
    let app = Router::new()
        .nest_service("/api", api_routes);
    Ok(app)
}