use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    username: String,
    password: String,
    created_at: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    name: String,
    password: String
}