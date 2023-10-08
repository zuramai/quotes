use serde::{Deserialize};

use super::model::quote::Quote;

pub type QuoteList = Vec<Quote>;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateQuoteRequest {
    pub quote: String,
    pub tags: Option<Vec<String>>,
    pub author_id: Option<i32>,
    // Provided if no author_id
    pub author_name: Option<String> 
}

#[derive(Deserialize, Debug)]
pub struct CreateAuthorRequest {
    pub name: String
}

