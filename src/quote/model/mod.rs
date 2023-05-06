use chrono::{Utc, DateTime};
use mongodb::bson::{self,oid::ObjectId};
use serde::{self, Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub quote: String,
    pub tags: Vec<String>,
    pub by: String,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub updated_at: DateTime<Utc>,
}