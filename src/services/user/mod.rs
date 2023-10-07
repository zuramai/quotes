use std::sync::Arc;

use axum::{Router, routing, Json, response::IntoResponse, extract::State, http::{StatusCode, HeaderMap}};
use serde::Serialize;

use crate::{context::ServerContext, error::Error, db::DB, utils::response::ApiResponse};

use self::{repository::UserRepository, schema::{LoginRequest, RegisterRequest, AuthResponse}};

pub mod model;
pub mod repository;
pub mod schema;

pub struct Service {
    pub repo: UserRepository
}

impl Service {
    pub fn new(db: Arc<DB>) -> Self {
        Service {
            repo: UserRepository { db }
        }
    }
}

pub fn router() -> Router<Arc<ServerContext>> {
    Router::new()
        .route("/logout", routing::post(logout))
        .route("/login", routing::post(login))
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

    // find user by username
    let find_user = server_context.user_service.repo.find_user_by_username(&body.username).await;

    let err = Err(Error::Unauthorized("Invalid credentials".into()));
    if find_user.is_err() {
        // User is exists
        return err;
    }

    // Check password
    let user = find_user.unwrap();
    if user.password.ne(&body.password) {
        return err;
    }

    // Create token
    let token = uuid::Uuid::new_v4().to_string();
    server_context.0.user_service.repo.update_user_token(user.id, &token).await?;

    Ok(ApiResponse::success("Login success".into(), Some(AuthResponse {
        token
    }), None))
}

pub async fn logout(
    headers: HeaderMap,
    server_context: State<Arc<ServerContext>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, Error> {
    // Get user id from token
    let err = Error::Unauthorized("Invalid token".into());
    let authorization = headers.get("Authorization");
    if authorization.is_none() {
        return Err(err); 
    }
    let token = &authorization.unwrap().to_str().unwrap().to_string()[7..].to_string();

    let user = server_context.0.user_service.repo.find_user_by_token(token).await.map_err(|_| err)?;

    // Remove the token
    server_context.0.user_service.repo.update_user_token(user.id, &"".into()).await?;

    Ok(ApiResponse::<String>::success("Loguout success".into(), None, None))
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
