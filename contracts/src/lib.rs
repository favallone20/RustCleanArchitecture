// Libreria di contratti (DTOs) condivisi tra frontend e backend
// Supporta serializzazione JSON e può essere compilata per WASM

pub mod dto;
pub mod error;

pub use dto::*;
pub use error::*;
