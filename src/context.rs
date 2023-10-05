use std::sync::Arc;

use crate::{db::DB, config::Config, services::quote};

pub struct ServerContext {
    pub db: Arc<DB>,
    pub config: Arc<Config>,
    pub quote_service: Arc<quote::Service>
    
    // Add other services
}