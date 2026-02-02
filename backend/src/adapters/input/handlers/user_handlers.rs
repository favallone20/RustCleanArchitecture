// USER HANDLERS - Gestisce tutti gli endpoint /api/users/*
// Ogni handler è una funzione async che processa una richiesta HTTP specifica

use crate::domain::{DomainError, UserService};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use contracts::{ApiResponse, CreateUserDto, UpdateUserDto, UserDto};
use std::sync::Arc;
use uuid::Uuid;

// ============================================================================
// CRUD Handlers
// ============================================================================

/// POST /api/users - Crea un nuovo utente
pub async fn create_user(
    State(service): State<Arc<UserService>>,
    Json(dto): Json<CreateUserDto>,
) -> Result<Json<ApiResponse<UserDto>>, DomainError> {
    let user = service.create_user(dto.email, dto.name, dto.password).await?;
    let dto = user_to_dto(&user);
    Ok(Json(ApiResponse::success(dto)))
}

/// GET /api/users/:id - Ottiene un singolo utente per ID
pub async fn get_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<UserDto>>, DomainError> {
    let user = service.get_user(id).await?;
    let dto = user_to_dto(&user);
    Ok(Json(ApiResponse::success(dto)))
}

/// GET /api/users - Lista tutti gli utenti
pub async fn get_all_users(
    State(service): State<Arc<UserService>>,
) -> Result<Json<ApiResponse<Vec<UserDto>>>, DomainError> {
    let users = service.list_users().await?;
    let dtos = users.iter().map(user_to_dto).collect();
    Ok(Json(ApiResponse::success(dtos)))
}

/// PUT /api/users/:id - Aggiorna un utente esistente
pub async fn update_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
    Json(dto): Json<UpdateUserDto>,
) -> Result<Json<ApiResponse<UserDto>>, DomainError> {
    let user = service.update_user(id, dto.email, dto.name).await?;
    let dto = user_to_dto(&user);
    Ok(Json(ApiResponse::success(dto)))
}

/// DELETE /api/users/:id - Elimina un utente
pub async fn delete_user(
    State(service): State<Arc<UserService>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, DomainError> {
    service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Mapper Helper - Converte Domain Entity -> DTO
// ============================================================================

fn user_to_dto(user: &crate::domain::User) -> UserDto {
    UserDto {
        id: user.id(),
        email: user.email().value().to_string(),
        name: user.name().to_string(),
        created_at: user.created_at(),
        updated_at: user.updated_at(),
    }
}
