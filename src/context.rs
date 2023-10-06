use std::sync::Arc;

use crate::{db::DB, config::Config, services::{quote, user}};

pub struct ServerContext {
    pub db: Arc<DB>,
    pub config: Arc<Config>,
    pub quote_service: Arc<quote::Service>,
    pub user_service: Arc<user::Service>
    
    // Add other services
}