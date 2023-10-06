use std::sync::Arc;

use chrono::format;

use crate::{db::DB, error::Error, services::{quote::{schema::QuoteList, model::{quote::Quote, quote_author::QuoteAuthor, quote_tag::{Tag, QuoteTag}}}, user::{model::User, schema::UserResponse}}};

use super::Repository;

impl Repository {
    pub async fn get_quotes(&self, db: Arc<DB>) -> Result<QuoteList, Error> {
        tracing::info!("Fetching quotes from db..");

        let mut quotes: QuoteList = Vec::new();

        let result = sqlx::query!("
            SELECT 
                quotes.*,  
                quote_authors.name AS author_name,
                quote_authors.slug AS author_slug,
                users.username AS username
            FROM quotes
            JOIN quote_authors ON quote_authors.id = quotes.author_id
            JOIN users ON users.id = quotes.created_by
            ")
            .fetch_all(&db.conn)
            .await?;


        
        for quote in result {
            quotes.push(Quote {
                quote: quote.quote,
                id: quote.id,   
                author: QuoteAuthor { 
                    id: quote.author_id,
                    name: quote.author_name, 
                    slug: quote.author_slug
                },
                created_by: UserResponse {
                    id: quote.created_by,
                    username: quote.username.unwrap(),
                },
                tags: vec![],
                likes_count: quote.likes_count,
                created_at: quote.created_at,
                updated_at: quote.updated_at,                
            })
        }
        let all_tags = self.get_quotes_tags(db, quotes.iter().map(|q| q.id).collect()).await?;

        // Set quote tags
        quotes.iter_mut().for_each(|quote| {
            quote.tags = all_tags.iter().filter(|tag| tag.quote_id == quote.id).map(|tag| tag.tag_name.clone().unwrap()).collect();
        });

        tracing::info!("Quotes fetched!");
        Ok(quotes)
    }

    pub async fn get_quotes_tags(&self, db: Arc<DB>, quotes_id: Vec<i32>) -> Result<Vec<QuoteTag>, Error> {
        let result = sqlx::query!(
            "SELECT quote_tags.quote_id, tags.id, tags.tag FROM quote_tags 
            JOIN tags ON quote_tags.tag_id = tags.id
            WHERE quote_id = ANY($1)",
            &quotes_id[..]
        )
        .fetch_all(&db.conn)
        .await?;

        let mut tags = Vec::new();

        for tag in result {
            tags.push(QuoteTag {
                tag_name: Some(tag.tag),
                tag_id: tag.id,
                quote_id: tag.quote_id
            });
        }

        Ok(tags)
    }

    pub async fn get_quote_tags_by_quote_id(&self, db: Arc<DB>, quote_id: i32) -> Result<Vec<Tag>, Error> {
        let result = sqlx::query!(
            "SELECT quote_tags.id, tags.tag FROM quote_tags 
            JOIN tags ON quote_tags.tag_id = tags.id
            WHERE quote_id = $1",
            quote_id
        )
        .fetch_all(&db.conn)
        .await?;

        let mut tags = Vec::new();

        for tag in result {
            tags.push(Tag {
                tag: tag.tag
            });
        }

        Ok(tags)
    }
}