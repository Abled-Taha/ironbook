use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("API key does not exist")]
    InvalidApiKey,

    #[error("Username already exists")]
    UsernameAlreadyExists,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid name")]
    InvalidName,

    #[error("Invalid owner email")]
    InvalidOwnerEmail,

    #[error("API key name already exists")]
    ApiKeyNameAlreadyExists,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

impl AppError {
    pub fn code(&self) -> u32 {
        match self {
            AppError::InvalidApiKey => 1001,
            AppError::UsernameAlreadyExists => 1002,
            AppError::EmailAlreadyExists => 1003,
            AppError::InvalidCredentials => 1004,
            AppError::InvalidName => 1005,
            AppError::InvalidOwnerEmail => 1006,
            AppError::ApiKeyNameAlreadyExists => 1007,
            AppError::DatabaseError(_) => 5000,
            AppError::Internal(_) => 5001,
        }
    }

    pub fn to_grpc_status(&self) -> Status {
        let grpc_code = match self {
            // Authentication failures
            AppError::InvalidApiKey | AppError::InvalidCredentials => tonic::Code::Unauthenticated,

            // Resource already exists
            AppError::UsernameAlreadyExists
            | AppError::EmailAlreadyExists
            | AppError::ApiKeyNameAlreadyExists => tonic::Code::AlreadyExists,

            // Invalid client-supplied input
            AppError::InvalidName | AppError::InvalidOwnerEmail => tonic::Code::InvalidArgument,

            // Server-side failures
            AppError::DatabaseError(_) | AppError::Internal(_) => tonic::Code::Internal,
        };

        Status::new(grpc_code, format!("[code: {}] {}", self.code(), self))
    }

    pub fn http_status(&self) -> StatusCode {
        match self {
            // Authentication failures
            AppError::InvalidApiKey | AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,

            // Resource already exists
            AppError::UsernameAlreadyExists
            | AppError::EmailAlreadyExists
            | AppError::ApiKeyNameAlreadyExists => StatusCode::CONFLICT,

            // Invalid client-supplied input
            AppError::InvalidName | AppError::InvalidOwnerEmail => StatusCode::BAD_REQUEST,

            // Server-side failures
            AppError::DatabaseError(_) | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.http_status();
        let body = Json(json!({
        "error": self.to_string(),
        "code": self.code()
        }));

        (status, body).into_response()
    }
}
