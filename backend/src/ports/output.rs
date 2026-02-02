// OUTPUT PORTS (Driven Ports)
// Definiscono le interfacce che l'applicazione necessita dal mondo esterno
// Sono implementati dagli Adapters (es: FileStorage, Database, External APIs)

use crate::domain::{DomainResult, User};
use async_trait::async_trait;
use uuid::Uuid;

/// Port di output per la persistenza degli utenti
/// Definisce le operazioni di storage richieste dall'applicazione
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Salva un nuovo utente
    async fn save(&self, user: User) -> DomainResult<User>;

    /// Trova un utente per ID
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<User>>;

    /// Trova un utente per email
    async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>>;

    /// Verifica se esiste un utente con questa email
    async fn exists_by_email(&self, email: &str) -> DomainResult<bool>;

    /// Trova tutti gli utenti
    async fn find_all(&self) -> DomainResult<Vec<User>>;

    /// Aggiorna un utente esistente
    async fn update(&self, user: User) -> DomainResult<User>;

    /// Elimina un utente per ID
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
}
