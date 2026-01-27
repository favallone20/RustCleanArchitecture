use crate::application::dto::UserDto;
use crate::domain::{DomainError, UserRepository};

/// Use Case: Lista tutti gli utenti
pub struct ListUsersUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> ListUsersUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> Result<Vec<UserDto>, DomainError> {
        tracing::debug!("Listing all users");
        let users = self.repository.find_all().await?;
        tracing::info!(user_count = users.len(), "Users retrieved successfully");
        Ok(UserDto::from_users(users))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::CreateUserDto;
    use crate::application::use_cases::CreateUserUseCase;
    use crate::infrastructure::persistence::in_memory_user_repository::InMemoryUserRepository;

    #[tokio::test]
    async fn test_list_users() {
        let repository = InMemoryUserRepository::new();
        
        // Crea alcuni utenti
        let create_use_case = CreateUserUseCase::new(repository.clone());
        create_use_case
            .execute(CreateUserDto {
                email: "user1@example.com".to_string(),
                name: "User 1".to_string(),
            })
            .await
            .unwrap();
        
        create_use_case
            .execute(CreateUserDto {
                email: "user2@example.com".to_string(),
                name: "User 2".to_string(),
            })
            .await
            .unwrap();

        // Lista gli utenti
        let list_use_case = ListUsersUseCase::new(repository);
        let result = list_use_case.execute().await;

        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 2);
    }

    #[tokio::test]
    async fn test_list_users_empty() {
        let repository = InMemoryUserRepository::new();
        let use_case = ListUsersUseCase::new(repository);

        let result = use_case.execute().await;

        assert!(result.is_ok());
        let users = result.unwrap();
        assert_eq!(users.len(), 0);
    }
}
