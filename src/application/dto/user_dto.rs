use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::User;

/// DTO per la creazione di un utente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub email: String,
    pub name: String,
}

/// DTO per l'aggiornamento di un utente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub email: Option<String>,
    pub name: Option<String>,
}

/// DTO per la risposta con i dati dell'utente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: *user.id(),
            email: user.email().value().to_string(),
            name: user.name().value().to_string(),
            created_at: *user.created_at(),
            updated_at: *user.updated_at(),
        }
    }
}

impl UserDto {
    pub fn from_users(users: Vec<User>) -> Vec<Self> {
        users.into_iter().map(Self::from).collect()
    }
}
