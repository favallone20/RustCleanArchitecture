use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod user_events;

pub use user_events::*;

/// Trait per tutti gli eventi di dominio
/// Gli eventi rappresentano fatti che sono accaduti nel sistema
pub trait DomainEvent: Send + Sync {
    /// Tipo dell'evento (es: "user.created")
    fn event_type(&self) -> &str;

    /// Quando è occorso l'evento
    fn occurred_at(&self) -> DateTime<Utc>;

    /// ID dell'aggregato che ha generato l'evento
    fn aggregate_id(&self) -> Uuid;

    /// Versione dell'evento (per evoluzione schema)
    fn version(&self) -> u32 {
        1
    }
}

/// Wrapper per serializzare eventi
/// Utile per event sourcing e event store
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_id: Uuid,
    pub event_type: String,
    pub aggregate_id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub version: u32,
    pub payload: serde_json::Value,
}
