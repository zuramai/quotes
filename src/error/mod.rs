use thiserror::Error;

#[derive(Error, Debug, Clone)]
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
}