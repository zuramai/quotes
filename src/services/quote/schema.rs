use serde::{Serialize, Deserialize};

use super::model::quote::Quote;

pub type QuoteList = Vec<Quote>;

#[derive(Deserialize, Debug)]
pub struct CreateQuoteRequest {
    pub quote: String,
    pub tags: Option<Vec<String>>,
    pub author_id: i32,
}