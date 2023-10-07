use std::sync::Arc;

use chrono::Utc;
use sqlx::{QueryBuilder, Row};
use tracing::info;
use slug::{self, slugify};

use crate::services::quote::model::quote_author::QuoteAuthor;
use crate::services::quote::model::quote_tag::Tag;
use crate::{services::quote::{schema::{CreateQuoteRequest, CreateAuthorRequest}, model::quote::Quote}, db::DB};

use super::QuoteRepository;

impl QuoteRepository {
    pub async fn insert_author(&self,  author: CreateAuthorRequest) -> Result<QuoteAuthor, crate::error::Error> {
        let q = sqlx::query_as!(QuoteAuthor, "INSERT INTO quote_authors (name, slug, updated_at) VALUES ($1, $2, NOW()) ON CONFLICT (slug) DO UPDATE SET name = EXCLUDED.name RETURNING *", 
            &author.name, 
            slug::slugify(&author.name)
        )
            .fetch_one(&self.db.conn)
            .await?;
        Ok(q)
    }
    pub async fn upsert_authors(&self,  authors: Vec<String>) -> Result<(), crate::error::Error> {
        let mut builder = QueryBuilder::new("INSERT INTO quote_authors (name, slug)");
        let mut authors = authors;

        authors.dedup_by(|a,b|  slugify(a) == slugify(b));
        
        builder.push_values(authors, |mut b, t| {
            b.push_bind(t.clone()).push_bind(slugify(&t));
        }).push("ON CONFLICT (slug) DO NOTHING");


        builder.build()
            .execute(&self.db.conn)
            .await?;
        Ok(())
    }
    pub async fn insert_quote(&self, quote: CreateQuoteRequest) -> Result<i32, crate::error::Error> {
        let q = sqlx::query!("INSERT INTO quotes (
            quote,
            author_id,
            created_by,
            likes_count)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            ",quote.quote,quote.author_id, 1, 0)
            .fetch_one(&self.db.conn)
            .await?;
        
        Ok(q.id)
    }

    pub async fn insert_tags(&self, tags: Vec<String>) -> Result<Vec<Tag>, crate::error::Error> {
        let mut builder = QueryBuilder::new("INSERT INTO tags (tag) ");
        builder.push_values(tags, |mut b, tag| {
            b.push_bind(tag);
        }).push("ON CONFLICT (tag) DO UPDATE SET tag = EXCLUDED.tag RETURNING *");
        let query = builder.build();
        let result = query.fetch_all(&self.db.conn).await?;
        
        let mut inserted_tags: Vec<Tag> = vec![];

        for row in result {
            inserted_tags.push(Tag {
                id: row.try_get("id")?,
                tag: row.try_get("tag")?
            });
        }

        Ok(inserted_tags)
    }
 
    pub async fn insert_quote_tags(&self, quote_id: i32, tags: Vec<String>) -> Result<(), crate::error::Error> {
        // Insert uncreated tags
        let inserted_tags = self.insert_tags(tags).await?;

        let mut builder = QueryBuilder::new("INSERT INTO quote_tags (quote_id, tag_id) ");
        builder.push_values(inserted_tags, |mut b, tag| {
            b.push_bind(quote_id).push_bind(tag.id);
        });
        let query = builder.build();
        query.execute(&self.db.conn).await?;
        
        Ok(())
    }
}