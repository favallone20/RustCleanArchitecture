// PORTS - Interfacce che definiscono i contratti
// Nella Hexagonal Architecture, i ports sono le porte attraverso cui
// il dominio comunica con il mondo esterno

pub mod input;
pub mod output;

pub use input::*;
pub use output::*;
