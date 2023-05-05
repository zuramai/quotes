use axum::{Router, Json, response::IntoResponse, routing::get, http::{StatusCode, Response}};
use response::ApiResponse;

mod model;
mod response;

async fn health_checker_handler<T>() -> impl IntoResponse {
    ApiResponse::error("test".into(), Some(StatusCode::OK))
        .send()
        
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(health_checker_handler));
    println!("Server started at port 8000");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
        
}
