use std::sync::Arc;

use crate::{db::DB, quote::{model::Quote, schema::QuoteList}, error::Error};

use super::Repository;
use futures::TryStreamExt;
use mongodb::{options::FindOptions, bson::doc};

impl Repository {
    pub async fn get_quotes(&self, db: Arc<DB>) -> Result<QuoteList, Error> {
        tracing::info!("Fetching quotes from db..");

        let find_options = FindOptions::builder().build();
        let mut cursor = db.conn.collection::<Quote>("quotes").find(doc! { }, find_options).await?;
        let mut quotes = Vec::new();
        while let Some(result) = cursor.try_next().await? {
            quotes.push(result);
        }
        tracing::info!("Quotes fetched!");
        Ok(quotes)
    }
}