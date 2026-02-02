use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Risorsa non trovata: {0}")]
    NotFound(String),

    #[error("Conflitto: {0}")]
    Conflict(String),

    #[error("Email non valida: {0}")]
    InvalidEmail(String),

    #[error("Password non valida: {0}")]
    InvalidPassword(String),

    #[error("Errore di storage: {0}")]
    StorageError(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
