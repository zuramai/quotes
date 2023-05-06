use axum::{Router, Json, response::IntoResponse, routing::get, http::{StatusCode, Response}};
use dotenv::dotenv;
use envconfig::Envconfig;
use quotes::{config, error};
use quotes::app::app;

#[tokio::main]


async fn main() -> Result<(), quotes::error::Error> {
    tracing_subscriber::fmt().init();
    dotenv().ok();
    
    let config = config::Config::init_from_env().map_err(|err| {
        tracing::error!("Failed to load environment variables {:?}",err);
        error::Error::Internal("Failed to load environment variables".into())
    })?;
    
    let app = app().await?;
    tracing::info!("Server started at port 8000");

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await;

    Ok(())
        
}
