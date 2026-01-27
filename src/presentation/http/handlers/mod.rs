pub mod health_handlers;
pub mod user_handlers;
pub mod v1;

// Re-export per facilitare l'uso
pub use health_handlers::*;
pub use user_handlers::*;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::domain::DomainError;

/// Wrapper per gli errori dell'applicazione
/// Converte DomainError in risposte HTTP appropriate
pub struct AppError(DomainError);

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        AppError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self.0 {
            DomainError::InvalidEmail(ref msg) => {
                tracing::warn!(error = %msg, "Invalid email");
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            DomainError::UserNotFound(ref msg) => {
                tracing::warn!(error = %msg, "User not found");
                (StatusCode::NOT_FOUND, msg.clone())
            }
            DomainError::UserAlreadyExists(ref msg) => {
                tracing::warn!(error = %msg, "User already exists");
                (StatusCode::CONFLICT, msg.clone())
            }
            DomainError::InvalidUserName(ref msg) => {
                tracing::warn!(error = %msg, "Invalid user name");
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            DomainError::ValidationError(ref msg) => {
                tracing::warn!(error = %msg, "Validation error");
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            DomainError::RepositoryError(ref msg) => {
                tracing::error!(error = %msg, "Repository error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(serde_json::json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
