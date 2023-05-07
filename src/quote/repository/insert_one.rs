use std::sync::Arc;

use chrono::Utc;
use mongodb::bson::oid::ObjectId;

use crate::{quote::{model::Quote, schema::CreateQuoteRequest}, db::DB, error::Error};

use super::Repository;

impl Repository {
    pub async fn insert_one(&self, db: Arc<DB>, quote: CreateQuoteRequest) -> Result<Quote, Error> {
        let quote =  Quote {
            id: ObjectId::new(),
            author: quote.author,
            created_at: Utc::now(),
            updated_at:  Utc::now(),
            quote: quote.quote,
            tags: quote.tags
        };
        
        db.conn.collection::<Quote>("quotes").insert_one(&quote, None).await?;

        Ok(quote)
    }
}