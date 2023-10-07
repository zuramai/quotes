use std::sync::Arc;

use tracing::info;

use crate::{db::DB, error::Error};

use super::model::User;

pub struct UserRepository {
    pub db: Arc<DB>
}

impl UserRepository {
    pub async fn find_user_by_username(&self, username: &String) -> Result<User, Error> {
        let q = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1 LIMIT 1", username)
            .fetch_one(&self.db.conn)
            .await?;
        Ok(q)
    }
    pub async fn insert_user(&self, username: String, password: String) -> Result<User, Error> {
        let token = uuid::Uuid::new_v4().to_string();
        let q = sqlx::query_as!(User, "INSERT INTO users (username, password, token) VALUES ($1,$2,$3) RETURNING *", username, password, token)
            .fetch_one(&self.db.conn)
            .await?;
        Ok(q)
    }
    pub async fn update_user_token(&self, user_id: i32, token: &String) -> Result<(), Error> {
        sqlx::query!("UPDATE users SET token = $1 WHERE id = $2", token, user_id).execute(&self.db.conn).await?;
        Ok(())
    }
    pub async fn find_user_by_token(&self, token: &String) -> Result<User, Error> {
        let q = sqlx::query!("SELECT * FROM users WHERE token = $1", token).fetch_one(&self.db.conn).await?;
        let user = User {
            created_at: q.created_at,
            id: q.id,
            password: q.password,
            token: q.token,
            username: q.username,
        };
        Ok(user)
    }
}