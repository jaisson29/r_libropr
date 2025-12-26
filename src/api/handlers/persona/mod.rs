use std::sync::Arc;
use axum::{
    Json,
    extract::{State, Path},
};

use crate::{
    api::middleware::AuthUser,
    infra::AppState,
    errors::AppError,
    core::models::Persona,
};

/// GET /api/personas/:idper
/// Obtener una persona por su ID
pub async fn get_persona(
    auth_user: AuthUser,
    State(_state): State<Arc<AppState>>,
    Path(idper): Path<i64>,
) -> Result<Json<Persona>, AppError> {
    // TODO: Implementar lógica de obtención
    Err(AppError::NotFound("Persona not found".to_string()))
}

/// GET /api/personas
/// Listar todas las personas (paginado)
pub async fn list_personas(
    auth_user: AuthUser,
    State(_state): State<Arc<AppState>>,
) -> Result<Json<Vec<Persona>>, AppError> {
    // TODO: Implementar lógica de listado
    Ok(Json(Vec::new()))
}

/// POST /api/personas
/// Crear una nueva persona
pub async fn create_persona(
    auth_user: AuthUser,
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<Persona>,
) -> Result<Json<Persona>, AppError> {
    // TODO: Implementar lógica de creación
    Err(AppError::NotImplemented("Creation not implemented".to_string()))
}

/// PUT /api/personas/:idper
/// Actualizar una persona
pub async fn update_persona(
    auth_user: AuthUser,
    State(_state): State<Arc<AppState>>,
    Path(idper): Path<i64>,
    Json(_payload): Json<Persona>,
) -> Result<Json<Persona>, AppError> {
    // TODO: Implementar lógica de actualización
    Err(AppError::NotImplemented("Update not implemented".to_string()))
}

/// DELETE /api/personas/:idper
/// Eliminar una persona (desactivar)
pub async fn delete_persona(
    auth_user: AuthUser,
    State(_state): State<Arc<AppState>>,
    Path(idper): Path<i64>,
) -> Result<(), AppError> {
    // TODO: Implementar lógica de eliminación
    Err(AppError::NotImplemented("Deletion not implemented".to_string()))
}

/// GET /api/personas/by-document/:ndocper
/// Obtener una persona por su número de documento
pub async fn get_persona_by_document(
    auth_user: AuthUser,
    State(_state): State<Arc<AppState>>,
    Path(ndocper): Path<String>,
) -> Result<Json<Persona>, AppError> {
    // TODO: Implementar lógica de búsqueda
    Err(AppError::NotFound("Persona not found".to_string()))
}

/// GET /api/personas/by-email/:emaper
/// Obtener una persona por su email
pub async fn get_persona_by_email(
    auth_user: AuthUser,
    State(_state): State<Arc<AppState>>,
    Path(emaper): Path<String>,
) -> Result<Json<Persona>, AppError> {
    // TODO: Implementar lógica de búsqueda
    Err(AppError::NotFound("Persona not found".to_string()))
}
