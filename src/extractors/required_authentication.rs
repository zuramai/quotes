use std::sync::Arc;

use axum::{extract::{FromRequestParts, FromRef}, http::request::Parts, async_trait};

use tracing::info;

use crate::{error::Error, services::user::model::User, context::ServerContext};

pub struct RequiredAuthentication(pub User);

#[async_trait]
impl<S> FromRequestParts<S> for RequiredAuthentication 
where 
    S: Send + Sync,
    Arc<ServerContext>: FromRef<S>
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) ->  Result<Self, Self::Rejection> {
        info!("Get user info");
        let server_context = Arc::<ServerContext>::from_ref(state);
       
        // Get user id from token
        let err = Error::Unauthorized("Invalid token".into());
        let authorization = parts.headers.get("Authorization");
        if authorization.is_none() {
            return Err(err); 
        }
        let token = &authorization.unwrap().to_str().unwrap().to_string()[7..].to_string();
        
        let user = server_context.user_service.repo.find_user_by_token(token).await.map_err(|_| err)?;
        Ok(RequiredAuthentication(user))
    }
}