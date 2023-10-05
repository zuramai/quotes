use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub id: i32,
    pub name: String
}

pub type Authors = Vec<Author>;
