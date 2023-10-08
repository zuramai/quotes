use std::{sync::Arc, io::BufReader, fs::File, path::PathBuf};

use slug::slugify;
use tracing::info;
use serde::Deserialize;

use crate::{db::DB, services::{user::repository::UserRepository, quote::{repository::QuoteRepository, model::quote::Quote}}, error::Error};

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
    pub fn new(user_repository: UserRepository, quote_repository: QuoteRepository) -> Self {
        Self {
            quote_repository, user_repository
        }
    }
    pub async fn seed(&self) -> Result<(), Error> {
        info!("Seeding: users");
        self.seed_users().await?;
        info!("Seeding: quotes");
        self.seed_quotes().await?;
        Ok(())
    }
    
    pub async fn seed_quotes(&self) -> Result<(), Error> {
        // Get quotes from json file
        let filepath = PathBuf::from("./src/database/english_quotes.json");
        let file = match File::open(filepath.canonicalize().unwrap()) {
            Err(e) => {
                panic!("File english quotes json not found");
            },
            Ok(o) => o
        };
        let buf = BufReader::new(file);
        let q: QuoteJson = serde_json::from_reader(buf).unwrap();

        // Upsert authors first
        let authors: Vec<String> = q.quotes.iter().map(|q| q.source.clone()).collect();
        self.quote_repository.upsert_authors(authors).await?;
        
        let mut builder = sqlx::QueryBuilder::new("INSERT INTO quotes (quote, created_by, author_id, likes_count)");
        
        // Get all authors
        let all_authors = self.quote_repository.get_all_authors().await?;

        let quotes: Vec<&QuoteItemJSON> = q.quotes.iter().filter(|quote| quote.length < 120).collect();

        let query = builder.push_values(quotes, |mut b, quote| {
            // Find author id
            let author = all_authors.iter().find(|author| author.slug == slugify(&quote.source)).unwrap();
            b.push_bind(quote.text.clone()).push_bind(1).push_bind(author.id).push_bind(0);
        }).build();

        match query.execute(&self.quote_repository.db.conn).await {
            Err(e) => tracing::error!("Error inserting quotes seeder, {:?}", e),
            Ok(_) => tracing::info!("Quotes seeding succeed")
        };
        Ok(())
    }

    pub async fn seed_users(&self) -> Result<(), Error> {
        match self.user_repository.insert_user("admin".into(), "admin".into()).await {
            Err(_) => tracing::error!("Error inserting user seeder"),
            Ok(_) => tracing::info!("User seeding succeed")
        };
        Ok(())
    }
}