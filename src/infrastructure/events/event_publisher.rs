use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::{DomainError, DomainEvent, EventEnvelope};

/// Trait per pubblicare eventi
#[async_trait]
pub trait EventPublisher: Send + Sync {
    /// Pubblica un singolo evento
    async fn publish<E>(&self, event: E) -> Result<(), DomainError>
    where
        E: DomainEvent + Serialize + 'static;

    /// Pubblica multipli eventi (transazionale)
    async fn publish_batch<E>(&self, events: Vec<E>) -> Result<(), DomainError>
    where
        E: DomainEvent + Serialize + 'static;
}

/// Implementazione in-memory (per testing/sviluppo)
/// In produzione, questo potrebbe essere sostituito con:
/// - RabbitMQ
/// - Kafka
/// - Redis Streams
/// - AWS EventBridge
/// etc.
#[derive(Clone)]
pub struct InMemoryEventPublisher {
    events: Arc<Mutex<Vec<EventEnvelope>>>,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Ottieni tutti gli eventi pubblicati (utile per testing)
    pub async fn get_events(&self) -> Vec<EventEnvelope> {
        self.events.lock().await.clone()
    }

    /// Pulisci tutti gli eventi (utile per testing)
    pub async fn clear(&self) {
        self.events.lock().await.clear();
    }

    /// Conta gli eventi pubblicati
    pub async fn count(&self) -> usize {
        self.events.lock().await.len()
    }
}

impl Default for InMemoryEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventPublisher for InMemoryEventPublisher {
    async fn publish<E>(&self, event: E) -> Result<(), DomainError>
    where
        E: DomainEvent + Serialize + 'static,
    {
        let envelope = EventEnvelope {
            event_id: Uuid::new_v4(),
            event_type: event.event_type().to_string(),
            aggregate_id: event.aggregate_id(),
            occurred_at: event.occurred_at(),
            version: event.version(),
            payload: serde_json::to_value(&event)
                .map_err(|e| DomainError::RepositoryError(format!("Failed to serialize event: {}", e)))?,
        };

        tracing::info!(
            event_type = %envelope.event_type,
            aggregate_id = %envelope.aggregate_id,
            event_id = %envelope.event_id,
            "Publishing domain event"
        );

        self.events.lock().await.push(envelope);
        Ok(())
    }

    async fn publish_batch<E>(&self, events: Vec<E>) -> Result<(), DomainError>
    where
        E: DomainEvent + Serialize + 'static,
    {
        tracing::info!(count = events.len(), "Publishing batch of events");
        
        for event in events {
            self.publish(event).await?;
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::events::UserCreatedEvent;

    #[tokio::test]
    async fn test_publish_event() {
        let publisher = InMemoryEventPublisher::new();
        let event = UserCreatedEvent::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "Test User".to_string(),
        );

        let result = publisher.publish(event).await;
        assert!(result.is_ok());

        let events = publisher.get_events().await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "user.created");
    }

    #[tokio::test]
    async fn test_publish_batch() {
        let publisher = InMemoryEventPublisher::new();
        
        let events = vec![
            UserCreatedEvent::new(
                Uuid::new_v4(),
                "test1@example.com".to_string(),
                "Test User 1".to_string(),
            ),
            UserCreatedEvent::new(
                Uuid::new_v4(),
                "test2@example.com".to_string(),
                "Test User 2".to_string(),
            ),
        ];

        let result = publisher.publish_batch(events).await;
        assert!(result.is_ok());

        let published = publisher.get_events().await;
        assert_eq!(published.len(), 2);
    }

    #[tokio::test]
    async fn test_clear_events() {
        let publisher = InMemoryEventPublisher::new();
        let event = UserCreatedEvent::new(
            Uuid::new_v4(),
            "test@example.com".to_string(),
            "Test User".to_string(),
        );

        publisher.publish(event).await.unwrap();
        assert_eq!(publisher.count().await, 1);

        publisher.clear().await;
        assert_eq!(publisher.count().await, 0);
    }
}
