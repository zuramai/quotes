use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing::{Route, self}, response::IntoResponse};
use crate::{error::Error, user, utils::response::ApiResponse, config::Config, db::DB, context::ServerContext};
use super::quote;

pub async fn init<S>(db: DB, config: Config) -> Result<Router<S>, Error> {
    let api_routes = Router::new()
        .route("/healthchecker", routing::get(healthchecker))
        .merge(quote::router())
        .merge(user::router());

    let app = Router::new()
        .nest("/api", api_routes)
        .with_state(Arc::new(ServerContext {
            db: Arc::new(db),
            config: Arc::new(config),
            quote_service: Arc::new(quote::Service::new())
        }));
    Ok(app)
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
        .send()
}