use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing::{self}, response::IntoResponse, http::{header::{AUTHORIZATION, ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE}}, Extension};
use tower_http::cors::{CorsLayer, Any};
use crate::{error::Error, utils::response::ApiResponse, config::Config, db::DB, context::ServerContext, services::{quote, user}};
use tower::ServiceBuilder;


pub async fn init<S>(db: Arc<DB>, config: Config) -> Result<Router<S>, Error> {
    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any).allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCESS_CONTROL_ALLOW_ORIGIN]);
    
    let api_routes = Router::new()
        .merge(quote::router())
        .merge(user::router())
        .layer(ServiceBuilder::new().layer(cors));


    let db = db;
    let app = Router::new()
        .route("/", routing::get(welcome))
        .nest("/api", api_routes)
        .layer(
            ServiceBuilder::new()
        )
        .with_state(Arc::new(ServerContext {
            db: db.clone(),
            config: Arc::new(config),
            quote_service: Arc::new(quote::Service::new(db.clone())),
            user_service: Arc::new(user::Service::new(db.clone())),
        }));
    Ok(app)
}

pub async fn serve(host: SocketAddr, config: Config, db: Arc<DB>) -> Result<(), super::error::Error> {
    let app = init(db, config).await?;

    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub async fn welcome() -> impl IntoResponse {
    ApiResponse::<String>::success("Welcome to Quotes!".into(), None, None)
}
