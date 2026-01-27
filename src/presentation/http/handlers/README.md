# 📂 Handlers Directory

Questa cartella contiene gli HTTP handlers organizzati per risorsa/entità.

## 📋 File Presenti

- **`mod.rs`** - Esporta tutti gli handler e definisce `AppError`
- **`user_handlers.rs`** - Handler per la risorsa User (CRUD completo)
- **`example_product_handlers.rs.template`** - Template per nuovi handler

## ➕ Aggiungere Nuovi Handler

### Quick Start

1. **Copia il template**:
   ```bash
   cp example_product_handlers.rs.template product_handlers.rs
   ```

2. **Implementa i tuoi handler** in `product_handlers.rs`

3. **Esporta nel module** (`mod.rs`):
   ```rust
   pub mod product_handlers;
   pub use product_handlers::*;
   ```

4. **Aggiungi route** in `../routes.rs`

5. **Test!**

## 📝 Pattern Handler

Ogni handler segue questo pattern:

```rust
pub async fn create_resource(
    State(repository): State<Arc<Repository>>,
    Json(dto): Json<CreateDto>,
) -> Result<Json<ResourceDto>, AppError> {
    let use_case = CreateUseCase::new(repository.as_ref().clone());
    let resource = use_case.execute(dto).await?;
    Ok(Json(resource))
}
```

**Regole**:
- Handler brevi (< 20 linee)
- Niente business logic (solo orchestrazione)
- Delega tutto ai use cases
- Usa `AppError` per errori

## 📚 Documentazione Completa

Vedi `/docs/HANDLERS_STRUCTURE.md` per guida dettagliata.

## 🎯 Esempi

### Handler Semplice (GET)
```rust
pub async fn get_user(
    State(repository): State<Arc<InMemoryUserRepository>>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserDto>, AppError> {
    let use_case = GetUserUseCase::new(repository.as_ref().clone());
    let user = use_case.execute(id).await?;
    Ok(Json(user))
}
```

### Handler con Body (POST)
```rust
pub async fn create_user(
    State(repository): State<Arc<InMemoryUserRepository>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<UserDto>, AppError> {
    let use_case = CreateUserUseCase::new(repository.as_ref().clone());
    let user = use_case.execute(dto).await?;
    Ok(Json(user))
}
```

### Handler con Path e Body (PUT)
```rust
pub async fn update_user(
    State(repository): State<Arc<InMemoryUserRepository>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateUserDto>,
) -> Result<Json<UserDto>, AppError> {
    let use_case = UpdateUserUseCase::new(repository.as_ref().clone());
    let user = use_case.execute(id, dto).await?;
    Ok(Json(user))
}
```

### Handler Delete (restituisce 204)
```rust
pub async fn delete_user(
    State(repository): State<Arc<InMemoryUserRepository>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let use_case = DeleteUserUseCase::new(repository.as_ref().clone());
    use_case.execute(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
```

## ✅ Checklist per Nuovi Handler

- [ ] Implementato domain entity
- [ ] Implementato use cases
- [ ] Implementato repository
- [ ] Creato file handler (`*_handlers.rs`)
- [ ] Esportato in `mod.rs`
- [ ] Aggiunto route in `routes.rs`
- [ ] Scritto test
- [ ] Documentato API

---

**Tip**: Usa `example_product_handlers.rs.template` come punto di partenza!
