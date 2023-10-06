use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub created_at: NaiveDateTime
}
#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String
}

