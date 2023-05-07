use chrono::{DateTime, Utc};
use mongodb::bson::{bson, oid::ObjectId};

use crate::{quote::model::Quote, db::DB, error::Error};

use super::Repository;

impl Repository {
    pub async fn insert_one(db: DB, quote: String, author: String) -> Result<Quote, Error> {
        let quote =  Quote {
            id: ObjectId::new(),
            author,
            created_at: Utc::now(),
            updated_at:  Utc::now(),
            quote,
            tags: vec!(String::from("arst"))
        };
        
        db.conn.collection::<Quote>("quotes").insert_one(&quote, None).await?;

        Ok(quote)
    }
}