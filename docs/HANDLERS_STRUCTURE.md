# 📂 Struttura degli Handler

Questa guida spiega come è organizzata la struttura degli handler e come aggiungerne di nuovi.

## 🏗️ Struttura Modulare

Invece di un singolo file `handlers.rs` monolitico, gli handler sono organizzati in una cartella con file separati per ogni risorsa/entità:

```
src/presentation/http/
├── handlers/
│   ├── mod.rs                               # Esporta tutti gli handler
│   ├── user_handlers.rs                     # Handler per User
│   ├── product_handlers.rs                  # (esempio) Handler per Product
│   └── example_product_handlers.rs.template # Template per nuovi handler
├── routes.rs                                # Configurazione route
└── mod.rs                                   # Modulo HTTP
```

## 📋 Vantaggi di questa Struttura

### ✅ Scalabilità
- Ogni entità ha il suo file dedicato
- Facile aggiungere nuove risorse senza ingigantire file esistenti
- File piccoli e focalizzati (< 200 linee)

### ✅ Manutenibilità
- Facile trovare handler specifici
- Modifiche isolate (cambi a User non toccano Product)
- Review del codice più semplici

### ✅ Team-friendly
- Più sviluppatori possono lavorare in parallelo
- Meno conflitti Git su file condivisi
- Ownership chiaro per risorsa

## 📝 File Esistenti

### `handlers/mod.rs`

Modulo principale che:
1. Esporta tutti i moduli handler
2. Definisce `AppError` (conversione `DomainError` → HTTP)
3. Re-esporta handler per uso semplificato

```rust
pub mod user_handlers;
// pub mod product_handlers;  // Aggiungi quando implementi

pub use user_handlers::*;

pub struct AppError(DomainError);
// ... implementazione conversione errori
```

### `handlers/user_handlers.rs`

Handler specifici per la risorsa User:

```rust
/// Handler per creare un nuovo utente
pub async fn create_user(...) -> Result<Json<UserDto>, AppError> {
    let use_case = CreateUserUseCase::new(repository.as_ref().clone());
    let user = use_case.execute(dto).await?;
    Ok(Json(user))
}

// get_user, list_users, update_user, delete_user
```

**Pattern Comune**:
1. Estrae parametri da Axum (`State`, `Path`, `Json`)
2. Crea il use case con repository
3. Esegue il use case
4. Ritorna risposta HTTP (o errore)

## ➕ Aggiungere Nuovi Handler

### Scenario: Aggiungere handler per Product

#### 1. Implementa il Domain Layer

```rust
// src/domain/entities/product.rs
pub struct Product {
    id: Uuid,
    name: String,
    price: Decimal,
    // ...
}
```

#### 2. Implementa Application Layer

```rust
// src/application/use_cases/create_product.rs
pub struct CreateProductUseCase<R: ProductRepository> {
    repository: R,
}
```

#### 3. Implementa Infrastructure Layer

```rust
// src/infrastructure/persistence/in_memory_product_repository.rs
pub struct InMemoryProductRepository {
    products: Arc<DashMap<Uuid, Product>>,
}
```

#### 4. Crea Handler File

**File**: `src/presentation/http/handlers/product_handlers.rs`

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::{
    CreateProductDto, CreateProductUseCase, ProductDto,
    // ... altri use cases
};
use crate::infrastructure::InMemoryProductRepository;

use super::AppError;

/// Handler per creare un nuovo prodotto
pub async fn create_product(
    State(repository): State<Arc<InMemoryProductRepository>>,
    Json(dto): Json<CreateProductDto>,
) -> Result<Json<ProductDto>, AppError> {
    let use_case = CreateProductUseCase::new(repository.as_ref().clone());
    let product = use_case.execute(dto).await?;
    Ok(Json(product))
}

/// Handler per ottenere un prodotto
pub async fn get_product(
    State(repository): State<Arc<InMemoryProductRepository>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProductDto>, AppError> {
    let use_case = GetProductUseCase::new(repository.as_ref().clone());
    let product = use_case.execute(id).await?;
    Ok(Json(product))
}

/// Handler per listare prodotti
pub async fn list_products(
    State(repository): State<Arc<InMemoryProductRepository>>,
) -> Result<Json<Vec<ProductDto>>, AppError> {
    let use_case = ListProductsUseCase::new(repository.as_ref().clone());
    let products = use_case.execute().await?;
    Ok(Json(products))
}

// ... altri handler
```

#### 5. Esporta nel Module

**File**: `src/presentation/http/handlers/mod.rs`

```rust
pub mod user_handlers;
pub mod product_handlers;  // ← Aggiungi questa riga

pub use user_handlers::*;
pub use product_handlers::*;  // ← Aggiungi questa riga

pub struct AppError(DomainError);
// ...
```

#### 6. Aggiungi Route

**File**: `src/presentation/http/routes.rs`

```rust
pub fn create_routes(
    user_repository: Arc<InMemoryUserRepository>,
    product_repository: Arc<InMemoryProductRepository>,  // ← Nuovo parametro
) -> Router {
    // User routes
    let user_routes = Router::new()
        .route("/api/users", post(handlers::create_user))
        .route("/api/users", get(handlers::list_users))
        .route("/api/users/:id", get(handlers::get_user))
        .route("/api/users/:id", put(handlers::update_user))
        .route("/api/users/:id", delete(handlers::delete_user))
        .with_state(user_repository);

    // Product routes ← Aggiungi questo blocco
    let product_routes = Router::new()
        .route("/api/products", post(handlers::create_product))
        .route("/api/products", get(handlers::list_products))
        .route("/api/products/:id", get(handlers::get_product))
        .route("/api/products/:id", put(handlers::update_product))
        .route("/api/products/:id", delete(handlers::delete_product))
        .with_state(product_repository);

    // Combina tutte le route
    user_routes.merge(product_routes)  // ← Merge delle route
}
```

#### 7. Aggiorna Main

**File**: `src/main.rs`

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... logging setup ...

    let user_repository = Arc::new(InMemoryUserRepository::new());
    let product_repository = Arc::new(InMemoryProductRepository::new());  // ← Nuovo repo

    let app = create_routes(user_repository, product_repository)  // ← Passa entrambi
        .layer(CorsLayer::permissive())
        .layer(tower_http::trace::TraceLayer::new_for_http());

    // ... rest of setup ...
}
```

## 🎯 Best Practices

### 1. Un File per Risorsa

✅ **Buono**: `user_handlers.rs`, `product_handlers.rs`, `order_handlers.rs`

❌ **Cattivo**: `handlers.rs` con tutto dentro

### 2. Handler Focalizzati

Ogni handler deve:
- Fare UNA cosa
- Essere < 20 linee
- Delegare logica ai use cases
- Non contenere business logic

```rust
// ✅ BUONO: Semplice e focalizzato
pub async fn create_user(
    State(repository): State<Arc<InMemoryUserRepository>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<UserDto>, AppError> {
    let use_case = CreateUserUseCase::new(repository.as_ref().clone());
    let user = use_case.execute(dto).await?;
    Ok(Json(user))
}

// ❌ CATTIVO: Business logic nell'handler
pub async fn create_user(...) -> Result<...> {
    // Validazione email
    if !dto.email.contains('@') { ... }
    
    // Check duplicati
    if repository.find_by_email(&dto.email).await?.is_some() { ... }
    
    // Crea utente
    let user = User::new(...);
    
    // Salva
    repository.save(user).await?;
    
    // Questa logica dovrebbe essere nel use case!
}
```

### 3. Naming Convention

```rust
// Operazione + Risorsa
create_user()    // POST /api/users
get_user()       // GET /api/users/:id
list_users()     // GET /api/users
update_user()    // PUT /api/users/:id
delete_user()    // DELETE /api/users/:id

// Per operazioni custom
activate_user()  // POST /api/users/:id/activate
search_users()   // GET /api/users/search
```

### 4. Error Handling Consistente

Tutti gli handler usano `AppError` che converte automaticamente:

```rust
DomainError::InvalidEmail → 400 Bad Request
DomainError::UserNotFound → 404 Not Found
DomainError::UserAlreadyExists → 409 Conflict
DomainError::ValidationError → 400 Bad Request
```

### 5. State Management

```rust
// ✅ BUONO: State per tipo specifico
State(repository): State<Arc<InMemoryUserRepository>>

// Se hai più dipendenze, crea un AppState
pub struct AppState {
    user_repo: Arc<InMemoryUserRepository>,
    product_repo: Arc<InMemoryProductRepository>,
    auth_service: Arc<dyn AuthenticationService>,
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<UserDto>, AppError> {
    let use_case = CreateUserUseCase::new(state.user_repo.as_ref().clone());
    // ...
}
```

## 📊 Esempi Reali

### Handler CRUD Completo

```rust
// src/presentation/http/handlers/product_handlers.rs

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::{
    CreateProductDto, DeleteProductUseCase, GetProductUseCase, 
    ListProductsUseCase, ProductDto, UpdateProductDto, UpdateProductUseCase,
};
use crate::infrastructure::InMemoryProductRepository;

use super::AppError;

pub async fn create_product(
    State(repository): State<Arc<InMemoryProductRepository>>,
    Json(dto): Json<CreateProductDto>,
) -> Result<Json<ProductDto>, AppError> {
    let use_case = CreateProductUseCase::new(repository.as_ref().clone());
    let product = use_case.execute(dto).await?;
    Ok(Json(product))
}

pub async fn get_product(
    State(repository): State<Arc<InMemoryProductRepository>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ProductDto>, AppError> {
    let use_case = GetProductUseCase::new(repository.as_ref().clone());
    let product = use_case.execute(id).await?;
    Ok(Json(product))
}

pub async fn list_products(
    State(repository): State<Arc<InMemoryProductRepository>>,
) -> Result<Json<Vec<ProductDto>>, AppError> {
    let use_case = ListProductsUseCase::new(repository.as_ref().clone());
    let products = use_case.execute().await?;
    Ok(Json(products))
}

pub async fn update_product(
    State(repository): State<Arc<InMemoryProductRepository>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateProductDto>,
) -> Result<Json<ProductDto>, AppError> {
    let use_case = UpdateProductUseCase::new(repository.as_ref().clone());
    let product = use_case.execute(id, dto).await?;
    Ok(Json(product))
}

pub async fn delete_product(
    State(repository): State<Arc<InMemoryProductRepository>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let use_case = DeleteProductUseCase::new(repository.as_ref().clone());
    use_case.execute(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
```

## 🔍 Template Incluso

Il progetto include `example_product_handlers.rs.template` che puoi copiare come punto di partenza per nuovi handler:

```bash
# Copia il template
cp src/presentation/http/handlers/example_product_handlers.rs.template \
   src/presentation/http/handlers/product_handlers.rs

# Decomenta il codice e personalizza
```

## 📚 Riassunto

1. **Un file per risorsa**: `user_handlers.rs`, `product_handlers.rs`, etc.
2. **Handler semplici**: Solo orchestrazione, niente business logic
3. **Esporta in mod.rs**: Rendi disponibili i nuovi handler
4. **Aggiungi route**: Configura endpoint in `routes.rs`
5. **Testa**: Scrivi test per ogni handler

Questa struttura ti permette di scalare facilmente da 1 a 100+ endpoint mantenendo il codice pulito e organizzato! 🚀
