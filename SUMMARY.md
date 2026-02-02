# 📖 Riepilogo Architettura

## 🎯 Terminologia Usata

### Input/Output (invece di Primary/Secondary)

- **INPUT** = Riceve richieste dall'esterno → guida l'applicazione
- **OUTPUT** = Fornisce servizi esterni → supporta l'applicazione

| Concetto | Questa Implementazione | Alternativa Classica |
|----------|------------------------|---------------------|
| Riceve richieste dall'utente | **Input** Ports/Adapters | Primary / Driving |
| Fornisce storage/servizi | **Output** Ports/Adapters | Secondary / Driven |

## 🏗️ Struttura dei Layer

```
INPUT ADAPTERS (ricevono richieste)
    ↓
INPUT PORTS (interfacce esposte)
    ↓
DOMAIN (business logic)
    ↓
OUTPUT PORTS (interfacce richieste)
    ↓
OUTPUT ADAPTERS (implementazioni concrete)
```

## 📁 Mappatura File → Concetti

| Directory | Cosa Contiene | Ruolo |
|-----------|---------------|-------|
| `domain/` | Entità, Value Objects, Services | Core business logic |
| `ports/input.rs` | Trait UserManagement | Cosa l'app può fare |
| `ports/output.rs` | Trait UserRepository | Di cosa ha bisogno l'app |
| `adapters/input/` | HTTP API, CLI | Come riceve input |
| `adapters/output/` | FileStorage, DB | Come ottiene servizi |

## 🔄 Flusso Completo

```
HTTP Request
    ↓
HttpServer (input adapter)
    ↓
UserService (domain service)
    ↓
UserRepository trait (output port)
    ↓
FileUserRepository (output adapter)
    ↓
filesystem
```

## 💡 Regola Mnemonica

**IN** = Input = **IN**put dall'esterno  
**OUT** = Output = **OUT** verso risorse esterne

Molto più semplice di Primary/Secondary! 😊
