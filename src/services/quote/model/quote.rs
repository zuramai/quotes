use chrono::{DateTime, NaiveDateTime};

use crate::services::user::{model::User, schema::UserResponse};

use super::quote_author::QuoteAuthor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub id: i32,
    pub quote: String,
    pub tags: Vec<String>,
    pub author: QuoteAuthor,
    pub created_by: UserResponse,
    pub likes_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}