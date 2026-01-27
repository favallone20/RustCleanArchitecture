use uuid::Uuid;
use crate::application::dto::UserDto;
use crate::domain::{DomainError, UserRepository};

/// Use Case: Ottieni un utente per ID
pub struct GetUserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> GetUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<UserDto, DomainError> {
        tracing::debug!(user_id = %user_id, "Getting user by ID");

        let user = self
            .repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| {
                tracing::warn!(user_id = %user_id, "User not found");
                DomainError::UserNotFound(user_id.to_string())
            })?;

        tracing::debug!(user_id = %user_id, "User found");
        Ok(UserDto::from(user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::CreateUserDto;
    use crate::application::use_cases::CreateUserUseCase;
    use crate::infrastructure::persistence::in_memory_user_repository::InMemoryUserRepository;

    #[tokio::test]
    async fn test_get_user_success() {
        let repository = InMemoryUserRepository::new();
        
        // Crea un utente prima
        let create_use_case = CreateUserUseCase::new(repository.clone());
        let dto = CreateUserDto {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };
        let created_user = create_use_case.execute(dto).await.unwrap();

        // Ora prova a recuperarlo
        let get_use_case = GetUserUseCase::new(repository);
        let result = get_use_case.execute(created_user.id).await;

        assert!(result.is_ok());
        let user_dto = result.unwrap();
        assert_eq!(user_dto.id, created_user.id);
        assert_eq!(user_dto.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let repository = InMemoryUserRepository::new();
        let use_case = GetUserUseCase::new(repository);

        let non_existent_id = Uuid::new_v4();
        let result = use_case.execute(non_existent_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::UserNotFound(_)));
    }
}
