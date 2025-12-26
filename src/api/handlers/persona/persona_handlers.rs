use std::sync::Arc;
use axum::{
    Json,
    extract::{State, Path},
};

use crate::{
    api::middleware::AuthUser, core::models::Persona, errors::{AppError, AppResult}, infra::AppState
};

/// GET /api/v1/persona/:idper
/// Obtener una persona por su ID
pub async fn get_persona(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(idper): Path<i64>,
) -> AppResult<Json<Persona>> {
    let persona = state.services.persona
        .get_by_id(idper)
        .await?
        .ok_or_else(|| AppError::NotFound("Persona no encontrada".to_string()))?;
    
    Ok(Json(persona))
}

/// GET /api/v1/persona
/// Listar todas las personas (paginado)
pub async fn list_personas(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<Persona>>> {
    // Por defecto: primeros 100 registros
    let personas = state.services.persona.list(100, 0).await?;
    Ok(Json(personas))
}

/// POST /api/v1/persona
/// Crear una nueva persona
pub async fn create_persona(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Persona>,
) -> AppResult<Json<Persona>> {
    let nueva_persona = state.services.persona.create(payload).await?;
    Ok(Json(nueva_persona))
}

/// PUT /api/v1/persona/:idper
/// Actualizar una persona
pub async fn update_persona(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(idper): Path<i64>,
    Json(payload): Json<Persona>,
) -> AppResult<Json<Persona>> {
    let persona_actualizada = state.services.persona.update(idper, payload).await?;
    Ok(Json(persona_actualizada))
}

/// DELETE /api/v1/persona/:idper
/// Eliminar una persona (desactivar)
pub async fn delete_persona(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(idper): Path<i64>,
) -> AppResult<()> {
    state.services.persona.delete(idper).await?;
    Ok(())
}

/// GET /api/v1/persona/by-document/:ndocper
/// Obtener una persona por su número de documento
pub async fn get_persona_by_document(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(ndocper): Path<String>,
) -> AppResult<Json<Persona>> {
    let persona = state.services.persona
        .get_by_document(&ndocper)
        .await?
        .ok_or_else(|| AppError::NotFound("Persona no encontrada".to_string()))?;
    
    Ok(Json(persona))
}

/// GET /api/v1/persona/by-email/:emaper
/// Obtener una persona por su email
pub async fn get_persona_by_email(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(emaper): Path<String>,
) -> AppResult<Json<Persona>> {
    let persona = state.services.persona
        .get_by_email(&emaper)
        .await?
        .ok_or_else(|| AppError::NotFound("Persona no encontrada".to_string()))?;
    
    Ok(Json(persona))
}
