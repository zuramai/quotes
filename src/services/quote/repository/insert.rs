use std::sync::Arc;

use chrono::Utc;
use tracing::info;
use slug;

use crate::services::quote::model::quote_author::QuoteAuthor;
use crate::{services::quote::{schema::{CreateQuoteRequest, CreateAuthorRequest}, model::quote::Quote}, db::DB};

use super::Repository;

impl Repository {
    pub async fn insert_author(&self, db: Arc<DB>, author: CreateAuthorRequest) -> Result<QuoteAuthor, crate::error::Error> {
        let q = sqlx::query_as!(QuoteAuthor, "INSERT INTO quote_authors (name, slug) VALUES ($1, $2) RETURNING *", &author.name, slug::slugify(&author.name))
            .fetch_one(&db.conn)
            .await?;
        
        Ok(q)
    }
    pub async fn insert_quote(&self, db: Arc<DB>, quote: CreateQuoteRequest) -> Result<(), crate::error::Error> {
        let q = sqlx::query!("INSERT INTO quotes (
            quote,
            author_id,
            created_by,
            likes_count)
            VALUES ($1, $2, $3, $4)
            ",quote.quote,quote.author_id, 1, 0)
            .execute(&db.conn)
            .await?;
        
        info!("DB: Create new quote. {q:?}");
        
        Ok(())
    }
}