use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::application::{
    CreateUserDto, CreateUserUseCase, DeleteUserUseCase, GetUserUseCase, ListUsersUseCase,
    UpdateUserDto, UpdateUserUseCase, UserDto,
};
use crate::DynUserRepository;

use super::AppError;

/// Handler per creare un nuovo utente
pub async fn create_user(
    State(repository): State<DynUserRepository>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<UserDto>, AppError> {
    let use_case = CreateUserUseCase::new(repository);
    let user = use_case.execute(dto).await?;
    Ok(Json(user))
}

/// Handler per ottenere un utente per ID
pub async fn get_user(
    State(repository): State<DynUserRepository>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserDto>, AppError> {
    let use_case = GetUserUseCase::new(repository);
    let user = use_case.execute(id).await?;
    Ok(Json(user))
}

/// Handler per listare tutti gli utenti
pub async fn list_users(
    State(repository): State<DynUserRepository>,
) -> Result<Json<Vec<UserDto>>, AppError> {
    let use_case = ListUsersUseCase::new(repository);
    let users = use_case.execute().await?;
    Ok(Json(users))
}

/// Handler per aggiornare un utente
pub async fn update_user(
    State(repository): State<DynUserRepository>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateUserDto>,
) -> Result<Json<UserDto>, AppError> {
    let use_case = UpdateUserUseCase::new(repository);
    let user = use_case.execute(id, dto).await?;
    Ok(Json(user))
}

/// Handler per eliminare un utente
pub async fn delete_user(
    State(repository): State<DynUserRepository>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let use_case = DeleteUserUseCase::new(repository);
    use_case.execute(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
