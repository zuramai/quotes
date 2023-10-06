use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub tag: String
}

pub struct QuoteTag {
    pub quote_id: i32,
    pub tag_id: i32,
    pub tag_name: Option<String>
}