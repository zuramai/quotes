use axum::{response::IntoResponse, http::StatusCode};

use crate::utils::response::ApiResponse;


#[derive(thiserror::Error, Debug)]
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

    #[error("{0}")]
    AxumError(#[from] axum::Error),

    #[error("Database Error: {0}")]
    PgError(#[from] sqlx::error::Error),

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
        ApiResponse::<String>::error(self.to_string(), Some(status_code)).into_response()
    }
}