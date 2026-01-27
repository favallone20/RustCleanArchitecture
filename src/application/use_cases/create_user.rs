use crate::application::dto::{CreateUserDto, UserDto};
use crate::domain::{DomainError, Email, Name, User, UserRepository};

/// Use Case: Crea un nuovo utente
pub struct CreateUserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> CreateUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, dto: CreateUserDto) -> Result<UserDto, DomainError> {
        tracing::info!(
            email = %dto.email,
            name = %dto.name,
            "Creating new user"
        );

        // Validazione dell'email tramite Value Object
        let email = Email::new(dto.email)?;

        // Validazione del nome tramite Value Object
        // Non serve più validare manualmente - il Value Object lo fa per noi!
        let name = Name::new(dto.name)?;

        // Verifica che l'utente non esista già
        if self.repository.exists_by_email(&email).await? {
            tracing::warn!(
                email = %email.value(),
                "Attempt to create user with existing email"
            );
            return Err(DomainError::UserAlreadyExists(email.value().to_string()));
        }

        // Crea l'entità User
        let user = User::new(email, name);
        let user_id = *user.id();

        // Salva l'utente tramite il repository
        let saved_user = self.repository.save(user).await?;

        tracing::info!(
            user_id = %user_id,
            email = %saved_user.email().value(),
            "User created successfully"
        );

        // Ritorna il DTO
        Ok(UserDto::from(saved_user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::persistence::in_memory_user_repository::InMemoryUserRepository;

    #[tokio::test]
    async fn test_create_user_success() {
        let repository = InMemoryUserRepository::new();
        let use_case = CreateUserUseCase::new(repository);

        let dto = CreateUserDto {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        let result = use_case.execute(dto).await;
        assert!(result.is_ok());

        let user_dto = result.unwrap();
        assert_eq!(user_dto.email, "test@example.com");
        assert_eq!(user_dto.name, "Test User");
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let repository = InMemoryUserRepository::new();
        let use_case = CreateUserUseCase::new(repository);

        let dto = CreateUserDto {
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
        };

        // Prima creazione dovrebbe avere successo
        let _ = use_case.execute(dto.clone()).await.unwrap();

        // Seconda creazione dovrebbe fallire
        let result = use_case.execute(dto).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::UserAlreadyExists(_)));
    }

    #[tokio::test]
    async fn test_create_user_invalid_email() {
        let repository = InMemoryUserRepository::new();
        let use_case = CreateUserUseCase::new(repository);

        let dto = CreateUserDto {
            email: "invalid-email".to_string(),
            name: "Test User".to_string(),
        };

        let result = use_case.execute(dto).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::InvalidEmail(_)));
    }
}
