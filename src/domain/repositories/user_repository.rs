use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::value_objects::Email;

/// User Repository Trait - Interfaccia per la persistenza degli utenti
/// Questo è un Port nell'architettura esagonale
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Salva un nuovo utente
    async fn save(&self, user: User) -> Result<User, DomainError>;

    /// Trova un utente per ID
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError>;

    /// Trova un utente per email
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError>;

    /// Trova tutti gli utenti
    async fn find_all(&self) -> Result<Vec<User>, DomainError>;

    /// Aggiorna un utente esistente
    async fn update(&self, user: User) -> Result<User, DomainError>;

    /// Elimina un utente per ID
    async fn delete(&self, id: &Uuid) -> Result<(), DomainError>;

    /// Controlla se esiste un utente con una determinata email
    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError>;
}

/// Implementazione di UserRepository per Arc<dyn UserRepository>
/// Questo permette di usare Arc<dyn UserRepository> direttamente negli use cases
#[async_trait]
impl UserRepository for Arc<dyn UserRepository> {
    async fn save(&self, user: User) -> Result<User, DomainError> {
        self.as_ref().save(user).await
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError> {
        self.as_ref().find_by_id(id).await
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        self.as_ref().find_by_email(email).await
    }

    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        self.as_ref().find_all().await
    }

    async fn update(&self, user: User) -> Result<User, DomainError> {
        self.as_ref().update(user).await
    }

    async fn delete(&self, id: &Uuid) -> Result<(), DomainError> {
        self.as_ref().delete(id).await
    }

    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError> {
        self.as_ref().exists_by_email(email).await
    }
}
