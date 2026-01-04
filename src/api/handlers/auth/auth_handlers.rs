use std::sync::Arc;

use axum::{Json, extract::State};
use jsonwebtoken::Header;

use crate::{
    api::dtos::{LoginRequestDTO, LoginResponseDTO}, domain::Claims, errors::{AppError, AppResult}, infra::AppState
};

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequestDTO>,
) -> AppResult<Json<LoginResponseDTO>> {
    // Lógica de autenticación aquí (verificación de credenciales, generación de token, etc.)
    // Por ahora, retornamos un token ficticio
    let persona = state
        .repos
        .persona
        .get_by_emaper(&payload.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Credenciales inválidas".to_string()))?;

    let claims: Claims = Claims {
        sub: persona.idper,
        exp: 10000000000,
        idper: persona.idper,
        nomper: persona.nomper,
        idpef: persona.idpef,
        nompef: persona.idpef.to_string(),
        emaper: persona.emaper.clone(),
    };
    let secret = std::env::var("JWT_SECRET")
        .map_err(|_| AppError::Unauthorized("JWT_SECRET no configurado".to_string()))?;

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Error al generar el token: {}", e)))?;
    let response = LoginResponseDTO { token };
    Ok(Json(response))
}
