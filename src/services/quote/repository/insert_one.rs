use std::sync::Arc;

use chrono::Utc;


use crate::{services::quote::{schema::CreateQuoteRequest, model::quote::Quote}, db::DB};

use super::Repository;

impl Repository {
    pub async fn insert_one(&self, db: Arc<DB>, quote: CreateQuoteRequest) -> Result<Quote, crate::error::Error> {
        unimplemented!()
    }
}