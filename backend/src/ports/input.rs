// INPUT PORTS (Driving Ports)
// Definiscono le operazioni che l'applicazione espone al mondo esterno
// Sono implementati dai Domain Services

use crate::domain::{DomainResult, User};
use uuid::Uuid;

/// Port di input per la gestione degli utenti
/// Definisce le operazioni che l'applicazione può eseguire
#[allow(dead_code)]
pub trait UserManagement {
    async fn create_user(&self, email: String, name: String, password: String) -> DomainResult<User>;
    async fn get_user(&self, id: Uuid) -> DomainResult<User>;
    async fn list_users(&self) -> DomainResult<Vec<User>>;
    async fn update_user(&self, id: Uuid, email: Option<String>, name: Option<String>) -> DomainResult<User>;
    async fn delete_user(&self, id: Uuid) -> DomainResult<()>;
}

// Note: Nella nostra implementazione, UserService implementa direttamente
// queste operazioni. In un'implementazione ancora più pura, potremmo
// creare un wrapper che implementa esplicitamente questo trait.
