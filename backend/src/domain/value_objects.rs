use super::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

// ============================================================================
// Email Value Object
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: impl Into<String>) -> DomainResult<Self> {
        let email = email.into();
        
        if !email.contains('@') || email.len() < 3 {
            return Err(DomainError::InvalidEmail(email));
        }

        Ok(Self(email))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================================================
// Password Value Object
// ============================================================================

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: impl Into<String>) -> DomainResult<Self> {
        let password = password.into();
        
        if password.len() < 8 {
            return Err(DomainError::InvalidPassword(
                "La password deve contenere almeno 8 caratteri".to_string()
            ));
        }

        Ok(Self(password))
    }

    /// In produzione, usare bcrypt o argon2
    pub fn hash(&self) -> String {
        format!("hashed_{}", self.0)
    }

    pub fn verify(&self, hash: &str) -> bool {
        hash == format!("hashed_{}", self.0)
    }
}
