use axum::{response::IntoResponse, http::StatusCode};

use crate::utils::response::ApiResponse;


#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    PermissionDenied(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error("{0}")]
    AlreadyExists(String),

    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("duplicate key error: {0}")]
    MongoErrorKind(mongodb::error::ErrorKind),

    #[error("duplicate key error: {0}")]
    MongoDuplicateError(mongodb::error::Error),

    #[error("error during mongodb query: {0}")]
    MongoQueryError(mongodb::error::Error),

    #[error("error serializing BSON")]
    MongoSerializeBsonError(#[from] mongodb::bson::ser::Error),

    #[error("validation error")]
    MongoDataError(#[from] mongodb::bson::document::ValueAccessError),
    
    #[error("invalid ID: {0}")]
    InvalidIDError(String),

    #[error("Quote with ID: {0} not found")]
    NotFoundError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status_code = match self {
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::PermissionDenied(_) => StatusCode::UNAUTHORIZED,
            Error::AlreadyExists(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        };
        ApiResponse::<String>::error(self.clone().to_string(), Some(status_code)).into_response()
    }
}