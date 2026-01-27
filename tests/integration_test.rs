use clean_architecture_rust::{
    CreateUserDto, CreateUserUseCase, DeleteUserUseCase, GetUserUseCase, InMemoryUserRepository,
    ListUsersUseCase, UpdateUserDto, UpdateUserUseCase,
};

/// Test di integrazione completo che verifica il flusso end-to-end
#[tokio::test]
async fn test_complete_user_lifecycle() {
    // Setup: Crea il repository
    let repository = InMemoryUserRepository::new();

    // Test 1: Crea un utente
    let create_use_case = CreateUserUseCase::new(repository.clone());
    let create_dto = CreateUserDto {
        email: "integration@test.com".to_string(),
        name: "Integration Test User".to_string(),
    };

    let created_user = create_use_case
        .execute(create_dto)
        .await
        .expect("Failed to create user");

    assert_eq!(created_user.email, "integration@test.com");
    assert_eq!(created_user.name, "Integration Test User");

    // Test 2: Recupera l'utente per ID
    let get_use_case = GetUserUseCase::new(repository.clone());
    let retrieved_user = get_use_case
        .execute(created_user.id)
        .await
        .expect("Failed to get user");

    assert_eq!(retrieved_user.id, created_user.id);
    assert_eq!(retrieved_user.email, created_user.email);

    // Test 3: Aggiorna l'utente
    let update_use_case = UpdateUserUseCase::new(repository.clone());
    let update_dto = UpdateUserDto {
        email: None,
        name: Some("Updated Name".to_string()),
    };

    let updated_user = update_use_case
        .execute(created_user.id, update_dto)
        .await
        .expect("Failed to update user");

    assert_eq!(updated_user.name, "Updated Name");
    assert_eq!(updated_user.email, "integration@test.com");

    // Test 4: Lista tutti gli utenti
    let list_use_case = ListUsersUseCase::new(repository.clone());
    let all_users = list_use_case
        .execute()
        .await
        .expect("Failed to list users");

    assert_eq!(all_users.len(), 1);
    assert_eq!(all_users[0].name, "Updated Name");

    // Test 5: Elimina l'utente
    let delete_use_case = DeleteUserUseCase::new(repository.clone());
    delete_use_case
        .execute(created_user.id)
        .await
        .expect("Failed to delete user");

    // Test 6: Verifica che l'utente sia stato eliminato
    let all_users_after_delete = list_use_case
        .execute()
        .await
        .expect("Failed to list users after delete");

    assert_eq!(all_users_after_delete.len(), 0);
}

#[tokio::test]
async fn test_multiple_users() {
    let repository = InMemoryUserRepository::new();
    let create_use_case = CreateUserUseCase::new(repository.clone());

    // Crea 3 utenti
    for i in 1..=3 {
        let dto = CreateUserDto {
            email: format!("user{}@test.com", i),
            name: format!("User {}", i),
        };
        create_use_case.execute(dto).await.expect("Failed to create user");
    }

    // Verifica che ci siano 3 utenti
    let list_use_case = ListUsersUseCase::new(repository.clone());
    let users = list_use_case.execute().await.expect("Failed to list users");

    assert_eq!(users.len(), 3);
}

#[tokio::test]
async fn test_duplicate_email_validation() {
    let repository = InMemoryUserRepository::new();
    let create_use_case = CreateUserUseCase::new(repository.clone());

    // Crea il primo utente
    let dto = CreateUserDto {
        email: "duplicate@test.com".to_string(),
        name: "First User".to_string(),
    };
    create_use_case
        .execute(dto.clone())
        .await
        .expect("Failed to create first user");

    // Tenta di creare un secondo utente con la stessa email
    let result = create_use_case.execute(dto).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_email_to_existing_email() {
    let repository = InMemoryUserRepository::new();
    let create_use_case = CreateUserUseCase::new(repository.clone());

    // Crea due utenti
    let user1 = create_use_case
        .execute(CreateUserDto {
            email: "user1@test.com".to_string(),
            name: "User 1".to_string(),
        })
        .await
        .expect("Failed to create user 1");

    create_use_case
        .execute(CreateUserDto {
            email: "user2@test.com".to_string(),
            name: "User 2".to_string(),
        })
        .await
        .expect("Failed to create user 2");

    // Tenta di aggiornare l'email di user1 a quella di user2
    let update_use_case = UpdateUserUseCase::new(repository.clone());
    let result = update_use_case
        .execute(
            user1.id,
            UpdateUserDto {
                email: Some("user2@test.com".to_string()),
                name: None,
            },
        )
        .await;

    assert!(result.is_err());
}
