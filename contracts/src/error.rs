use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    NotFound,
    Unauthorized,
    Forbidden,
    ValidationError,
    InternalError,
    Conflict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: ErrorCode,
    pub message: String,
}

impl ApiError {
    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            code: ErrorCode::NotFound,
            message: message.into(),
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            code: ErrorCode::Unauthorized,
            message: message.into(),
        }
    }

    pub fn validation_error(message: impl Into<String>) -> Self {
        Self {
            code: ErrorCode::ValidationError,
            message: message.into(),
        }
    }

    pub fn internal_error(message: impl Into<String>) -> Self {
        Self {
            code: ErrorCode::InternalError,
            message: message.into(),
        }
    }

    pub fn conflict(message: impl Into<String>) -> Self {
        Self {
            code: ErrorCode::Conflict,
            message: message.into(),
        }
    }
}
