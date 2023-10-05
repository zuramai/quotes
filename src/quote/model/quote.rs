use chrono::DateTime;

use super::quote_author::Author;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub id: i32,
    pub quote: String,
    pub tags: Vec<String>,
    pub author: Author,
    pub likes_count: i32,
    pub created_at: Option<DateTime<chrono::Local>>,
    pub updated_at: Option<DateTime<chrono::Local>>,
}