// HTTP API ADAPTER - Input Adapter che espone REST API
// Configurazione del server e routing principale

use super::handlers::user_handlers;
use crate::domain::{DomainError, UserService};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use contracts::ApiResponse;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

// ============================================================================
// Server HTTP
// ============================================================================

pub struct HttpServer {
    user_service: Arc<UserService>,
}

impl HttpServer {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    pub async fn start(self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let app = self.routes();

        println!("🌐 Server HTTP in ascolto su http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }

    fn routes(self) -> Router {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        Router::new()
            // Health check
            .route("/health", get(health_check))
            
            // User routes - Delegati a user_handlers
            .route("/api/users", post(user_handlers::create_user))
            .route("/api/users", get(user_handlers::get_all_users))
            .route("/api/users/:id", get(user_handlers::get_user))
            .route("/api/users/:id", put(user_handlers::update_user))
            .route("/api/users/:id", delete(user_handlers::delete_user))
            
            .layer(cors)
            .with_state(self.user_service)
    }
}

// ============================================================================
// Health Check Handler
// ============================================================================

async fn health_check() -> &'static str {
    "OK"
}

// ============================================================================
// Error Handling - Converte DomainError in HTTP Response
// ============================================================================

impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            DomainError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            DomainError::InvalidEmail(msg) => {
                (StatusCode::BAD_REQUEST, format!("Email non valida: {}", msg))
            }
            DomainError::InvalidPassword(msg) => (StatusCode::BAD_REQUEST, msg),
            DomainError::StorageError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Errore di storage: {}", msg))
            }
        };

        let body = Json(ApiResponse::<()>::error(message));
        (status, body).into_response()
    }
}
