use super::DomainEvent;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Evento: Utente creato
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCreatedEvent {
    pub user_id: Uuid,
    pub email: String,
    pub name: String,
    pub occurred_at: DateTime<Utc>,
}

impl UserCreatedEvent {
    pub fn new(user_id: Uuid, email: String, name: String) -> Self {
        Self {
            user_id,
            email,
            name,
            occurred_at: Utc::now(),
        }
    }
}

impl DomainEvent for UserCreatedEvent {
    fn event_type(&self) -> &str {
        "user.created"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
}

/// Evento: Utente aggiornato
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUpdatedEvent {
    pub user_id: Uuid,
    pub email: Option<String>,
    pub name: Option<String>,
    pub occurred_at: DateTime<Utc>,
}

impl UserUpdatedEvent {
    pub fn new(user_id: Uuid, email: Option<String>, name: Option<String>) -> Self {
        Self {
            user_id,
            email,
            name,
            occurred_at: Utc::now(),
        }
    }
}

impl DomainEvent for UserUpdatedEvent {
    fn event_type(&self) -> &str {
        "user.updated"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
}

/// Evento: Utente eliminato
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDeletedEvent {
    pub user_id: Uuid,
    pub occurred_at: DateTime<Utc>,
}

impl UserDeletedEvent {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            occurred_at: Utc::now(),
        }
    }
}

impl DomainEvent for UserDeletedEvent {
    fn event_type(&self) -> &str {
        "user.deleted"
    }

    fn occurred_at(&self) -> DateTime<Utc> {
        self.occurred_at
    }

    fn aggregate_id(&self) -> Uuid {
        self.user_id
    }
}
