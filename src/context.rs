use std::sync::Arc;

use crate::quote;

pub struct ServerContext {
    pub quote_service: Arc<quote::Service>
    
    // Add other services
}