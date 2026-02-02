mod adapters;
mod domain;
mod ports;

use adapters::input::http_api::HttpServer;
use adapters::output::file_storage::FileUserRepository;
use domain::services::UserService;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Avvio Backend - Hexagonal Architecture");

    // 1. Crea gli output adapters (dipendenze esterne)
    let user_repository = Arc::new(FileUserRepository::new("data/users.json"));

    // 2. Crea i domain services (core business logic)
    let user_service = Arc::new(UserService::new(user_repository));

    // 3. Avvia l'input adapter (HTTP API)
    let server = HttpServer::new(user_service);
    server.start("127.0.0.1:3000").await?;

    Ok(())
}
