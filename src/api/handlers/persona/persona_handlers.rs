use axum::{
    Json,
    extract::{Path, State},
};
use std::sync::Arc;

use crate::{
    domain::{AuthUser, Persona}, errors::{AppError, AppResult}, infra::AppState
};

/// GET /api/v1/persona/:idper
/// Obtener una persona por su ID
pub async fn get_persona(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(idper): Path<i64>,
) -> AppResult<Json<Persona>> {
    let persona = state
        .services
        .persona
        .get_by_id(idper)
        .await?
        .ok_or_else(|| AppError::NotFound("Persona no encontrada".to_string()))?;

    // Si la persona es superadmin y el usuario no es superadmin, denegar acceso
    if persona.idpef == 1 && !auth_user.is_super_admin() {
        return Err(AppError::Forbidden(
            "No tiene permisos para ver este usuario".to_string(),
        ));
    }

    Ok(Json(persona))
}

/// GET /api/v1/persona
/// Listar todas las personas (paginado)
pub async fn list_personas(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
) -> AppResult<Json<Vec<Persona>>> {
    // Por defecto: primeros 100 registros
    let mut personas = state.services.persona.list(100, 0).await?;

    // Si el usuario no es superadmin, filtrar los superadmins de la lista
    if !auth_user.is_super_admin() {
        // Suponiendo que idpef 1 es super_admin, necesitamos obtener el id correcto
        // Por ahora, filtramos por el nombre del perfil buscando en la base de datos
        // o asumimos que idpef 1 = super_admin
        personas.retain(|p| p.idpef != 1); // Ajusta el ID según tu base de datos
    }

    tracing::info!("Usuario {} listó personas", auth_user.nomper);
    tracing::info!("Usuario {:?}", auth_user);

    Ok(Json(personas))
}

/// POST /api/v1/persona
/// Crear una nueva persona
pub async fn create_persona(
    auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Persona>,
) -> AppResult<Json<Persona>> {
    // Si intenta crear un superadmin (idpef = 1) y no es superadmin, denegar
    if payload.idpef == 1 && !auth_user.is_super_admin() {
        return Err(AppError::Forbidden(
            "No tiene permisos para crear superadministradores".to_string(),
        ));
    }

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
    // Verificar si la persona que se está actualizando es un superadmin
    let persona_existente = state
        .services
        .persona
        .get_by_id(idper)
        .await?
        .ok_or_else(|| AppError::NotFound("Persona no encontrada".to_string()))?;

    // Si la persona existente es superadmin o se intenta cambiar a superadmin
    // y el usuario actual no es superadmin, denegar
    if (persona_existente.idpef == 1 || payload.idpef == 1) && !auth_user.is_super_admin() {
        return Err(AppError::Forbidden(
            "No tiene permisos para modificar superadministradores".to_string(),
        )); 
    }

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
    let persona = state
        .services
        .persona
        .get_by_id(idper)
        .await?
        .ok_or_else(|| AppError::NotFound("Persona no encontrada".to_string()))?;
    // Prevenir eliminación de superadmin
    if persona.idpef == 1 && !auth_user.is_super_admin() {
        return Err(AppError::Forbidden(
            "No se puede eliminar un superadministrador".to_string(),
        ));
    }
    state.services.persona.delete(idper).await?;
    Ok(())
}

/// GET /api/v1/persona/by-document/:ndocper
/// Obtener una persona por su número de documento
pub async fn get_persona_by_document(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(ndocper): Path<String>,
) -> AppResult<Json<Persona>> {
    let persona = state
        .services
        .persona
        .get_by_document(&ndocper)
        .await?
        .ok_or_else(|| AppError::NotFound("Persona no encontrada".to_string()))?;

    Ok(Json(persona))
}

/// GET /api/v1/persona/by-email/:emaper
/// Obtener una persona por su email
pub async fn get_persona_by_email(
    _auth_user: AuthUser,
    State(state): State<Arc<AppState>>,
    Path(emaper): Path<String>,
) -> AppResult<Json<Persona>> {
    let persona = state
        .services
        .persona
        .get_by_email(&emaper)
        .await?
        .ok_or_else(|| AppError::NotFound("Persona no encontrada".to_string()))?;

    Ok(Json(persona))
}
