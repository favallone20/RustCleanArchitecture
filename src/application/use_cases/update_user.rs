use uuid::Uuid;
use crate::application::dto::{UpdateUserDto, UserDto};
use crate::domain::{DomainError, Email, Name, UserRepository};

/// Use Case: Aggiorna un utente
pub struct UpdateUserUseCase<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> UpdateUserUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        dto: UpdateUserDto,
    ) -> Result<UserDto, DomainError> {
        tracing::info!(
            user_id = %user_id,
            new_email = ?dto.email,
            new_name = ?dto.name,
            "Updating user"
        );

        // Trova l'utente esistente
        let mut user = self
            .repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| {
                tracing::warn!(user_id = %user_id, "User not found for update");
                DomainError::UserNotFound(user_id.to_string())
            })?;

        // Aggiorna l'email se fornita
        if let Some(email_str) = dto.email {
            let new_email = Email::new(email_str)?;
            
            // Verifica che la nuova email non sia già in uso da un altro utente
            if let Some(existing_user) = self.repository.find_by_email(&new_email).await? {
                if existing_user.id() != user.id() {
                    tracing::warn!(
                        email = %new_email.value(),
                        "Attempt to update to existing email"
                    );
                    return Err(DomainError::UserAlreadyExists(
                        new_email.value().to_string(),
                    ));
                }
            }
            
            user.update_email(new_email);
        }

        // Aggiorna il nome se fornito
        if let Some(name_str) = dto.name {
            // Validazione automatica tramite Value Object
            let new_name = Name::new(name_str)?;
            user.update_name(new_name);
        }

        // Salva le modifiche
        let updated_user = self.repository.update(user).await?;

        tracing::info!(user_id = %user_id, "User updated successfully");
        Ok(UserDto::from(updated_user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::CreateUserDto;
    use crate::application::use_cases::CreateUserUseCase;
    use crate::infrastructure::persistence::in_memory_user_repository::InMemoryUserRepository;

    #[tokio::test]
    async fn test_update_user_name() {
        let repository = InMemoryUserRepository::new();
        
        // Crea un utente
        let create_use_case = CreateUserUseCase::new(repository.clone());
        let created_user = create_use_case
            .execute(CreateUserDto {
                email: "test@example.com".to_string(),
                name: "Old Name".to_string(),
            })
            .await
            .unwrap();

        // Aggiorna il nome
        let update_use_case = UpdateUserUseCase::new(repository);
        let result = update_use_case
            .execute(
                created_user.id,
                UpdateUserDto {
                    email: None,
                    name: Some("New Name".to_string()),
                },
            )
            .await;

        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.name, "New Name");
        assert_eq!(updated_user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_update_user_email() {
        let repository = InMemoryUserRepository::new();
        
        // Crea un utente
        let create_use_case = CreateUserUseCase::new(repository.clone());
        let created_user = create_use_case
            .execute(CreateUserDto {
                email: "old@example.com".to_string(),
                name: "Test User".to_string(),
            })
            .await
            .unwrap();

        // Aggiorna l'email
        let update_use_case = UpdateUserUseCase::new(repository);
        let result = update_use_case
            .execute(
                created_user.id,
                UpdateUserDto {
                    email: Some("new@example.com".to_string()),
                    name: None,
                },
            )
            .await;

        assert!(result.is_ok());
        let updated_user = result.unwrap();
        assert_eq!(updated_user.email, "new@example.com");
    }

    #[tokio::test]
    async fn test_update_user_not_found() {
        let repository = InMemoryUserRepository::new();
        let use_case = UpdateUserUseCase::new(repository);

        let result = use_case
            .execute(
                Uuid::new_v4(),
                UpdateUserDto {
                    email: None,
                    name: Some("New Name".to_string()),
                },
            )
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), DomainError::UserNotFound(_)));
    }
}
