// DOMAIN - Il cuore dell'applicazione
// Contiene entità, value objects, business logic e domain services
// NON dipende da nessun altro layer

pub mod entities;
pub mod error;
pub mod services;
pub mod value_objects;

pub use entities::*;
pub use error::*;
pub use services::*;
pub use value_objects::*;
