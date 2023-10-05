use chrono::DateTime;

use crate::services::user::model::User;

use super::quote_author::QuoteAuthor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub id: i32,
    pub quote: String,
    pub tags: Vec<String>,
    pub author: QuoteAuthor,
    pub created_by: User,
    pub likes_count: i32,
    pub created_at: DateTime<chrono::Local>,
    pub updated_at: DateTime<chrono::Local>,
}