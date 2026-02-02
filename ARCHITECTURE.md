# Hexagonal Architecture - Guida all'Implementazione

## 🎯 Cos'è l'Hexagonal Architecture?

L'Hexagonal Architecture (detta anche "Ports and Adapters") è un pattern architetturale che mette il **Domain (business logic)** al centro dell'applicazione, isolandolo completamente dalle dipendenze esterne.

**Terminologia**: Usiamo **Input/Output** invece di Primary/Secondary perché è più intuitivo e immediato.

## 🏛️ I Tre Layer Fondamentali

### 1️⃣ Domain (Il Cuore)

**Cosa contiene:**
- **Entities**: Oggetti con identità e ciclo di vita (`User`)
- **Value Objects**: Oggetti immutabili con validazione (`Email`, `Password`)
- **Domain Services**: Logica di business che orchestra le entità (`UserService`)
- **Domain Errors**: Errori specifici del business

**Regola d'oro:** Il Domain **NON** dipende da nulla. Nessun import di framework, database, o infrastruttura.

```rust
// ✅ GIUSTO - Domain puro
pub struct User {
    id: Uuid,
    email: Email,
    name: String,
}

// ❌ SBAGLIATO - Domain che dipende da infrastruttura
use sqlx::FromRow;  // NO! Non importare librerie di database

#[derive(FromRow)]  // NO! Nessuna annotation di framework
pub struct User { }
```

### 2️⃣ Ports (Le Interfacce)

**Cosa contiene:**
- **Input Ports**: Interfacce che l'applicazione espone (cosa può fare)
- **Output Ports**: Interfacce che l'applicazione richiede (di cosa ha bisogno)

```rust
// Output Port - Il Domain definisce DI COSA ha bisogno
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: User) -> DomainResult<User>;
    // ...
}
```

**Regola chiave:** I Ports sono definiti dal Domain, non dall'infrastruttura!

### 3️⃣ Adapters (Le Implementazioni)

**Cosa contiene:**
- **Input Adapters** (Driving): Ricevono input dall'esterno
  - HTTP API (Axum) - con handlers organizzati per risorsa
  - CLI
  - GraphQL
  
- **Output Adapters** (Driven): Forniscono servizi al domain
  - File Storage
  - Database (PostgreSQL, MongoDB)
  - External APIs

```rust
// Output Adapter - Implementazione concreta del port
pub struct FileUserRepository { }

impl UserRepository for FileUserRepository {
    async fn save(&self, user: User) -> DomainResult<User> {
        // Implementazione con file system
    }
}

// Input Adapter - Handler HTTP per una specifica risorsa
pub async fn create_user(
    State(service): State<Arc<UserService>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<ApiResponse<UserDto>>, DomainError> {
    // Delega al domain service
}
```

## 📁 Struttura dei File

```
backend/src/
├── main.rs                  # Dependency injection e setup
│
├── domain/                  # 💎 DOMAIN LAYER
│   ├── mod.rs
│   ├── entities.rs         # User, Product entities
│   ├── value_objects.rs    # Email, Password, Money, Stock
│   ├── services.rs         # UserService (business logic)
│   └── error.rs            # DomainError, DomainResult
│
├── ports/                   # 🔌 PORTS (Interfaces)
│   ├── mod.rs
│   ├── input.rs            # Interfacce esposte (UserManagement)
│   └── output.rs           # Interfacce richieste (UserRepository)
│
└── adapters/                # 🔧 ADAPTERS
    ├── mod.rs
    │
    ├── input/               # INPUT ADAPTERS
    │   ├── mod.rs
    │   ├── http_api.rs     # Server setup e routing
    │   └── handlers/       # 📂 Handlers organizzati per risorsa
    │       ├── mod.rs
    │       └── user_handlers.rs   # POST/GET/PUT/DELETE /api/users
    │
    └── output/              # OUTPUT ADAPTERS
        ├── mod.rs
        └── file_storage.rs  # FileUserRepository (JSON storage)
```

### Organizzazione degli Handlers

Ogni risorsa ha il suo file di handlers:

- **`user_handlers.rs`** - Gestisce `/api/users/*`
  - `create_user()` → POST /api/users
  - `get_user()` → GET /api/users/:id
  - `get_all_users()` → GET /api/users
  - `update_user()` → PUT /api/users/:id
  - `delete_user()` → DELETE /api/users/:id

**Perché separare gli handlers?**
- ✅ Un file per risorsa = facile da trovare
- ✅ Responsabilità chiara e singola
- ✅ Facile aggiungere nuove risorse (es: `product_handlers.rs`)
- ✅ Testing più semplice e isolato

## 🔄 Flusso Completo di una Richiesta

```
1. HTTP Request: POST /api/users
   ↓
2. HttpServer (http_api.rs)
   ↓ routing
3. user_handlers::create_user() ← Input Adapter
   ↓ estrae State e Json
4. UserService.create_user() ← Domain Service
   ↓ orchestrazione business logic
5. UserRepository trait ← Output Port (interfaccia)
   ↓ implementato da
6. FileUserRepository.save() ← Output Adapter
   ↓ persistenza
7. File system (users.json)

Risposta: JSON con UserDto
```

### Dettaglio Step by Step

```
┌────────────────────┐
│  HTTP POST Request │
│  /api/users        │
└─────────┬──────────┘
          │
          ▼
┌────────────────────────────────────┐
│  HttpServer (http_api.rs)          │
│  Router match → user_handlers::    │
│  create_user                       │
└─────────┬──────────────────────────┘
          │
          ▼
┌────────────────────────────────────┐
│  user_handlers.rs                  │ ← Input Adapter
│  - Estrae CreateUserDto dal body   │
│  - Chiama domain service           │
│  - Converte User → UserDto         │
└─────────┬──────────────────────────┘
          │
          ▼
┌────────────────────────────────────┐
│  UserService (domain/services.rs)  │ ← Domain
│  - Validazione business logic      │
│  - Crea entità User                │
│  - Chiama repository.save()        │
└─────────┬──────────────────────────┘
          │
          ▼
┌────────────────────────────────────┐
│  UserRepository trait              │ ← Output Port
│  (ports/output.rs)                 │
└─────────┬──────────────────────────┘
          │ implementato da
          ▼
┌────────────────────────────────────┐
│  FileUserRepository                │ ← Output Adapter
│  (adapters/output/file_storage.rs)│
│  - Salva su users.json             │
└────────────────────────────────────┘
```

## 🔌 Dependency Injection

Il `main.rs` è l'unico posto dove colleghiamo tutto insieme:

```rust
#[tokio::main]
async fn main() {
    // 1. Crea gli Output Adapters
    let repository = Arc::new(FileUserRepository::new("data/users.json"));
    
    // 2. Inietta nel Domain Service
    let user_service = Arc::new(UserService::new(repository));
    
    // 3. Inietta negli Input Adapters
    let api = HttpServer::new(user_service);
    api.start("127.0.0.1:3000").await;
}
```

## 🆕 Come Aggiungere un Nuovo Handler

### Scenario: Aggiungi gestione Prodotti

**1. Crea il file handler**

`adapters/input/handlers/product_handlers.rs`:

```rust
use crate::domain::{DomainError, ProductService};
use axum::{extract::{Path, State}, Json};
use contracts::{ApiResponse, CreateProductDto, ProductDto};
use std::sync::Arc;
use uuid::Uuid;

pub async fn create_product(
    State(service): State<Arc<ProductService>>,
    Json(dto): Json<CreateProductDto>,
) -> Result<Json<ApiResponse<ProductDto>>, DomainError> {
    let product = service.create_product(dto.name, dto.price, dto.stock).await?;
    let dto = product_to_dto(&product);
    Ok(Json(ApiResponse::success(dto)))
}

pub async fn get_product(
    State(service): State<Arc<ProductService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProductDto>>, DomainError> {
    let product = service.get_product(id).await?;
    Ok(Json(ApiResponse::success(product_to_dto(&product))))
}

fn product_to_dto(product: &Product) -> ProductDto {
    // conversione
}
```

**2. Registra il modulo**

`adapters/input/handlers/mod.rs`:

```rust
pub mod user_handlers;
pub mod product_handlers;  // ← Nuovo!

pub use user_handlers::*;
pub use product_handlers::*;
```

**3. Aggiungi le routes**

`adapters/input/http_api.rs`:

```rust
use super::handlers::{user_handlers, product_handlers};

fn routes(self) -> Router {
    Router::new()
        // User routes
        .route("/api/users", post(user_handlers::create_user))
        // ...
        
        // Product routes ← Nuovo!
        .route("/api/products", post(product_handlers::create_product))
        .route("/api/products/:id", get(product_handlers::get_product))
        
        .with_state(self.user_service)
}
```

**Fatto!** Ogni handler vive nel suo file dedicato. 🎉

## 🎭 Esempio Pratico: Cambiare Persistenza

### Scenario: Voglio salvare su PostgreSQL invece che su File

**Cosa devi fare:**

1. **Crea un nuovo Output Adapter** (`postgres_repository.rs`)
```rust
pub struct PostgresUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: User) -> DomainResult<User> {
        // Implementazione con PostgreSQL
    }
}
```

2. **Cambia solo il `main.rs`**
```rust
// Cambia SOLO questa riga:
let repository = Arc::new(PostgresUserRepository::new(pool));
// ↑ Prima era FileUserRepository, ora PostgresUserRepository

// IL RESTO DEL CODICE RIMANE IDENTICO! 🎉
```

**Cosa NON devi fare:**
- ❌ NON modificare il Domain
- ❌ NON modificare i Ports
- ❌ NON modificare l'Input Adapter (HTTP API)
- ❌ NON modificare UserService
- ❌ NON modificare gli Handlers

## 🧪 Testing Semplificato

### Test del Domain (senza dipendenze)

```rust
#[tokio::test]
async fn test_create_user() {
    // Crea un Mock Repository
    let mock_repo = Arc::new(MockUserRepository::new());
    let service = UserService::new(mock_repo);
    
    let user = service
        .create_user("test@example.com".to_string(), "Test".to_string(), "password123".to_string())
        .await
        .unwrap();
    
    assert_eq!(user.email().value(), "test@example.com");
}
```

### Test degli Handlers

```rust
#[tokio::test]
async fn test_create_user_handler() {
    let mock_service = Arc::new(MockUserService::new());
    let app = create_test_app(mock_service);
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"email":"test@test.com","name":"Test","password":"pass1234"}"#))
                .unwrap()
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
}
```

## 🆚 Confronto: Hexagonal vs Layered Architecture

### Architettura a Layer Tradizionale:
```
┌─────────────────┐
│   Presentation  │ (UI, API)
├─────────────────┤
│    Business     │ (Logic)
├─────────────────┤
│   Persistence   │ (Database)
└─────────────────┘
```
❌ Dipendenze vanno dall'alto verso il basso  
❌ Il Business dipende dal Database

### Hexagonal Architecture:
```
        ┌─────────────┐
        │   Adapters  │
        └──────┬──────┘
               │
        ┌──────▼──────┐
        │    Ports    │
        └──────┬──────┘
               │
        ┌──────▼──────┐
        │   Domain    │ ← Centro indipendente
        └─────────────┘
```
✅ Dipendenze vanno verso l'interno  
✅ Il Domain non dipende da nulla

## 📚 Vantaggi dell'Hexagonal Architecture

1. **Testabilità**: Testa il business logic senza database
2. **Flessibilità**: Cambia infrastruttura senza toccare il core
3. **Indipendenza**: Il domain non "sa" se sta usando file, DB, o API
4. **Manutenibilità**: Ogni componente ha una responsabilità chiara
5. **Protezione**: Il business logic è al sicuro da cambiamenti esterni
6. **Scalabilità**: Aggiungi nuovi handler/adapters facilmente

## 🎓 Best Practices

### ✅ DA FARE

- Metti TUTTA la business logic nel Domain
- Usa Value Objects per validazione
- Tieni i Ports nel modulo `ports/`
- Tieni gli Adapters nel modulo `adapters/`
- Fai Dependency Injection esplicita nel `main.rs`
- **Un file per risorsa negli handlers** (user_handlers.rs, product_handlers.rs)

### ❌ DA EVITARE

- NON mettere logica di business negli Adapters o Handlers
- NON far dipendere il Domain dall'infrastruttura
- NON usare annotation di framework nel Domain
- NON accedere direttamente agli Adapters dal Domain
- NON mescolare handler di risorse diverse nello stesso file

## 🔗 Risorse Utili

- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/) - Alistair Cockburn (creatore)
- [Netflix Blog](https://netflixtechblog.com/ready-for-changes-with-hexagonal-architecture-b315ec967749)
- [Domain-Driven Design](https://martinfowler.com/tags/domain%20driven%20design.html) - Martin Fowler

---

**Remember:** L'obiettivo è **proteggere il business logic** da cambiamenti tecnologici. Il Domain è il re! 👑
