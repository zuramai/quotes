use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    id: ObjectId,
    name: String,
    username: String,
    password: String,
}