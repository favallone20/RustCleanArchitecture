use async_trait::async_trait;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{DomainError, Email, User, UserRepository};

/// Implementazione in-memory del UserRepository
/// Utilizza DashMap per il thread-safe storage senza lock espliciti
#[derive(Clone)]
pub struct InMemoryUserRepository {
    users: Arc<DashMap<Uuid, User>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Arc::new(DashMap::new()),
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn save(&self, user: User) -> Result<User, DomainError> {
        let id = *user.id();
        self.users.insert(id, user.clone());
        Ok(user)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, DomainError> {
        Ok(self.users.get(id).map(|entry| entry.value().clone()))
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, DomainError> {
        Ok(self
            .users
            .iter()
            .find(|entry| entry.value().email() == email)
            .map(|entry| entry.value().clone()))
    }

    async fn find_all(&self) -> Result<Vec<User>, DomainError> {
        Ok(self
            .users
            .iter()
            .map(|entry| entry.value().clone())
            .collect())
    }

    async fn update(&self, user: User) -> Result<User, DomainError> {
        let id = *user.id();
        
        if !self.users.contains_key(&id) {
            return Err(DomainError::UserNotFound(id.to_string()));
        }

        self.users.insert(id, user.clone());
        Ok(user)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), DomainError> {
        self.users
            .remove(id)
            .ok_or_else(|| DomainError::UserNotFound(id.to_string()))?;
        Ok(())
    }

    async fn exists_by_email(&self, email: &Email) -> Result<bool, DomainError> {
        Ok(self
            .users
            .iter()
            .any(|entry| entry.value().email() == email))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Name;

    #[tokio::test]
    async fn test_save_and_find_by_id() {
        let repo = InMemoryUserRepository::new();
        let email = Email::new("test@example.com").unwrap();
        let name = Name::new("Test User".to_string()).unwrap();
        let user = User::new(email, name);
        let user_id = *user.id();

        repo.save(user.clone()).await.unwrap();

        let found = repo.find_by_id(&user_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), &user_id);
    }

    #[tokio::test]
    async fn test_find_by_email() {
        let repo = InMemoryUserRepository::new();
        let email = Email::new("test@example.com").unwrap();
        let name = Name::new("Test User".to_string()).unwrap();
        let user = User::new(email.clone(), name);

        repo.save(user).await.unwrap();

        let found = repo.find_by_email(&email).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().email(), &email);
    }

    #[tokio::test]
    async fn test_update() {
        let repo = InMemoryUserRepository::new();
        let email = Email::new("test@example.com").unwrap();
        let name = Name::new("Old Name".to_string()).unwrap();
        let mut user = User::new(email, name);
        let user_id = *user.id();

        repo.save(user.clone()).await.unwrap();

        let new_name = Name::new("New Name".to_string()).unwrap();
        user.update_name(new_name.clone());
        repo.update(user).await.unwrap();

        let found = repo.find_by_id(&user_id).await.unwrap().unwrap();
        assert_eq!(found.name(), &new_name);
    }

    #[tokio::test]
    async fn test_delete() {
        let repo = InMemoryUserRepository::new();
        let email = Email::new("test@example.com").unwrap();
        let name = Name::new("Test User".to_string()).unwrap();
        let user = User::new(email, name);
        let user_id = *user.id();

        repo.save(user).await.unwrap();
        repo.delete(&user_id).await.unwrap();

        let found = repo.find_by_id(&user_id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_exists_by_email() {
        let repo = InMemoryUserRepository::new();
        let email = Email::new("test@example.com").unwrap();
        let name = Name::new("Test User".to_string()).unwrap();
        let user = User::new(email.clone(), name);

        assert!(!repo.exists_by_email(&email).await.unwrap());

        repo.save(user).await.unwrap();

        assert!(repo.exists_by_email(&email).await.unwrap());
    }

    #[tokio::test]
    async fn test_find_all() {
        let repo = InMemoryUserRepository::new();
        
        let user1 = User::new(
            Email::new("user1@example.com").unwrap(),
            Name::new("User 1".to_string()).unwrap(),
        );
        let user2 = User::new(
            Email::new("user2@example.com").unwrap(),
            Name::new("User 2".to_string()).unwrap(),
        );

        repo.save(user1).await.unwrap();
        repo.save(user2).await.unwrap();

        let all_users = repo.find_all().await.unwrap();
        assert_eq!(all_users.len(), 2);
    }
}
