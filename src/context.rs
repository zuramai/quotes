use std::sync::Arc;

use crate::{quote, db::DB, config::Config};

pub struct ServerContext {
    pub db: Arc<DB>,
    pub config: Arc<Config>,
    pub quote_service: Arc<quote::Service>
    
    // Add other services
}