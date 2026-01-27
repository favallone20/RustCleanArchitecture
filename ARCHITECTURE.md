# Architettura del Progetto

## 🏛️ Panoramica della Clean Architecture

Questo progetto implementa la **Clean Architecture** (anche conosciuta come Onion Architecture o Hexagonal Architecture) in Rust. L'obiettivo principale è creare un sistema che sia:

- **Indipendente dai framework**: La logica di business non dipende da librerie esterne
- **Testabile**: Il business logic può essere testato senza UI, Database, o altri servizi esterni
- **Indipendente dalla UI**: La UI può cambiare facilmente senza modificare il resto del sistema
- **Indipendente dal Database**: Puoi sostituire PostgreSQL con MongoDB senza modificare la logica di business
- **Indipendente da agenti esterni**: La logica di business non sa nulla del mondo esterno

## 📐 Struttura dei Layer

### 1. Domain Layer (Innermost Layer)

**Percorso**: `src/domain/`

**Responsabilità**:
- Contiene le **entità di business** e le **regole di business fondamentali**
- Definisce i **value objects** che garantiscono invarianti
- Definisce le **interfacce dei repository** (ports)
- **Non ha dipendenze esterne**

**Componenti**:
```
domain/
├── entities/          # Entità di business (User)
├── value_objects/     # Value Objects (Email)
├── repositories/      # Trait dei repository (UserRepository)
└── errors/           # Errori del dominio (DomainError)
```

**Principi**:
- Le entità contengono logica di business pura
- I value objects garantiscono validità (es: Email sempre valida)
- I repository trait definiscono COSA fare, non COME farlo
- Nessuna dipendenza da database, HTTP, o framework

**Esempio**:
```rust
// L'entità User contiene logica di business
impl User {
    pub fn update_name(&mut self, name: String) {
        self.name = name;
        self.updated_at = Utc::now(); // Regola di business
    }
}

// Email è un Value Object che garantisce validità
let email = Email::new("invalid").unwrap_err();
```

### 2. Application Layer

**Percorso**: `src/application/`

**Responsabilità**:
- Contiene i **Use Cases** (casi d'uso dell'applicazione)
- Orchestrare il flusso di dati tra presentation e domain
- Implementa la logica applicativa (non di business)
- Dipende solo dal Domain Layer

**Componenti**:
```
application/
├── use_cases/        # Use Cases (CreateUser, GetUser, etc.)
└── dto/             # Data Transfer Objects
```

**Principi**:
- Ogni use case fa UNA cosa specifica
- I use case dipendono dalle astrazioni (trait) non dalle implementazioni
- I DTO convertono tra domain entities e presentation layer
- Nessuna conoscenza di HTTP, database, o framework

**Esempio**:
```rust
pub struct CreateUserUseCase<R: UserRepository> {
    repository: R, // Dipende dall'astrazione, non dall'implementazione
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub async fn execute(&self, dto: CreateUserDto) -> Result<UserDto, DomainError> {
        // 1. Validazione tramite Value Objects
        let email = Email::new(dto.email)?;
        
        // 2. Verifica business rules
        if self.repository.exists_by_email(&email).await? {
            return Err(DomainError::UserAlreadyExists(...));
        }
        
        // 3. Crea entità
        let user = User::new(email, dto.name);
        
        // 4. Persiste tramite repository
        let saved = self.repository.save(user).await?;
        
        Ok(UserDto::from(saved))
    }
}
```

### 3. Infrastructure Layer

**Percorso**: `src/infrastructure/`

**Responsabilità**:
- **Implementa** le interfacce definite nel Domain Layer
- Gestisce persistenza (database)
- Gestisce servizi esterni
- Gestisce configurazione

**Componenti**:
```
infrastructure/
├── persistence/      # Implementazioni dei repository
│   └── in_memory_user_repository.rs
└── config/          # Configurazione dell'app
```

**Principi**:
- Implementa i trait definiti nel domain
- Dipende dal domain layer (dependency inversion)
- Può essere facilmente sostituito (es: da in-memory a PostgreSQL)
- Contiene i dettagli di implementazione

**Esempio**:
```rust
// Implementa il trait UserRepository definito nel domain
#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: User) -> Result<User, DomainError> {
        // Dettagli di implementazione specifici
        self.users.insert(*user.id(), user.clone());
        Ok(user)
    }
}

// Puoi facilmente creare PostgresUserRepository che implementa
// lo stesso trait senza modificare il domain o application layer!
```

### 4. Presentation Layer (Outermost Layer)

**Percorso**: `src/presentation/`

**Responsabilità**:
- Gestisce le richieste HTTP
- Converte dati HTTP in DTO
- Gestisce errori e risposte HTTP
- Routing

**Componenti**:
```
presentation/
└── http/
    ├── handlers.rs   # Handler HTTP
    └── routes.rs     # Definizione route
```

**Principi**:
- Dipende solo dagli use cases
- Converte tra HTTP e domain
- Gestisce concerns specifici del web (status code, serialization)

**Esempio**:
```rust
pub async fn create_user(
    State(repository): State<Arc<InMemoryUserRepository>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<UserDto>, AppError> {
    // 1. Crea il use case con dependency injection
    let use_case = CreateUserUseCase::new(repository.as_ref().clone());
    
    // 2. Esegue il use case
    let user = use_case.execute(dto).await?;
    
    // 3. Ritorna HTTP response
    Ok(Json(user))
}
```

## 🔄 Flusso dei Dati

```
HTTP Request
    ↓
[Presentation Layer] - Handler riceve request
    ↓
[Application Layer] - Use Case esegue logica applicativa
    ↓
[Domain Layer] - Entità e business logic
    ↓
[Infrastructure Layer] - Repository persiste dati
    ↓
[Domain Layer] - Ritorna entità
    ↓
[Application Layer] - Converte in DTO
    ↓
[Presentation Layer] - Converte in HTTP Response
    ↓
HTTP Response
```

## 🎯 Dependency Rule

**Regola fondamentale**: Le dipendenze puntano sempre VERSO L'INTERNO

```
Presentation → Application → Domain ← Infrastructure
```

- **Presentation** dipende da Application
- **Application** dipende da Domain
- **Infrastructure** dipende da Domain (Dependency Inversion!)
- **Domain** non dipende da nessuno

## 💉 Dependency Injection

Il progetto usa la Dependency Injection per invertire le dipendenze:

```rust
// In main.rs
let repository = Arc::new(InMemoryUserRepository::new());
let app = create_routes(repository);

// Il repository viene iniettato negli handler
pub async fn create_user(
    State(repository): State<Arc<InMemoryUserRepository>>,
    // ...
) {
    let use_case = CreateUserUseCase::new(repository.as_ref().clone());
    // ...
}
```

## 🔧 Sostituire l'Implementazione

Uno dei vantaggi principali è la facilità di sostituire implementazioni:

### Esempio: Passare da In-Memory a PostgreSQL

1. Crea `PostgresUserRepository` che implementa `UserRepository`
2. In `main.rs`, cambia una riga:

```rust
// Prima
let repository = Arc::new(InMemoryUserRepository::new());

// Dopo
let repository = Arc::new(PostgresUserRepository::new(pool).await?);
```

3. **Nessun'altra modifica necessaria!** Il domain, application e presentation layer rimangono identici.

## 🧪 Testing Strategy

### Unit Tests
Ogni layer ha i suoi unit tests:
- **Domain**: Test delle entità e value objects
- **Application**: Test dei use cases con mock repository
- **Infrastructure**: Test delle implementazioni dei repository
- **Presentation**: Test degli handler HTTP

### Integration Tests
Test end-to-end in `tests/integration_test.rs`:
- Testano l'intero flusso da use case a repository
- Verificano che tutti i layer collaborino correttamente

## 🚀 Vantaggi di questa Architettura

1. **Manutenibilità**: Ogni layer ha responsabilità chiare
2. **Testabilità**: Puoi testare la logica di business in isolamento
3. **Flessibilità**: Puoi cambiare database, framework, UI senza toccare il core
4. **Scalabilità**: Facile aggiungere nuovi use cases o feature
5. **Onboarding**: Struttura chiara per nuovi sviluppatori

## 📚 Risorse

- [The Clean Architecture - Robert C. Martin](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Hexagonal Architecture](https://alistair.cockburn.us/hexagonal-architecture/)
- [Domain-Driven Design](https://martinfowler.com/bliki/DomainDrivenDesign.html)
