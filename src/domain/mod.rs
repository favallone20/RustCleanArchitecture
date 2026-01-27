pub mod entities;
pub mod value_objects;
pub mod repositories;
pub mod errors;
pub mod events;

// Re-exports per facilitare l'uso
pub use entities::User;
pub use value_objects::{Email, Name};
pub use repositories::UserRepository;
pub use errors::DomainError;
pub use events::{DomainEvent, EventEnvelope, UserCreatedEvent, UserUpdatedEvent, UserDeletedEvent};
