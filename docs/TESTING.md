# Testing Guide

Questa guida spiega la strategia di testing utilizzata nel progetto.

## 🎯 Piramide dei Test

```
        /\
       /  \        E2E Tests (pochi)
      /----\
     /      \      Integration Tests (alcuni)
    /--------\
   /          \    Unit Tests (molti)
  /____________\
```

## 🔬 Unit Tests

### Domain Layer Tests

Test delle entità e value objects senza dipendenze esterne.

#### Testing Entità

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email.clone(), "Test User".to_string());

        assert_eq!(user.email(), &email);
        assert_eq!(user.name(), "Test User");
    }

    #[test]
    fn test_user_business_logic() {
        let email = Email::new("test@example.com").unwrap();
        let mut user = User::new(email, "Old Name".to_string());
        let old_updated_at = *user.updated_at();

        user.update_name("New Name".to_string());

        assert_eq!(user.name(), "New Name");
        assert!(user.updated_at() > &old_updated_at); // Business rule
    }
}
```

#### Testing Value Objects

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let result = Email::new("test@example.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let result = Email::new("invalid-email");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::InvalidEmail(_)));
    }

    #[test]
    fn test_email_validation_rules() {
        // Test vari scenari di validazione
        assert!(Email::new("user@domain.com").is_ok());
        assert!(Email::new("user+tag@domain.com").is_ok());
        assert!(Email::new("@domain.com").is_err());
        assert!(Email::new("user@").is_err());
        assert!(Email::new("user").is_err());
    }
}
```

### Application Layer Tests

Test dei use cases con repository reali o mock.

#### Testing con Repository In-Memory

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::persistence::InMemoryUserRepository;

    #[tokio::test]
    async fn test_create_user_success() {
        // Arrange
        let repository = InMemoryUserRepository::new();
        let use_case = CreateUserUseCase::new(repository);
        let dto = CreateUserDto {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        // Arrange
        let repository = InMemoryUserRepository::new();
        let use_case = CreateUserUseCase::new(repository);
        let dto = CreateUserDto {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Act
        use_case.execute(dto.clone()).await.unwrap();
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::UserAlreadyExists(email) => {
                assert_eq!(email, "test@example.com");
            }
            _ => panic!("Expected UserAlreadyExists error"),
        }
    }
}
```

#### Testing con Mock Repository

Per test più isolati, usa mockall:

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
            async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError>;
        }
    }

    #[tokio::test]
    async fn test_create_user_with_mock() {
        // Arrange
        let mut mock_repo = MockUserRepo::new();
        
        mock_repo
            .expect_exists_by_email()
            .with(eq(Email::new("test@example.com").unwrap()))
            .times(1)
            .returning(|_| Ok(false));
            
        mock_repo
            .expect_save()
            .times(1)
            .returning(|user| Ok(user));
        
        let use_case = CreateUserUseCase::new(mock_repo);
        let dto = CreateUserDto {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_repository_error_handling() {
        // Arrange
        let mut mock_repo = MockUserRepo::new();
        
        mock_repo
            .expect_exists_by_email()
            .returning(|_| Err(DomainError::ValidationError("DB Error".to_string())));
        
        let use_case = CreateUserUseCase::new(mock_repo);
        let dto = CreateUserDto {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Act
        let result = use_case.execute(dto).await;

        // Assert
        assert!(result.is_err());
    }
}
```

### Infrastructure Layer Tests

Test delle implementazioni dei repository.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_save_and_find() {
        let repo = InMemoryUserRepository::new();
        let email = Email::new("test@example.com").unwrap();
        let user = User::new(email, "Test".to_string());
        let user_id = *user.id();

        repo.save(user).await.unwrap();

        let found = repo.find_by_id(&user_id).await.unwrap();
        assert!(found.is_some());
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let repo = InMemoryUserRepository::new();
        let repo1 = repo.clone();
        let repo2 = repo.clone();

        // Testa thread-safety
        let handle1 = tokio::spawn(async move {
            let user = User::new(
                Email::new("user1@test.com").unwrap(),
                "User 1".to_string(),
            );
            repo1.save(user).await
        });

        let handle2 = tokio::spawn(async move {
            let user = User::new(
                Email::new("user2@test.com").unwrap(),
                "User 2".to_string(),
            );
            repo2.save(user).await
        });

        let results = tokio::try_join!(handle1, handle2);
        assert!(results.is_ok());
    }
}
```

## 🔗 Integration Tests

Test che verificano l'integrazione tra più layer.

### File: `tests/integration_test.rs`

```rust
use clean_architecture_rust::{
    CreateUserDto, CreateUserUseCase, GetUserUseCase, InMemoryUserRepository,
};

#[tokio::test]
async fn test_complete_user_lifecycle() {
    // Setup
    let repository = InMemoryUserRepository::new();

    // Create
    let create_use_case = CreateUserUseCase::new(repository.clone());
    let created = create_use_case
        .execute(CreateUserDto {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        })
        .await
        .expect("Failed to create user");

    // Read
    let get_use_case = GetUserUseCase::new(repository.clone());
    let retrieved = get_use_case
        .execute(created.id)
        .await
        .expect("Failed to get user");

    // Assert
    assert_eq!(created.id, retrieved.id);
    assert_eq!(created.email, retrieved.email);
}
```

## 🌐 HTTP/API Tests

Test degli endpoint HTTP (opzionale, richiede dipendenze aggiuntive).

### Con `axum-test`

```toml
[dev-dependencies]
axum-test = "14.0"
```

```rust
use axum_test::TestServer;

#[tokio::test]
async fn test_create_user_endpoint() {
    // Setup
    let repository = Arc::new(InMemoryUserRepository::new());
    let app = create_routes(repository);
    let server = TestServer::new(app).unwrap();

    // Act
    let response = server
        .post("/api/users")
        .json(&serde_json::json!({
            "email": "test@example.com",
            "name": "Test User"
        }))
        .await;

    // Assert
    response.assert_status_ok();
    let user: UserDto = response.json();
    assert_eq!(user.email, "test@example.com");
}

#[tokio::test]
async fn test_create_user_invalid_email() {
    let repository = Arc::new(InMemoryUserRepository::new());
    let app = create_routes(repository);
    let server = TestServer::new(app).unwrap();

    let response = server
        .post("/api/users")
        .json(&serde_json::json!({
            "email": "invalid-email",
            "name": "Test User"
        }))
        .await;

    response.assert_status_bad_request();
}
```

## 🎭 Test Coverage

### Eseguire i Test con Coverage

```bash
# Installa tarpaulin
cargo install cargo-tarpaulin

# Esegui test con coverage
cargo tarpaulin --out Html --output-dir coverage
```

### Obiettivi di Coverage

- **Domain Layer**: > 95% (business logic critica)
- **Application Layer**: > 90% (use cases)
- **Infrastructure Layer**: > 80% (implementazioni)
- **Presentation Layer**: > 70% (handler HTTP)

## 🚀 Eseguire i Test

### Tutti i test

```bash
cargo test
```

### Test specifici

```bash
# Solo unit tests
cargo test --lib

# Solo integration tests
cargo test --test integration_test

# Test di un modulo specifico
cargo test domain::entities::user

# Test con output dettagliato
cargo test -- --nocapture

# Test in parallelo con più thread
cargo test -- --test-threads=4
```

### Test con logging

```bash
# Con log di debug
RUST_LOG=debug cargo test -- --nocapture

# Test specifico con log
RUST_LOG=debug cargo test test_create_user -- --nocapture
```

## 📊 Test Performance

### Benchmark con Criterion

```toml
[dev-dependencies]
criterion = "0.5"

[[bench]]
name = "user_creation"
harness = false
```

```rust
// benches/user_creation.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_user_creation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let repository = InMemoryUserRepository::new();
    let use_case = CreateUserUseCase::new(repository);

    c.bench_function("create_user", |b| {
        b.iter(|| {
            rt.block_on(async {
                use_case
                    .execute(CreateUserDto {
                        email: format!("test{}@example.com", uuid::Uuid::new_v4()),
                        name: "Test User".to_string(),
                    })
                    .await
            })
        })
    });
}

criterion_group!(benches, benchmark_user_creation);
criterion_main!(benches);
```

## 🔍 Test-Driven Development (TDD)

### Red-Green-Refactor Cycle

1. **Red**: Scrivi un test che fallisce

```rust
#[tokio::test]
async fn test_user_cannot_have_empty_name() {
    let repository = InMemoryUserRepository::new();
    let use_case = CreateUserUseCase::new(repository);
    
    let result = use_case
        .execute(CreateUserDto {
            email: "test@example.com".to_string(),
            name: "".to_string(), // Nome vuoto
        })
        .await;
    
    assert!(result.is_err());
}
```

2. **Green**: Implementa il codice minimo per far passare il test

```rust
pub async fn execute(&self, dto: CreateUserDto) -> Result<UserDto, DomainError> {
    if dto.name.trim().is_empty() {
        return Err(DomainError::InvalidUserName("Name cannot be empty".to_string()));
    }
    // ... resto del codice
}
```

3. **Refactor**: Migliora il codice mantenendo i test verdi

## 📝 Best Practices

1. **AAA Pattern**: Arrange-Act-Assert
2. **Test Isolation**: Ogni test deve essere indipendente
3. **Naming**: Usa nomi descrittivi (`test_create_user_with_invalid_email`)
4. **One Assertion**: Preferibilmente un'asserzione logica per test
5. **Fast Tests**: I test devono essere veloci (< 1s per unit test)
6. **Deterministic**: I test devono sempre dare lo stesso risultato
7. **Coverage**: Punta a coverage alto ma significativo
8. **Mock Judiciously**: Usa mock solo quando necessario

## 🐛 Debugging Tests

```bash
# Esegui un singolo test con debug
RUST_LOG=debug cargo test test_name -- --nocapture --test-threads=1

# Con rust-gdb
rust-gdb --args target/debug/deps/clean_architecture_rust-XXX test_name

# Con lldb
rust-lldb target/debug/deps/clean_architecture_rust-XXX -- test_name
```
