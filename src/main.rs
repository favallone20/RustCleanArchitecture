use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use clean_architecture_rust::{create_routes, AppConfig, DynUserRepository, InMemoryUserRepository};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inizializza il logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "clean_architecture_rust=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Carica la configurazione
    let config = AppConfig::from_env();
    let addr = config.server_address();

    tracing::info!("🚀 Starting Clean Architecture Rust Server");
    tracing::info!("📍 Server address: {}", addr);

    // Dependency Injection: Crea il repository
    // Ora è facile sostituire con PostgresUserRepository o altri!
    // Basta cambiare questa linea senza toccare altro codice
    let repository: DynUserRepository = Arc::new(InMemoryUserRepository::new());

    // Crea il router con tutte le route
    let app = create_routes(repository)
        .layer(CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // Crea il listener
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    tracing::info!("✅ Server listening on http://{}", addr);
    tracing::info!("📚 API Documentation:");
    tracing::info!("");
    tracing::info!("   Health Checks:");
    tracing::info!("   GET    /health            - Liveness probe");
    tracing::info!("   GET    /ready             - Readiness probe");
    tracing::info!("");
    tracing::info!("   User Management (v1):");
    tracing::info!("   POST   /api/v1/users      - Create a new user");
    tracing::info!("   GET    /api/v1/users      - List all users");
    tracing::info!("   GET    /api/v1/users/:id  - Get user by ID");
    tracing::info!("   PUT    /api/v1/users/:id  - Update user");
    tracing::info!("   DELETE /api/v1/users/:id  - Delete user");
    tracing::info!("");
    tracing::info!("   Legacy routes (deprecated, use /api/v1 instead):");
    tracing::info!("   POST   /api/users         - Create a new user");
    tracing::info!("   GET    /api/users         - List all users");
    tracing::info!("   (and others...)");
    tracing::info!("");
    tracing::info!("Example usage:");
    tracing::info!("  curl http://{}/health", addr);
    tracing::info!("  curl -X POST http://{}/api/v1/users \\", addr);
    tracing::info!("    -H 'Content-Type: application/json' \\");
    tracing::info!("    -d '{{\"email\":\"test@example.com\",\"name\":\"Test User\"}}'");

    // Avvia il server
    axum::serve(listener, app).await?;

    Ok(())
}
