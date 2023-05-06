use std::net::{SocketAddr, IpAddr};

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

    let ip: IpAddr = config.app_host;
    let port: u16 = config.app_port;
    let host = SocketAddr::new(ip, port);
    
    let app = app().await?;
    tracing::info!("Server started at {}", host);

    axum::Server::bind(&host)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
        
}
