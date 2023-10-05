use std::sync::Arc;

use crate::{db::DB, error::Error, services::quote::schema::QuoteList};

use super::Repository;

impl Repository {
    pub async fn get_quotes(&self, db: Arc<DB>) -> Result<QuoteList, Error> {
        tracing::info!("Fetching quotes from db..");

        let mut quotes: QuoteList = Vec::new();

            

        tracing::info!("Quotes fetched!");
        Ok(quotes)
    }
}