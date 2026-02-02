// FILE STORAGE ADAPTER - Output Adapter per persistenza su file JSON
// Implementa UserRepository usando il filesystem

use crate::domain::{DomainError, DomainResult, Email, User};
use crate::ports::output::UserRepository;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::RwLock;
use uuid::Uuid;

// ============================================================================
// File-based User Repository
// ============================================================================

pub struct FileUserRepository {
    file_path: PathBuf,
    cache: Arc<RwLock<HashMap<Uuid, UserRecord>>>,
}

/// Rappresentazione per la serializzazione su file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserRecord {
    id: Uuid,
    email: String,
    name: String,
    password_hash: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl FileUserRepository {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        let file_path = file_path.into();
        
        // Crea la directory se non esiste
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }

        let repo = Self {
            file_path,
            cache: Arc::new(RwLock::new(HashMap::new())),
        };

        // Carica i dati esistenti in modo sincrono all'avvio
        if let Ok(data) = std::fs::read_to_string(&repo.file_path) {
            if let Ok(users) = serde_json::from_str::<Vec<UserRecord>>(&data) {
                let mut cache = futures::executor::block_on(repo.cache.write());
                for user in users {
                    cache.insert(user.id, user);
                }
                println!("📂 Caricati {} utenti da {}", cache.len(), repo.file_path.display());
            }
        } else {
            println!("📂 Creato nuovo file storage: {}", repo.file_path.display());
        }

        repo
    }

    /// Salva la cache su file
    async fn persist(&self) -> DomainResult<()> {
        let cache = self.cache.read().await;
        let users: Vec<UserRecord> = cache.values().cloned().collect();
        
        let json = serde_json::to_string_pretty(&users)
            .map_err(|e| DomainError::StorageError(e.to_string()))?;

        fs::write(&self.file_path, json)
            .await
            .map_err(|e| DomainError::StorageError(e.to_string()))?;

        Ok(())
    }

    /// Converte User -> UserRecord
    fn to_record(user: &User) -> UserRecord {
        UserRecord {
            id: user.id(),
            email: user.email().value().to_string(),
            name: user.name().to_string(),
            password_hash: user.password_hash().to_string(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }

    /// Converte UserRecord -> User
    fn from_record(record: &UserRecord) -> DomainResult<User> {
        let email = Email::new(record.email.clone())?;
        Ok(User::reconstitute(
            record.id,
            email,
            record.name.clone(),
            record.password_hash.clone(),
            record.created_at,
            record.updated_at,
        ))
    }
}

#[async_trait]
impl UserRepository for FileUserRepository {
    async fn save(&self, user: User) -> DomainResult<User> {
        let record = Self::to_record(&user);
        let id = record.id;

        {
            let mut cache = self.cache.write().await;
            cache.insert(id, record);
        }

        self.persist().await?;
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<User>> {
        let cache = self.cache.read().await;
        
        match cache.get(&id) {
            Some(record) => Ok(Some(Self::from_record(record)?)),
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> DomainResult<Option<User>> {
        let cache = self.cache.read().await;
        
        for record in cache.values() {
            if record.email == email {
                return Ok(Some(Self::from_record(record)?));
            }
        }
        
        Ok(None)
    }

    async fn exists_by_email(&self, email: &str) -> DomainResult<bool> {
        Ok(self.find_by_email(email).await?.is_some())
    }

    async fn find_all(&self) -> DomainResult<Vec<User>> {
        let cache = self.cache.read().await;
        
        cache
            .values()
            .map(Self::from_record)
            .collect::<DomainResult<Vec<User>>>()
    }

    async fn update(&self, user: User) -> DomainResult<User> {
        let record = Self::to_record(&user);
        let id = record.id;

        {
            let mut cache = self.cache.write().await;
            
            if !cache.contains_key(&id) {
                return Err(DomainError::NotFound(format!("Utente {} non trovato", id)));
            }
            
            cache.insert(id, record);
        }

        self.persist().await?;
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        {
            let mut cache = self.cache.write().await;
            
            if cache.remove(&id).is_none() {
                return Err(DomainError::NotFound(format!("Utente {} non trovato", id)));
            }
        }

        self.persist().await?;
        Ok(())
    }
}
