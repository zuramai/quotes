use std::sync::Arc;

use chrono::Utc;
use sqlx::{QueryBuilder, Row};
use tracing::info;
use slug;

use crate::services::quote::model::quote_author::QuoteAuthor;
use crate::services::quote::model::quote_tag::Tag;
use crate::{services::quote::{schema::{CreateQuoteRequest, CreateAuthorRequest}, model::quote::Quote}, db::DB};

use super::QuoteRepository;

impl QuoteRepository {
    pub async fn insert_author(&self, db: Arc<DB>, author: CreateAuthorRequest) -> Result<QuoteAuthor, crate::error::Error> {
        let q = sqlx::query_as!(QuoteAuthor, "INSERT INTO quote_authors (name, slug) VALUES ($1, $2) ON CONFLICT (slug) DO UPDATE SET name = EXCLUDED.name RETURNING *", &author.name, slug::slugify(&author.name))
            .fetch_one(&db.conn)
            .await?;
        
        Ok(q)
    }
    pub async fn insert_quote(&self, db: Arc<DB>, quote: CreateQuoteRequest) -> Result<i32, crate::error::Error> {
        let q = sqlx::query!("INSERT INTO quotes (
            quote,
            author_id,
            created_by,
            likes_count)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            ",quote.quote,quote.author_id, 1, 0)
            .fetch_one(&db.conn)
            .await?;
        
        Ok(q.id)
    }

    pub async fn insert_tags(&self, db: Arc<DB>, tags: Vec<String>) -> Result<Vec<Tag>, crate::error::Error> {
        let mut builder = QueryBuilder::new("INSERT INTO tags (tag) ");
        builder.push_values(tags, |mut b, tag| {
            b.push_bind(tag);
        }).push("ON CONFLICT (tag) DO UPDATE SET tag = EXCLUDED.tag RETURNING *");
        let query = builder.build();
        let result = query.fetch_all(&db.conn).await?;
        
        let mut inserted_tags: Vec<Tag> = vec![];

        for row in result {
            inserted_tags.push(Tag {
                id: row.try_get("id")?,
                tag: row.try_get("tag")?
            });
        }

        Ok(inserted_tags)
    }
 
    pub async fn insert_quote_tags(&self, db: Arc<DB>, quote_id: i32, tags: Vec<String>) -> Result<(), crate::error::Error> {
        // Insert uncreated tags
        let inserted_tags = self.insert_tags(db.clone(), tags).await?;

        let mut builder = QueryBuilder::new("INSERT INTO quote_tags (quote_id, tag_id) ");
        builder.push_values(inserted_tags, |mut b, tag| {
            b.push_bind(quote_id).push_bind(tag.id);
        });
        let query = builder.build();
        query.execute(&db.conn).await?;
        
        Ok(())
    }
}