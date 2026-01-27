use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

/// Name Value Object - Garantisce che il nome sia sempre valido
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Name(String);

impl Name {
    /// Crea un nuovo Name validando che non sia vuoto
    pub fn new(value: String) -> Result<Self, DomainError> {
        let trimmed = value.trim();
        
        if trimmed.is_empty() {
            return Err(DomainError::InvalidUserName(
                "Name cannot be empty".to_string(),
            ));
        }

        if trimmed.len() > 100 {
            return Err(DomainError::InvalidUserName(
                "Name cannot exceed 100 characters".to_string(),
            ));
        }

        // Validazione caratteri: alfanumerici, spazi, apostrofi e trattini
        if !trimmed.chars().all(|c| c.is_alphanumeric() || c.is_whitespace() || "'-".contains(c)) {
            return Err(DomainError::InvalidUserName(
                "Name contains invalid characters".to_string(),
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// Ritorna il valore del nome
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_name() {
        let name = Name::new("John Doe".to_string());
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), "John Doe");
    }

    #[test]
    fn test_empty_name() {
        let name = Name::new("".to_string());
        assert!(name.is_err());
    }

    #[test]
    fn test_whitespace_name() {
        let name = Name::new("   ".to_string());
        assert!(name.is_err());
    }

    #[test]
    fn test_name_with_leading_trailing_spaces() {
        let name = Name::new("  John Doe  ".to_string());
        assert!(name.is_ok());
        assert_eq!(name.unwrap().value(), "John Doe");
    }

    #[test]
    fn test_name_too_long() {
        let long_name = "a".repeat(101);
        let name = Name::new(long_name);
        assert!(name.is_err());
    }

    #[test]
    fn test_name_with_special_chars() {
        let name = Name::new("O'Brien-Smith".to_string());
        assert!(name.is_ok());
    }

    #[test]
    fn test_name_with_invalid_chars() {
        let name = Name::new("John@Doe".to_string());
        assert!(name.is_err());
    }

    #[test]
    fn test_name_with_numbers() {
        let name = Name::new("John Doe 2nd".to_string());
        assert!(name.is_ok());
    }
}
