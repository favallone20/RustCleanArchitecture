# 🚀 Quick Start Guide

Guida rapida per iniziare a lavorare con questo progetto di Clean Architecture in Rust.

## 📋 Prerequisiti

- Rust 1.75 o superiore
- Cargo (installato con Rust)

## ⚡ Avvio Rapido

### 1. Compilare il progetto

```bash
cargo build
```

### 2. Eseguire i test

```bash
cargo test
```

Output atteso: ✅ **30 test passati** (26 unit tests + 4 integration tests)

### 3. Avviare il server

```bash
cargo run
```

Il server si avvierà su `http://127.0.0.1:3000`

### 4. Testare l'API

In un nuovo terminale:

```bash
# Creare un utente
curl -X POST http://127.0.0.1:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"email":"mario.rossi@example.com","name":"Mario Rossi"}'

# Risposta attesa:
# {"id":"...","email":"mario.rossi@example.com","name":"Mario Rossi","created_at":"...","updated_at":"..."}
```

## 🎯 Comandi Principali

### Sviluppo

```bash
# Compilare
cargo build

# Compilare in release (ottimizzato)
cargo build --release

# Eseguire
cargo run

# Verificare il codice senza compilare (più veloce)
cargo check

# Formattare il codice
cargo fmt

# Linting
cargo clippy
```

### Testing

```bash
# Tutti i test
cargo test

# Solo unit test
cargo test --lib

# Solo integration test
cargo test --test integration_test

# Test con output dettagliato
cargo test -- --nocapture

# Test specifico
cargo test test_create_user
```

### Con Make (opzionale)

```bash
# Vedere tutti i comandi disponibili
make help

# Build e test
make all

# Verifiche di qualità del codice
make dev-check

# Eseguire
make run
```

## 📚 Struttura del Progetto

```
src/
├── domain/              # Layer 1: Business Logic
│   ├── entities/        # User entity
│   ├── value_objects/   # Email value object
│   ├── repositories/    # Repository traits
│   └── errors/          # Domain errors
│
├── application/         # Layer 2: Use Cases
│   ├── use_cases/       # CreateUser, GetUser, etc.
│   └── dto/             # Data Transfer Objects
│
├── infrastructure/      # Layer 3: Implementation
│   ├── persistence/     # In-memory repository
│   └── config/          # App configuration
│
└── presentation/        # Layer 4: HTTP API
    └── http/            # Handlers and routes
```

## 🔍 API Endpoints

| Metodo | Endpoint | Descrizione |
|--------|----------|-------------|
| POST | `/api/users` | Crea un nuovo utente |
| GET | `/api/users` | Lista tutti gli utenti |
| GET | `/api/users/:id` | Ottieni utente per ID |
| PUT | `/api/users/:id` | Aggiorna utente |
| DELETE | `/api/users/:id` | Elimina utente |

## 📖 Esempi API

### Creare un utente

```bash
curl -X POST http://127.0.0.1:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "name": "Test User"
  }'
```

### Ottenere un utente

```bash
curl http://127.0.0.1:3000/api/users/{user_id}
```

### Listrare tutti gli utenti

```bash
curl http://127.0.0.1:3000/api/users
```

### Aggiornare un utente

```bash
curl -X PUT http://127.0.0.1:3000/api/users/{user_id} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Name"
  }'
```

### Eliminare un utente

```bash
curl -X DELETE http://127.0.0.1:3000/api/users/{user_id}
```

### Script di test automatico

```bash
# Esegui lo script di esempio (richiede jq)
bash examples/api_usage.sh
```

## 🐳 Docker

### Costruire l'immagine

```bash
docker build -t clean-architecture-rust .
```

### Eseguire il container

```bash
docker run -p 3000:3000 clean-architecture-rust
```

## 🛠️ Strumenti di Sviluppo (Opzionale)

### Installare strumenti utili

```bash
# Auto-reload durante lo sviluppo
cargo install cargo-watch

# Code coverage
cargo install cargo-tarpaulin

# Componenti Rust
rustup component add clippy
rustup component add rustfmt
```

### Utilizzare cargo-watch

```bash
# Ricompila automaticamente al cambio dei file
cargo watch -x run

# Esegui test automaticamente
cargo watch -x test
```

## 📊 Code Coverage

```bash
# Genera report HTML di coverage
cargo tarpaulin --out Html --output-dir coverage

# Apri coverage/index.html nel browser
```

## 🔧 Configurazione

### Variabili d'Ambiente

Crea un file `.env` (opzionale):

```bash
cp .env.example .env
```

Modifica `.env`:

```env
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
RUST_LOG=clean_architecture_rust=debug
```

### Logging

```bash
# Debug mode
RUST_LOG=debug cargo run

# Info mode (default)
RUST_LOG=info cargo run

# Solo errori
RUST_LOG=error cargo run
```

## 📝 Prossimi Passi

1. **Leggere la documentazione**:
   - [`README.md`](README.md) - Panoramica del progetto
   - [`ARCHITECTURE.md`](ARCHITECTURE.md) - Dettagli architetturali
   - [`docs/EXTENDING.md`](docs/EXTENDING.md) - Come estendere il progetto
   - [`docs/TESTING.md`](docs/TESTING.md) - Guida ai test

2. **Esplorare il codice**:
   - Inizia dal `src/domain/` per capire il business logic
   - Guarda `src/application/use_cases/` per i casi d'uso
   - Esamina `src/infrastructure/` per le implementazioni
   - Studia `src/presentation/` per gli HTTP handlers

3. **Modificare ed Estendere**:
   - Aggiungi nuovi campi all'entità User
   - Crea una nuova entità (es: Product)
   - Implementa un repository PostgreSQL
   - Aggiungi autenticazione JWT

## 🆘 Risoluzione Problemi

### Il server non si avvia

```bash
# Verifica che la porta 3000 sia libera
lsof -i :3000

# Oppure usa una porta diversa
SERVER_PORT=8080 cargo run
```

### Test falliscono

```bash
# Pulisci e ricompila
cargo clean
cargo build
cargo test
```

### Errori di compilazione

```bash
# Aggiorna Rust
rustup update

# Aggiorna dipendenze
cargo update
```

## 📞 Supporto

- Documentazione: Vedi cartella `docs/`
- Issues: Apri una issue su GitHub (se applicabile)
- Code: Tutto il codice è documentato con commenti

## ✨ Features Principali

- ✅ Clean Architecture completa
- ✅ 30+ test (unit + integration)
- ✅ Type-safe con Rust
- ✅ Async/await con Tokio
- ✅ REST API con Axum
- ✅ Value Objects per validazione
- ✅ Repository Pattern
- ✅ Dependency Injection
- ✅ Error Handling robusto
- ✅ Logging con tracing
- ✅ Docker support
- ✅ Makefile per comandi comuni

## 🎓 Apprendimento

Questo progetto dimostra:

1. **Separazione delle Responsabilità**: Ogni layer ha un ruolo preciso
2. **Dependency Inversion**: Le dipendenze puntano verso il dominio
3. **Testabilità**: Business logic testabile in isolamento
4. **Manutenibilità**: Facile aggiungere/modificare feature
5. **Scalabilità**: Architettura che scala con il progetto

Buon coding! 🚀
