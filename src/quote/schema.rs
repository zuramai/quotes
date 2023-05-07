use serde::{Serialize, Deserialize};

use super::model::Quote;

pub type QuoteList = Vec<Quote>;

#[derive(Serialize, Deserialize)]
pub struct CreateQuoteRequest {
    pub quote: String,
    pub tags: Vec<String>,
    pub author: String,
}