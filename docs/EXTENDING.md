# Estendere l'Architettura

Questa guida mostra come estendere l'architettura con nuove funzionalità.

## 📦 Aggiungere una Nuova Entità

### 1. Creare l'Entità nel Domain Layer

```rust
// src/domain/entities/product.rs
pub struct Product {
    id: Uuid,
    name: String,
    price: Decimal,
    created_at: DateTime<Utc>,
}

impl Product {
    pub fn new(name: String, price: Decimal) -> Result<Self, DomainError> {
        // Business logic: validazione prezzo
        if price < Decimal::ZERO {
            return Err(DomainError::InvalidPrice);
        }
        
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            price,
            created_at: Utc::now(),
        })
    }
}
```

### 2. Definire il Repository Trait

```rust
// src/domain/repositories/product_repository.rs
#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn save(&self, product: Product) -> Result<Product, DomainError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Product>, DomainError>;
}
```

### 3. Creare i Use Cases

```rust
// src/application/use_cases/create_product.rs
pub struct CreateProductUseCase<R: ProductRepository> {
    repository: R,
}

impl<R: ProductRepository> CreateProductUseCase<R> {
    pub async fn execute(&self, dto: CreateProductDto) -> Result<ProductDto, DomainError> {
        let product = Product::new(dto.name, dto.price)?;
        let saved = self.repository.save(product).await?;
        Ok(ProductDto::from(saved))
    }
}
```

### 4. Implementare il Repository

```rust
// src/infrastructure/persistence/in_memory_product_repository.rs
pub struct InMemoryProductRepository {
    products: Arc<DashMap<Uuid, Product>>,
}

#[async_trait]
impl ProductRepository for InMemoryProductRepository {
    async fn save(&self, product: Product) -> Result<Product, DomainError> {
        self.products.insert(*product.id(), product.clone());
        Ok(product)
    }
}
```

### 5. Aggiungere gli Handler HTTP

```rust
// src/presentation/http/handlers.rs
pub async fn create_product(
    State(repository): State<Arc<InMemoryProductRepository>>,
    Json(dto): Json<CreateProductDto>,
) -> Result<Json<ProductDto>, AppError> {
    let use_case = CreateProductUseCase::new(repository.as_ref().clone());
    let product = use_case.execute(dto).await?;
    Ok(Json(product))
}
```

## 🔄 Sostituire l'Implementazione del Database

### Da In-Memory a PostgreSQL

#### 1. Aggiungi le dipendenze

```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono"] }
```

#### 2. Crea l'implementazione PostgreSQL

```rust
// src/infrastructure/persistence/postgres_user_repository.rs
use sqlx::PgPool;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn save(&self, user: User) -> Result<User, DomainError> {
        sqlx::query!(
            r#"
            INSERT INTO users (id, email, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id(),
            user.email().value(),
            user.name(),
            user.created_at(),
            user.updated_at(),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        Ok(user)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError> {
        let row = sqlx::query!(
            r#"
            SELECT id, email, name, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::ValidationError(e.to_string()))?;

        match row {
            Some(row) => {
                let email = Email::new(row.email)?;
                let user = User::reconstruct(
                    row.id,
                    email,
                    row.name,
                    row.created_at,
                    row.updated_at,
                );
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    // ... implementa gli altri metodi
}
```

#### 3. Aggiorna main.rs

```rust
// Scegli il repository basandoti sulla configurazione
let repository = if config.use_postgres {
    let pool = PgPool::connect(&config.database_url).await?;
    Arc::new(PostgresUserRepository::new(pool)) as Arc<dyn UserRepository>
} else {
    Arc::new(InMemoryUserRepository::new()) as Arc<dyn UserRepository>
};
```

## 🎨 Aggiungere Validazioni Business

### Value Objects con Validazioni Complesse

```rust
// src/domain/value_objects/password.rs
pub struct Password {
    hashed: String,
}

impl Password {
    pub fn new(plain: &str) -> Result<Self, DomainError> {
        // Business rule: password deve essere sicura
        if plain.len() < 8 {
            return Err(DomainError::WeakPassword);
        }
        
        if !plain.chars().any(|c| c.is_uppercase()) {
            return Err(DomainError::WeakPassword);
        }
        
        if !plain.chars().any(|c| c.is_numeric()) {
            return Err(DomainError::WeakPassword);
        }
        
        // Hash della password
        let hashed = hash_password(plain)?;
        
        Ok(Self { hashed })
    }
    
    pub fn verify(&self, plain: &str) -> bool {
        verify_password(plain, &self.hashed)
    }
}
```

## 🔐 Aggiungere Autenticazione

### 1. Creare un Authentication Service (Port)

```rust
// src/application/ports/authentication_service.rs
#[async_trait]
pub trait AuthenticationService: Send + Sync {
    async fn authenticate(&self, email: &Email, password: &str) 
        -> Result<String, DomainError>; // Returns JWT token
    
    async fn validate_token(&self, token: &str) 
        -> Result<Uuid, DomainError>; // Returns user_id
}
```

### 2. Implementare il Service

```rust
// src/infrastructure/services/jwt_authentication_service.rs
pub struct JwtAuthenticationService {
    user_repository: Arc<dyn UserRepository>,
    secret_key: String,
}

#[async_trait]
impl AuthenticationService for JwtAuthenticationService {
    async fn authenticate(&self, email: &Email, password: &str) 
        -> Result<String, DomainError> 
    {
        let user = self.user_repository
            .find_by_email(email)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;
        
        // Verifica password...
        
        // Genera JWT token
        let token = generate_jwt(user.id(), &self.secret_key)?;
        
        Ok(token)
    }
}
```

### 3. Usare nei Use Cases

```rust
// src/application/use_cases/login_user.rs
pub struct LoginUserUseCase<A: AuthenticationService> {
    auth_service: A,
}

impl<A: AuthenticationService> LoginUserUseCase<A> {
    pub async fn execute(&self, dto: LoginDto) -> Result<TokenDto, DomainError> {
        let email = Email::new(dto.email)?;
        let token = self.auth_service
            .authenticate(&email, &dto.password)
            .await?;
        
        Ok(TokenDto { token })
    }
}
```

## 🧪 Testing Strategy per Nuove Features

### Mock dei Repository

```rust
#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use mockall::mock;

    mock! {
        pub UserRepo {}
        
        #[async_trait]
        impl UserRepository for UserRepo {
            async fn save(&self, user: User) -> Result<User, DomainError>;
            async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError>;
        }
    }

    #[tokio::test]
    async fn test_create_user_with_mock() {
        let mut mock_repo = MockUserRepo::new();
        
        mock_repo
            .expect_exists_by_email()
            .returning(|_| Ok(false));
            
        mock_repo
            .expect_save()
            .returning(|user| Ok(user));
        
        let use_case = CreateUserUseCase::new(mock_repo);
        // ... test
    }
}
```

## 📊 Aggiungere Eventi di Dominio

### 1. Definire gli Eventi

```rust
// src/domain/events/user_events.rs
#[derive(Debug, Clone)]
pub enum UserEvent {
    UserCreated { user_id: Uuid, email: String },
    UserUpdated { user_id: Uuid },
    UserDeleted { user_id: Uuid },
}
```

### 2. Event Publisher Port

```rust
// src/application/ports/event_publisher.rs
#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: UserEvent) -> Result<(), DomainError>;
}
```

### 3. Usare nei Use Cases

```rust
pub struct CreateUserUseCase<R: UserRepository, E: EventPublisher> {
    repository: R,
    event_publisher: E,
}

impl<R: UserRepository, E: EventPublisher> CreateUserUseCase<R, E> {
    pub async fn execute(&self, dto: CreateUserDto) -> Result<UserDto, DomainError> {
        let user = User::new(...);
        let saved = self.repository.save(user).await?;
        
        // Pubblica evento
        self.event_publisher
            .publish(UserEvent::UserCreated {
                user_id: *saved.id(),
                email: saved.email().value().to_string(),
            })
            .await?;
        
        Ok(UserDto::from(saved))
    }
}
```

## 🌐 Aggiungere API Versioning

```rust
// src/presentation/http/routes.rs
pub fn create_routes_v1(repository: Arc<InMemoryUserRepository>) -> Router {
    Router::new()
        .route("/api/v1/users", post(handlers::v1::create_user))
        // ...
}

pub fn create_routes_v2(repository: Arc<InMemoryUserRepository>) -> Router {
    Router::new()
        .route("/api/v2/users", post(handlers::v2::create_user))
        // ...
}

// In main.rs
let app = Router::new()
    .nest("/", create_routes_v1(repository.clone()))
    .nest("/", create_routes_v2(repository.clone()));
```

## 📝 Best Practices

1. **Mantieni il Domain puro**: Nessuna dipendenza esterna nel domain layer
2. **Use Cases single-purpose**: Ogni use case fa UNA cosa
3. **Test all levels**: Unit test per ogni layer, integration test end-to-end
4. **Dependency Injection**: Inietta dipendenze tramite costruttori
5. **Error Handling**: Usa tipi di errore specifici del dominio
6. **Value Objects**: Usa value objects per garantire invarianti
7. **Repository per Aggregate**: Un repository per aggregate root
8. **Async everywhere**: Usa async/await per I/O bound operations
