use thiserror::Error;

/// Domain Errors - Errori del domain layer
#[derive(Error, Debug, Clone)]
pub enum DomainError {
    #[error("Invalid email format: {0}")]
    InvalidEmail(String),

    #[error("User not found with id: {0}")]
    UserNotFound(String),

    #[error("User with email {0} already exists")]
    UserAlreadyExists(String),

    #[error("Invalid user name: {0}")]
    InvalidUserName(String),

    #[error("Domain validation error: {0}")]
    ValidationError(String),

    #[error("Repository error: {0}")]
    RepositoryError(String),
}
