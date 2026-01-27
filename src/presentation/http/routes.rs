use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::DynUserRepository;
use super::handlers;

/// Crea il router con tutte le route
pub fn create_routes(repository: DynUserRepository) -> Router {
    // Health check routes (non richiedono state)
    let health_routes = Router::new()
        .route("/health", get(handlers::health))
        .route("/ready", get(handlers::readiness));

    // API v1 routes
    let v1_routes = Router::new()
        .route("/users", post(handlers::v1::create_user))
        .route("/users", get(handlers::v1::list_users))
        .route("/users/:id", get(handlers::v1::get_user))
        .route("/users/:id", put(handlers::v1::update_user))
        .route("/users/:id", delete(handlers::v1::delete_user))
        .with_state(repository.clone());

    // Legacy routes (backward compatibility) - redirect to v1
    // Queste possono essere deprecate in futuro
    let legacy_routes = Router::new()
        .route("/api/users", post(handlers::create_user))
        .route("/api/users", get(handlers::list_users))
        .route("/api/users/:id", get(handlers::get_user))
        .route("/api/users/:id", put(handlers::update_user))
        .route("/api/users/:id", delete(handlers::delete_user))
        .with_state(repository);

    // Qui puoi aggiungere v2 routes in futuro
    // let v2_routes = Router::new()
    //     .route("/users", post(handlers::v2::create_user))
    //     ...
    //     .with_state(repository.clone());

    // Combina tutte le route
    health_routes
        .merge(legacy_routes)
        .nest("/api/v1", v1_routes)
        // .nest("/api/v2", v2_routes)  // Future API v2
}
