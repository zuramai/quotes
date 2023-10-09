



use chrono::NaiveDateTime;

use crate::{error::Error, services::{quote::{schema::QuoteList, model::{quote::Quote, quote_author::QuoteAuthor, quote_tag::{Tag, QuoteTag}}}, user::{schema::UserResponse}}};
use serde::Deserialize;
use super::QuoteRepository;

#[derive(Deserialize, Debug)]
pub struct QuotePagination {
    pub size: Option<i32>,
    pub page: Option<i32>
}

#[derive(sqlx::FromRow)]
pub struct QueryResult {
    pub quote: String,
    pub id: i32,
    pub author_id: i32,
    pub author_name: String,
    pub author_slug: String,
    pub author_updated_at: NaiveDateTime,
    pub created_by: i32,
    pub user_created_at: NaiveDateTime,
    pub username: String,
    pub likes_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<&QueryResult> for Quote {
    fn from(value: &QueryResult) -> Self {
        Self {
            quote: value.quote.clone(),
            id: value.id,   
            author: QuoteAuthor { 
                id: value.author_id,
                name: value.author_name.clone(), 
                slug: value.author_slug.clone(),
                updated_at: value.author_updated_at,
            },
            created_by: UserResponse {
                id: value.created_by,
                created_at: value.user_created_at,
                username: value.username.clone(),
            },
            tags: vec![],
            likes_count: value.likes_count,
            created_at: value.created_at,
            updated_at: value.updated_at,    
        }
    }
}

impl QuoteRepository {
    pub async fn get_all_authors(&self) -> Result<Vec<QuoteAuthor>, Error> {
        let result = sqlx::query_as!(QuoteAuthor, "SELECT * FROM quote_authors")
            .fetch_all(&self.db.conn)
            .await?;

        Ok(result)
    }
    pub async fn get_quotes_by_user_id(&self, user_id: i32, pagination: Option<QuotePagination>) -> Result<QuoteList, Error> {
        return self.get_quotes(pagination, Some(vec![("created_by", user_id.to_string())])).await
    }
    pub async fn get_quotes(&self, pagination: Option<QuotePagination>, filters: Option<Vec<(&str, String)>>) -> Result<QuoteList, Error> {
        tracing::info!("Fetching quotes from db.. {:?}", pagination);

        let mut limit = String::new();
        if let Some(paginate) = pagination {
            let size = paginate.size.unwrap_or(15);
            let offset = paginate.page.unwrap_or(0) * size;
            limit += format!("LIMIT {} OFFSET {}", size, offset).as_str();
        }

        let mut wheres = String::new();
        if let Some(filter) = filters {
            wheres.push_str("WHERE ");
            filter.iter().for_each(|(k,v)| {
                wheres.push_str(format!("{} = {}", k,v).as_str());
            })
        }
        let result: Vec<QueryResult> = sqlx::query_as(
        format!(r#"
                SELECT 
                    quotes.*,  
                    quote_authors.name AS author_name,
                    quote_authors.slug AS author_slug,
                    quote_authors.updated_at AS author_updated_at,
                    users.username AS username,
                    users.created_at AS user_created_at
                FROM quotes
                JOIN quote_authors ON quote_authors.id = quotes.author_id
                JOIN users ON users.id = quotes.created_by
                {wheres}
                {limit}
            "#).as_str()
            )
            .fetch_all(&self.db.conn)
            .await?;
        let mut quotes: QuoteList = result.iter().map(|q| Quote::from(q)).collect();
       
        let all_tags = self.get_quotes_tags(quotes.iter().map(|q| q.id).collect()).await?;
        
        // Set quote tags
        quotes.iter_mut().for_each(|quote| {
            quote.tags = all_tags.iter().filter(|tag| tag.quote_id == quote.id).map(|tag| tag.tag_name.clone().unwrap()).collect();
        });

        tracing::info!("Quotes fetched!");
        Ok(quotes)
    }

    pub async fn get_quotes_tags(&self, quotes_id: Vec<i32>) -> Result<Vec<QuoteTag>, Error> {
        let result = sqlx::query!(
            "SELECT quote_tags.quote_id, tags.id, tags.tag FROM quote_tags 
            JOIN tags ON quote_tags.tag_id = tags.id
            WHERE quote_id = ANY($1)",
            &quotes_id[..]
        )
        .fetch_all(&self.db.conn)
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

    pub async fn get_quote_tags_by_quote_id(&self, quote_id: i32) -> Result<Vec<Tag>, Error> {
        let result = sqlx::query!(
            "SELECT quote_tags.id, tags.tag FROM quote_tags 
            JOIN tags ON quote_tags.tag_id = tags.id
            WHERE quote_id = $1",
            quote_id
        )
        .fetch_all(&self.db.conn)
        .await?;

        let mut tags = Vec::new();

        for tag in result {
            tags.push(Tag {
                id: tag.id,
                tag: tag.tag
            });
        }

        Ok(tags)
    }
}