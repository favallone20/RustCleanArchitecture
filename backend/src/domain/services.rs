// DOMAIN SERVICES - Orchestrano la business logic usando entità e ports
// Questo è il cuore dell'Hexagonal Architecture

use super::{DomainError, DomainResult, Email, Password, User};
use crate::ports::output::UserRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    /// Crea un nuovo utente
    pub async fn create_user(
        &self,
        email: String,
        name: String,
        password: String,
    ) -> DomainResult<User> {
        // 1. Validazione: verifica che l'email non esista già
        if self.repository.exists_by_email(&email).await? {
            return Err(DomainError::Conflict(format!(
                "Un utente con email {} esiste già",
                email
            )));
        }

        // 2. Crea value objects (con validazione)
        let email = Email::new(email)?;
        let password = Password::new(password)?;

        // 3. Crea l'entità User
        let user = User::create(email, name, password);

        // 4. Persisti tramite il port
        self.repository.save(user).await
    }

    /// Recupera un utente per ID
    pub async fn get_user(&self, id: Uuid) -> DomainResult<User> {
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Utente {} non trovato", id)))
    }

    /// Lista tutti gli utenti
    pub async fn list_users(&self) -> DomainResult<Vec<User>> {
        self.repository.find_all().await
    }

    /// Aggiorna un utente
    pub async fn update_user(
        &self,
        id: Uuid,
        new_email: Option<String>,
        new_name: Option<String>,
    ) -> DomainResult<User> {
        // 1. Recupera l'utente esistente
        let mut user = self.get_user(id).await?;

        // 2. Applica le modifiche
        if let Some(email_str) = new_email {
            let email = Email::new(email_str)?;
            user.change_email(email);
        }

        if let Some(name) = new_name {
            user.change_name(name);
        }

        // 3. Salva le modifiche
        self.repository.update(user).await
    }

    /// Elimina un utente
    pub async fn delete_user(&self, id: Uuid) -> DomainResult<()> {
        // Verifica che esista
        self.get_user(id).await?;
        
        // Elimina
        self.repository.delete(id).await
    }

    /// Trova un utente per email
    pub async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>> {
        self.repository.find_by_email(email).await
    }
}
