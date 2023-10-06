use std::sync::Arc;

use axum::{Router, routing, Json, response::IntoResponse, extract::State, http::StatusCode};
use serde::Serialize;

use crate::{context::ServerContext, error::Error, db::DB, utils::response::ApiResponse};

use self::{repository::Repository, schema::{LoginRequest, RegisterRequest, AuthResponse}};

pub mod model;
pub mod repository;
pub mod schema;

pub struct Service {
    pub repo: Repository
}

impl Service {
    pub fn new(db: Arc<DB>) -> Self {
        Service {
            repo: Repository { db }
        }
    }
}

pub fn router() -> Router<Arc<ServerContext>> {
    Router::new()
        .route("/login", routing::get(login))
        .route("/register", routing::post(register))
        .route("/users", routing::get(index))
}

pub async fn index(
) -> impl IntoResponse {
    let response = Json(serde_json::json!({
        "message": "User list"
    }));
    response
}


pub async fn login(
    server_context: State<Arc<ServerContext>>,
    Json(body): Json<LoginRequest>
) -> Result<impl IntoResponse, Error> {
    
    Ok(())
}

pub async fn register(
    server_context: State<Arc<ServerContext>>,
    Json(body): Json<RegisterRequest>
) -> Result<impl IntoResponse, Error> {

    // Check username exists
    let find_user = server_context.user_service.repo.find_user_by_username(&body.username).await;

    if find_user.is_ok() {
        // User is exists
        return Err(Error::AlreadyExists("User already exists".into()));
    }

    // Proceed to insert user
    let user = server_context.0.user_service.repo.insert_user(body.username, body.password).await?;

    Ok(ApiResponse::success("Register success".into(), Some(AuthResponse {
        token: user.token
    }), Some(StatusCode::OK)))
}
