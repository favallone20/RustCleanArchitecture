use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadinessResponse {
    pub ready: bool,
    pub checks: ReadinessChecks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadinessChecks {
    pub database: bool,
    pub dependencies: bool,
}

/// Liveness probe - server è vivo?
/// Questo endpoint dovrebbe sempre rispondere 200 se il server è in esecuzione
pub async fn health() -> Json<HealthResponse> {
    tracing::debug!("Health check requested");
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Readiness probe - server è pronto a ricevere traffico?
/// Questo endpoint verifica che tutte le dipendenze siano disponibili
pub async fn readiness() -> Result<Json<ReadinessResponse>, StatusCode> {
    tracing::debug!("Readiness check requested");
    
    // TODO: Verifica connessione database quando implementato
    // Per ora usiamo sempre true per l'implementazione in-memory
    let db_ready = true;
    let deps_ready = true;

    let ready = db_ready && deps_ready;

    let response = ReadinessResponse {
        ready,
        checks: ReadinessChecks {
            database: db_ready,
            dependencies: deps_ready,
        },
    };

    if ready {
        tracing::debug!("System is ready");
        Ok(Json(response))
    } else {
        tracing::warn!(
            database = db_ready,
            dependencies = deps_ready,
            "System is not ready"
        );
        Err(StatusCode::SERVICE_UNAVAILABLE)
    }
}
