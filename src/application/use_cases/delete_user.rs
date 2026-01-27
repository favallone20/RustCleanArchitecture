use uuid::Uuid;
use crate::domain::{DomainError, UserRepository};

/// Use Case: Elimina un utente
pub struct DeleteUserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> DeleteUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<(), DomainError> {
        tracing::info!(user_id = %user_id, "Deleting user");

        // Verifica che l'utente esista
        if self.repository.find_by_id(&user_id).await?.is_none() {
            tracing::warn!(user_id = %user_id, "User not found for deletion");
            return Err(DomainError::UserNotFound(user_id.to_string()));
        }

        // Elimina l'utente
        self.repository.delete(&user_id).await?;

        tracing::info!(user_id = %user_id, "User deleted successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::CreateUserDto;
    use crate::application::use_cases::CreateUserUseCase;
    use crate::infrastructure::persistence::in_memory_user_repository::InMemoryUserRepository;

    #[tokio::test]
    async fn test_delete_user_success() {
        let repository = InMemoryUserRepository::new();
        
        // Crea un utente
        let create_use_case = CreateUserUseCase::new(repository.clone());
        let created_user = create_use_case
            .execute(CreateUserDto {
                email: "test@example.com".to_string(),
                name: "Test User".to_string(),
            })
            .await
            .unwrap();

        // Elimina l'utente
        let delete_use_case = DeleteUserUseCase::new(repository.clone());
        let result = delete_use_case.execute(created_user.id).await;

        assert!(result.is_ok());

        // Verifica che l'utente sia stato eliminato
        let user = repository.find_by_id(&created_user.id).await.unwrap();
        assert!(user.is_none());
    }

    #[tokio::test]
    async fn test_delete_user_not_found() {
        let repository = InMemoryUserRepository::new();
        let use_case = DeleteUserUseCase::new(repository);

        let result = use_case.execute(Uuid::new_v4()).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::UserNotFound(_)));
    }
}
