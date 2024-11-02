use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use minijinja::Error as MinijinjaError;
// use sqlx::error::Error as SqlxError;
use std::{io::Error as IOError, string::FromUtf8Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),
    // #[error("Database error: {0}")]
    // DatabaseError(#[from] SqlxError),
    #[error("Template error: {0}")]
    TemplateError(#[from] MinijinjaError),

    #[error("IO error: {0}")]
    IOError(#[from] IOError),

    #[error("Error convert from UTF-8: {0}")]
    Utf8Error(#[from] FromUtf8Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            // Self::DatabaseError(e) => {
            //     tracing::error!("Database error: {:?}", e);
            //     (
            //         StatusCode::INTERNAL_SERVER_ERROR,
            //         "Internal server error".to_string(),
            //     )
            // }
            Self::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };
        (status, error_message).into_response()
    }
}
