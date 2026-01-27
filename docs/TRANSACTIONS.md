# Transaction Support - Unit of Work Pattern

## Overview

Il pattern **Unit of Work** mantiene una lista di oggetti affettati da una transazione business e coordina la scrittura delle modifiche e la risoluzione di problemi di concorrenza.

## Perché serve?

Nel sistema attuale usiamo `InMemoryUserRepository` che non ha bisogno di transazioni. Ma quando migrerai a un database reale (PostgreSQL, MySQL, etc.), avrai bisogno di gestire transazioni per garantire la consistenza dei dati.

## Architettura

```
┌─────────────────┐
│  Use Case       │
├─────────────────┤
│ 1. begin()      │
│ 2. execute()    │
│ 3. commit()     │ ← Success
│    or           │
│    rollback()   │ ← Error
└─────────────────┘
        │
        ↓
┌─────────────────┐
│  UnitOfWork     │ (Trait)
├─────────────────┤
│ - begin()       │
│ - commit()      │
│ - rollback()    │
│ - repository()  │
└─────────────────┘
        │
        ↓
┌─────────────────────────────┐
│ PostgresUnitOfWork (Impl)  │
├─────────────────────────────┤
│ - Transaction<Postgres>    │
│ - PostgresUserRepository   │
└─────────────────────────────┘
```

## Implementazione

### 1. UnitOfWork Trait

Crea `src/domain/repositories/unit_of_work.rs`:

\`\`\`rust
use async_trait::async_trait;
use crate::domain::DomainError;
use crate::domain::repositories::UserRepository;

/// Unit of Work Pattern per gestire transazioni
#[async_trait]
pub trait UnitOfWork: Send + Sync {
    type Repository: UserRepository;

    /// Inizia una nuova transazione
    async fn begin(&mut self) -> Result<(), DomainError>;

    /// Commit della transazione corrente
    async fn commit(&mut self) -> Result<(), DomainError>;

    /// Rollback della transazione corrente
    async fn rollback(&mut self) -> Result<(), DomainError>;

    /// Ottieni il repository nell'ambito della transazione
    fn repository(&self) -> &Self::Repository;
}
\`\`\`

### 2. Implementazione PostgreSQL

Esempio di come implementare con SQLx:

\`\`\`rust
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PostgresUnitOfWork {
    pool: PgPool,
    transaction: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
    repository: PostgresUserRepository,
}

impl PostgresUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        let repository = PostgresUserRepository::new(pool.clone());
        Self {
            pool,
            transaction: Arc::new(Mutex::new(None)),
            repository,
        }
    }
}

#[async_trait]
impl UnitOfWork for PostgresUnitOfWork {
    type Repository = PostgresUserRepository;

    async fn begin(&mut self) -> Result<(), DomainError> {
        let tx = self.pool.begin().await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        
        *self.transaction.lock().await = Some(tx);
        tracing::debug!("Transaction started");
        Ok(())
    }

    async fn commit(&mut self) -> Result<(), DomainError> {
        if let Some(tx) = self.transaction.lock().await.take() {
            tx.commit().await
                .map_err(|e| DomainError::RepositoryError(e.to_string()))?;
            tracing::debug!("Transaction committed");
        }
        Ok(())
    }

    async fn rollback(&mut self) -> Result<(), DomainError> {
        if let Some(tx) = self.transaction.lock().await.take() {
            tx.rollback().await
                .map_err(|e| DomainError::RepositoryError(e.to_string()))?;
            tracing::warn!("Transaction rolled back");
        }
        Ok(())
    }

    fn repository(&self) -> &Self::Repository {
        &self.repository
    }
}
\`\`\`

### 3. Use Case con Transazione

\`\`\`rust
pub struct CreateUserWithTransactionUseCase<UoW: UnitOfWork> {
    unit_of_work: UoW,
}

impl<UoW: UnitOfWork> CreateUserWithTransactionUseCase<UoW> {
    pub fn new(unit_of_work: UoW) -> Self {
        Self { unit_of_work }
    }

    pub async fn execute(&mut self, dto: CreateUserDto) -> Result<UserDto, DomainError> {
        // Inizia transazione
        self.unit_of_work.begin().await?;

        let result = async {
            let email = Email::new(dto.email)?;
            let name = Name::new(dto.name)?;

            let repository = self.unit_of_work.repository();

            if repository.exists_by_email(&email).await? {
                return Err(DomainError::UserAlreadyExists(email.value().to_string()));
            }

            let user = User::new(email, name);
            let saved_user = repository.save(user).await?;

            Ok(UserDto::from(saved_user))
        }.await;

        match result {
            Ok(user_dto) => {
                // Commit se tutto ok
                self.unit_of_work.commit().await?;
                tracing::info!("User created with transaction");
                Ok(user_dto)
            }
            Err(err) => {
                // Rollback in caso di errore
                self.unit_of_work.rollback().await?;
                tracing::error!("Transaction rolled back due to error");
                Err(err)
            }
        }
    }
}
\`\`\`

## Quando usare le transazioni?

### ✅ Usa transazioni per:
- Operazioni che modificano multipli aggregati
- Operazioni critiche che richiedono ACID guarantees
- Operazioni che devono essere atomiche

### ❌ Non servono transazioni per:
- Semplici query di lettura
- Operazioni su un singolo aggregato
- Repository in-memory (già atomico)

## Esempi d'uso

### Trasferimento fondi (richiede transazione)

\`\`\`rust
pub async fn transfer_funds(
    &mut self,
    from_account: Uuid,
    to_account: Uuid,
    amount: Decimal,
) -> Result<(), DomainError> {
    self.unit_of_work.begin().await?;

    let result = async {
        let account_repo = self.unit_of_work.account_repository();
        
        // Preleva da un account
        account_repo.withdraw(from_account, amount).await?;
        
        // Deposita nell'altro
        account_repo.deposit(to_account, amount).await?;
        
        Ok(())
    }.await;

    match result {
        Ok(_) => {
            self.unit_of_work.commit().await?;
            Ok(())
        }
        Err(err) => {
            self.unit_of_work.rollback().await?;
            Err(err)
        }
    }
}
\`\`\`

## Migrazione Step-by-Step

### Step 1: Definisci il trait
Aggiungi `UnitOfWork` trait nel domain layer

### Step 2: Implementa per In-Memory (no-op)
Per compatibilità, crea una versione in-memory che fa no-op

### Step 3: Implementa per database reale
Quando sei pronto, implementa con SQLx/Diesel

### Step 4: Aggiorna use cases critici
Migra i use cases che beneficiano dalle transazioni

### Step 5: Testing
Testa rollback e commit scenarios

## Dependencies necessarie

\`\`\`toml
[dependencies]
# PostgreSQL
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }

# O MySQL
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "mysql", "uuid", "chrono"] }

# O Diesel
diesel = { version = "2.1", features = ["postgres", "uuid", "chrono"] }
diesel-async = "0.4"
\`\`\`

## Best Practices

1. **Keep transactions short**: Minimize il tempo tra begin() e commit()
2. **Handle errors gracefully**: Sempre rollback in caso di errore
3. **Log appropriately**: Log start, commit e rollback per debugging
4. **Use connection pooling**: Non creare nuove connessioni per ogni transazione
5. **Avoid nested transactions**: Tieni le transazioni piatte quando possibile

## Testing

\`\`\`rust
#[tokio::test]
async fn test_transaction_rollback() {
    let mut uow = PostgresUnitOfWork::new(pool.clone());
    
    uow.begin().await.unwrap();
    
    // Operazione che fallisce
    let result = uow.repository().save(invalid_user).await;
    assert!(result.is_err());
    
    // Rollback
    uow.rollback().await.unwrap();
    
    // Verifica che nulla sia stato salvato
    let users = uow.repository().find_all().await.unwrap();
    assert_eq!(users.len(), 0);
}
\`\`\`

## Conclusione

Il pattern Unit of Work è essenziale per applicazioni che usano database relazionali. La struttura attuale del progetto è già pronta per supportarlo - basta implementare il trait quando migri da in-memory a un database reale.
