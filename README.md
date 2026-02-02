# Backend Rust - Hexagonal Architecture (Pura)

Un backend moderno in Rust implementato seguendo i principi dell'**Hexagonal Architecture** (Ports and Adapters), con contratti condivisi tra frontend e backend.

## 📋 Indice

- [Caratteristiche](#caratteristiche)
- [Architettura](#architettura)
- [Struttura del Progetto](#struttura-del-progetto)
- [Installazione](#installazione)
- [Utilizzo](#utilizzo)
- [API Endpoints](#api-endpoints)
- [Contratti Condivisi](#contratti-condivisi)

## ✨ Caratteristiche

- **Hexagonal Architecture Pura**: Separazione netta tra Domain, Ports e Adapters
- **Contratti Condivisi**: Libreria `contracts` riutilizzabile tra frontend e backend
- **Persistenza File-based**: Storage JSON su file system (niente database pesanti)
- **Dipendenze Minime**: Solo le librerie essenziali
- **Type Safety**: Forte tipizzazione con Rust
- **Domain Isolato**: Business logic completamente indipendente dall'infrastruttura

## 🏗️ Architettura

### Hexagonal Architecture - Ports & Adapters

```
┌─────────────────────────────────────────────────────────────┐
│                      INPUT ADAPTERS                         │
│                     (Driving Adapters)                      │
│                                                             │
│  ┌──────────────┐        ┌──────────────┐                 │
│  │   HTTP API   │        │     CLI      │                 │
│  │   (Axum)     │        │  (Future)    │                 │
│  └──────┬───────┘        └──────┬───────┘                 │
│         │                       │                          │
└─────────┼───────────────────────┼──────────────────────────┘
          │                       │
          │     INPUT PORTS       │
          │      (Driving)        │
          ▼                       ▼
┌─────────────────────────────────────────────────────────────┐
│                       DOMAIN CORE                           │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              DOMAIN SERVICES                        │   │
│  │         (Business Logic Orchestration)              │   │
│  │  • UserService                                      │   │
│  └─────────────────────────────────────────────────────┘   │
│                          │                                  │
│                          │ uses                             │
│                          ▼                                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │              ENTITIES & VALUE OBJECTS               │   │
│  │  • User (Entity)                                    │   │
│  │  • Email, Password (Value Objects)                  │   │
│  └─────────────────────────────────────────────────────┘   │
│                          │                                  │
│                          │ requires                         │
│                          ▼                                  │
│                    OUTPUT PORTS                             │
│                     (Driven)                                │
│         UserRepository (Interface/Trait)                    │
└─────────────────────────┬───────────────────────────────────┘
                          │
                          │ implemented by
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                    OUTPUT ADAPTERS                          │
│                     (Driven Adapters)                       │
│                                                             │
│  ┌──────────────┐        ┌──────────────┐                 │
│  │ File Storage │        │   Database   │                 │
│  │   (JSON)     │        │  (Future)    │                 │
│  └──────────────┘        └──────────────┘                 │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Principi Chiave

1. **Domain al Centro**: La business logic non dipende da nulla
2. **Dependency Inversion**: Le dipendenze puntano verso l'interno
3. **Ports (Interfacce)**: Definiscono i contratti
4. **Adapters (Implementazioni)**: Connettono il domain con il mondo esterno
5. **Testabilità**: Ogni layer è facilmente testabile in isolamento

## 📁 Struttura del Progetto

```
backend-rust/
├── Cargo.toml              # Workspace configuration
├── Makefile                # Comandi di sviluppo
├── README.md
│
├── contracts/              # 📜 Contratti condivisi (DTO)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── dto.rs          # UserDto, CreateUserDto, etc.
│       └── error.rs        # ApiError, ErrorCode
│
├── backend/                # 🚀 Applicazione Backend
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs         # Entry point & dependency wiring
│   │   │
│   │   ├── domain/         # 💎 DOMAIN CORE
│   │   │   ├── mod.rs
│   │   │   ├── entities.rs      # User entity
│   │   │   ├── value_objects.rs # Email, Password
│   │   │   ├── services.rs      # UserService (business logic)
│   │   │   └── error.rs         # Domain errors
│   │   │
│   │   ├── ports/          # 🔌 PORTS (Interfaces)
│   │   │   ├── mod.rs
│   │   │   ├── input.rs         # Input ports (API contracts)
│   │   │   └── output.rs        # Output ports (Repository trait)
│   │   │
│   │   └── adapters/       # 🔧 ADAPTERS (Implementations)
│   │       ├── mod.rs
│   │       ├── input/           # Driving adapters
│   │       │   ├── mod.rs
│   │       │   └── http_api.rs  # REST API with Axum
│   │       └── output/          # Driven adapters
│   │           ├── mod.rs
│   │           └── file_storage.rs  # JSON file repository
│   │
│   └── data/               # 📂 Storage files (gitignored)
│       └── users.json
│
└── test-api.sh             # Script per testare le API
```

## 🚀 Installazione

### Prerequisiti

- Rust 1.75+ ([Installa Rust](https://rustup.rs/))

### Setup

1. **Clona il repository**
   ```bash
   cd "backend rust"
   ```

2. **Compila il progetto**
   ```bash
   cargo build
   ```

## 💻 Utilizzo

### Avvio del Server

```bash
# Dalla directory backend
cd backend
cargo run
```

Il server sarà disponibile su `http://127.0.0.1:3000`

### I Dati Vengono Salvati su File

I dati degli utenti sono persistiti in `backend/data/users.json` in formato JSON leggibile.

## 🌐 API Endpoints

### Health Check

```http
GET /health
```

### Users API

```http
POST   /api/users           # Crea un nuovo utente
GET    /api/users           # Lista tutti gli utenti
GET    /api/users/:id       # Ottieni un utente specifico
PUT    /api/users/:id       # Aggiorna un utente
DELETE /api/users/:id       # Elimina un utente
```

### Esempi

#### Crea Utente

```bash
curl -X POST http://127.0.0.1:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "email": "mario.rossi@example.com",
    "name": "Mario Rossi",
    "password": "password123"
  }'
```

**Risposta:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "mario.rossi@example.com",
    "name": "Mario Rossi",
    "created_at": "2024-01-01T12:00:00Z",
    "updated_at": "2024-01-01T12:00:00Z"
  },
  "error": null
}
```

#### Lista Utenti

```bash
curl http://127.0.0.1:3000/api/users
```

#### Aggiorna Utente

```bash
curl -X PUT http://127.0.0.1:3000/api/users/550e8400-e29b-41d4-a716-446655440000 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Mario Verdi"
  }'
```

#### Elimina Utente

```bash
curl -X DELETE http://127.0.0.1:3000/api/users/550e8400-e29b-41d4-a716-446655440000
```

## 📦 Contratti Condivisi

La crate `contracts` definisce i contratti tra frontend e backend.

### Nel Backend

```toml
[dependencies]
contracts = { path = "../contracts" }
```

```rust
use contracts::{UserDto, CreateUserDto, ApiResponse};
```

### Nel Frontend (esempio con Yew/Leptos)

```toml
[dependencies]
contracts = { path = "../backend-rust/contracts" }
```

```rust
use contracts::{UserDto, ApiResponse};

// Deserializza la risposta dall'API
let response: ApiResponse<UserDto> = serde_json::from_str(&json)?;
```

### DTO Disponibili

- **User**: `UserDto`, `CreateUserDto`, `UpdateUserDto`
- **Response**: `ApiResponse<T>`
- **Error**: `ApiError`, `ErrorCode`

## 🎯 Come Funziona l'Architettura Esagonale

### 1. Il Domain è al Centro

```rust
// domain/entities.rs - NON dipende da nulla
pub struct User {
    id: Uuid,
    email: Email,
    name: String,
    // ...
}

impl User {
    pub fn create(email: Email, name: String, password: Password) -> Self {
        // Business logic pura
    }
}
```

### 2. I Ports Definiscono i Contratti

```rust
// ports/secondary.rs - Interfaccia che il domain richiede
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> DomainResult<User>;
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<User>>;
    // ...
}
```

### 3. Gli Adapters Implementano i Ports

```rust
// adapters/secondary/file_storage.rs - Implementazione concreta
pub struct FileUserRepository { /* ... */ }

#[async_trait]
impl UserRepository for FileUserRepository {
    async fn save(&self, user: User) -> DomainResult<User> {
        // Salva su file JSON
    }
}
```

### 4. I Domain Services Orchestrano la Logica

```rust
// domain/services.rs - Usa i ports, non gli adapters
pub struct UserService {
    repository: Arc<dyn UserRepository>,  // <- Dipende dall'interfaccia
}

impl UserService {
    pub async fn create_user(&self, email: String, ...) -> DomainResult<User> {
        // 1. Validazione
        // 2. Creazione entità
        // 3. Persistenza tramite port
    }
}
```

### 5. Il Main Collega Tutto (Dependency Injection)

```rust
// main.rs - Wiring delle dipendenze
#[tokio::main]
async fn main() {
    // Crea gli adapters
    let repository = Arc::new(FileUserRepository::new("data/users.json"));
    
    // Inietta nel domain service
    let user_service = Arc::new(UserService::new(repository));
    
    // Avvia l'adapter primario
    let server = HttpServer::new(user_service);
    server.start("127.0.0.1:3000").await?;
}
```

## 🔄 Sostituire gli Adapters

Grazie all'architettura esagonale, puoi facilmente sostituire gli adapters:

### Esempio: Da File a Database

```rust
// Crea un nuovo adapter
pub struct PostgresUserRepository { /* ... */ }

#[async_trait]
impl UserRepository for PostgresUserRepository {
    // Implementa il trait
}

// Nel main.rs, cambia solo questa riga:
let repository = Arc::new(PostgresUserRepository::new(pool));
// Il resto del codice rimane identico!
```

### Esempio: Da HTTP a CLI

```rust
// Crea un nuovo input adapter
pub struct CliAdapter {
    user_service: Arc<UserService>,
}

impl CliAdapter {
    pub async fn run(&self) {
        // Leggi input da terminale
        // Chiama user_service
        // Mostra output
    }
}
```

## 🧪 Testing

L'architettura esagonale rende i test estremamente semplici:

```rust
// Test del Domain (senza dipendenze esterne)
#[tokio::test]
async fn test_create_user() {
    let mock_repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(mock_repo);
    
    let user = service
        .create_user("test@example.com".to_string(), "Test".to_string(), "password123".to_string())
        .await
        .unwrap();
    
    assert_eq!(user.email().value(), "test@example.com");
}
```

## 📚 Vantaggi di Questa Architettura

✅ **Indipendenza dal Framework**: Cambia Axum con Actix senza toccare il domain  
✅ **Indipendenza dal Database**: Passa da file a PostgreSQL facilmente  
✅ **Testabilità**: Testa il business logic senza dipendenze esterne  
✅ **Manutenibilità**: Ogni componente ha una responsabilità chiara  
✅ **Scalabilità**: Aggiungi nuovi adapters senza modificare il core  
✅ **Chiarezza**: L'architettura è evidente dalla struttura del codice  

## 🎓 Risorse

- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/) - Alistair Cockburn
- [Ports and Adapters Pattern](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software))
- [Domain-Driven Design](https://martinfowler.com/bliki/DomainDrivenDesign.html)

## 📄 Licenza

MIT License

## ✍️ Autore

Creato con ❤️ usando Rust e Hexagonal Architecture
