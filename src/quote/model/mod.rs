use chrono::{Utc, DateTime};
use mongodb::bson::{self,oid::ObjectId};
use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub quote: String,
    pub tags: Vec<String>,
    pub author: String,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String
}