use super::value_objects::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// User Entity - Contiene dati e comportamenti di business
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    email: Email,
    name: String,
    password_hash: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl User {
    /// Crea un nuovo utente
    pub fn create(email: Email, name: String, password: Password) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            name,
            password_hash: password.hash(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Ricostruisce un utente esistente (dal repository)
    pub fn reconstitute(
        id: Uuid,
        email: Email,
        name: String,
        password_hash: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            email,
            name,
            password_hash,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    // Business methods
    pub fn change_email(&mut self, new_email: Email) {
        self.email = new_email;
        self.updated_at = Utc::now();
    }

    pub fn change_name(&mut self, new_name: String) {
        self.name = new_name;
        self.updated_at = Utc::now();
    }

    pub fn change_password(&mut self, new_password: Password) {
        self.password_hash = new_password.hash();
        self.updated_at = Utc::now();
    }

    pub fn verify_password(&self, password: &Password) -> bool {
        password.verify(&self.password_hash)
    }
}
