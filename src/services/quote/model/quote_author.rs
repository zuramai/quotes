use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct QuoteAuthor {
    pub id: i32,
    pub slug: String,
    pub name: String
}

pub type QuoteAuthorList = Vec<QuoteAuthor>;
