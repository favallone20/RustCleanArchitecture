// INPUT ADAPTERS (Driving Adapters)
// Ricevono input dall'esterno e chiamano il dominio
// Esempi: REST API, CLI, GraphQL, gRPC

pub mod handlers;
pub mod http_api;

pub use http_api::HttpServer;
