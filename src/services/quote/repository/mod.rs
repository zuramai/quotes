use std::sync::Arc;

use crate::db::DB;

pub mod insert;
pub mod get;

pub struct QuoteRepository {
    pub db: Arc<DB>
}

impl QuoteRepository {
    pub fn new(db: Arc<DB>) -> Self {
        QuoteRepository { db }
    }
}