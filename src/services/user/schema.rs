use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
}