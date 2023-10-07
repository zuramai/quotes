use std::{sync::Arc, io::BufReader, fs::File};

use tracing::info;
use serde::Deserialize;

use crate::{db::DB, services::{user::repository::UserRepository, quote::{repository::QuoteRepository, model::quote::Quote}}};

pub struct Seeder {
    pub user_repository: UserRepository,
    pub quote_repository: QuoteRepository
}

#[derive(Deserialize)]
struct QuoteItemJSON {
    pub text: String,
    pub source: String,
    pub length: i32,
    pub id: i32,
}

#[derive(Deserialize)]
struct QuoteJson {
    pub language: String,
    pub groups: Vec<Vec<i32>>,
    pub quotes: Vec<QuoteItemJSON>
}

impl Seeder {
    pub async fn seed(&self) {
        info!("Seeding: users");
        self.seed_users().await;
        info!("Seeding: quotes");
        self.seed_quotes().await;
    }
    
    pub async fn seed_quotes(&self) {
        // Get quotes from json file
        let mut file = match File::open("english_quotes.json") {
            Err(e) => {
                panic!("File english quotes json not found");
            },
            Ok(o) => o
        };
        let buf = BufReader::new(file);
        let q: QuoteJson = serde_json::from_reader(buf).unwrap();
        
        let quotes: Vec<Quote> = vec![]; 
        let mut builder = sqlx::QueryBuilder::new("INSERT INTO quotes (quote, author_id, created_by)");
        let query = builder.push_values(q.quotes, |mut b, quote| {
            b.push_bind(quote.text).push_bind(quote.source).push_bind(1);
        }).build();

        match query.execute(&self.quote_repository.db.conn).await {
            Err(_) => tracing::error!("Error inserting quotes seeder"),
            Ok(_) => tracing::debug!("Quotes seeding succeed")
        };
    }

    pub async fn seed_users(&self) {
        match self.user_repository.insert_user("admin".into(), "admin".into()).await {
            Err(_) => tracing::error!("Error inserting user seeder"),
            Ok(_) => tracing::debug!("User seeding succeed")
        };
    }
}