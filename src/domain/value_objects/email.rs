use crate::domain::errors::DomainError;

/// Email Value Object - Garantisce che l'email sia sempre valida
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(email: impl Into<String>) -> Result<Self, DomainError> {
        let value = email.into();
        
        // Validazione base dell'email
        if !Self::is_valid(&value) {
            return Err(DomainError::InvalidEmail(value));
        }

        Ok(Self { value })
    }

    fn is_valid(email: &str) -> bool {
        // Validazione base: deve contenere @ e un dominio
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return false;
        }

        let local = parts[0];
        let domain = parts[1];

        !local.is_empty() 
            && !domain.is_empty() 
            && domain.contains('.') 
            && email.len() <= 254
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::new("test@example.com");
        assert!(email.is_ok());
    }

    #[test]
    fn test_invalid_email_no_at() {
        let email = Email::new("testexample.com");
        assert!(email.is_err());
    }

    #[test]
    fn test_invalid_email_no_domain() {
        let email = Email::new("test@");
        assert!(email.is_err());
    }

    #[test]
    fn test_invalid_email_no_local() {
        let email = Email::new("@example.com");
        assert!(email.is_err());
    }

    #[test]
    fn test_invalid_email_no_dot_in_domain() {
        let email = Email::new("test@example");
        assert!(email.is_err());
    }
}
