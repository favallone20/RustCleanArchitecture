use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::value_objects::{Email, Name};

/// User Entity - Entità principale del dominio
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    id: Uuid,
    email: Email,
    name: Name,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    /// Crea un nuovo utente (usato per la creazione)
    pub fn new(email: Email, name: Name) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            name,
            created_at: now,
            updated_at: now,
        }
    }

    /// Ricostruisce un utente da dati persistenti (usato dal repository)
    pub fn reconstruct(
        id: Uuid,
        email: Email,
        name: Name,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            email,
            name,
            created_at,
            updated_at,
        }
    }

    /// Aggiorna il nome dell'utente
    pub fn update_name(&mut self, name: Name) {
        self.name = name;
        self.updated_at = Utc::now();
    }

    /// Aggiorna l'email dell'utente
    pub fn update_email(&mut self, email: Email) {
        self.email = email;
        self.updated_at = Utc::now();
    }

    // Getters
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let email = Email::new("test@example.com").unwrap();
        let name = Name::new("Test User".to_string()).unwrap();
        let user = User::new(email.clone(), name.clone());

        assert_eq!(user.email(), &email);
        assert_eq!(user.name(), &name);
    }

    #[test]
    fn test_update_name() {
        let email = Email::new("test@example.com").unwrap();
        let name = Name::new("Old Name".to_string()).unwrap();
        let mut user = User::new(email, name);
        let old_updated_at = *user.updated_at();

        // Aspetta un po' per assicurarsi che l'updated_at cambi
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        let new_name = Name::new("New Name".to_string()).unwrap();
        user.update_name(new_name.clone());

        assert_eq!(user.name(), &new_name);
        assert!(user.updated_at() > &old_updated_at);
    }

    #[test]
    fn test_update_email() {
        let email = Email::new("old@example.com").unwrap();
        let name = Name::new("Test User".to_string()).unwrap();
        let mut user = User::new(email, name);
        
        let new_email = Email::new("new@example.com").unwrap();
        user.update_email(new_email.clone());

        assert_eq!(user.email(), &new_email);
    }
}
