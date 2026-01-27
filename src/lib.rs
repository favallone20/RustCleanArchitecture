// Clean Architecture in Rust - Library
// 
// Questa libreria implementa i principi della Clean Architecture:
// - Domain Layer: Entità di business e logica core
// - Application Layer: Use cases e orchestrazione
// - Infrastructure Layer: Implementazioni concrete (DB, servizi esterni)
// - Presentation Layer: HTTP handlers e API

use std::sync::Arc;

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-exports principali per facilitare l'uso esterno
pub use domain::{DomainError, Email, User, UserRepository};
pub use application::{
    CreateUserDto, UpdateUserDto, UserDto,
    CreateUserUseCase, GetUserUseCase, ListUsersUseCase, UpdateUserUseCase, DeleteUserUseCase,
};
pub use infrastructure::{AppConfig, InMemoryUserRepository};
pub use presentation::create_routes;

// Type alias per il repository usato nell'applicazione
// Questo permette di cambiare implementazione in un solo punto
// Arc<dyn T> è necessario per permettere la condivisione thread-safe di un trait object
pub type DynUserRepository = Arc<dyn UserRepository>;
