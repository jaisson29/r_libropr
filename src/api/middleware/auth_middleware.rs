use crate::domain::{AuthUser, Claims};
use crate::errors::AppError;
use crate::infra::AppState;
use axum::extract::FromRequestParts;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use std::{collections::HashMap, future::Future, sync::Arc};

impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<AppState>,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let state = state.clone();
        
        async move {
            // 1. Extraer y validar el token
            let auth_header = parts
                .headers
                .get(axum::http::header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| AppError::Unauthorized("Token no encontrado".to_string()))?;

            tracing::debug!("Authorization Header: {}", auth_header);

            if !auth_header.starts_with("Bearer ") {
                return Err(AppError::Unauthorized(
                    "Formato Bearer requerido".to_string(),
                ));
            }
            let token = &auth_header[7..];

            // 2. Decodificar y Validar JWT
            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|e| {
                tracing::warn!("Token inválido: {:?}", e);
                AppError::Unauthorized("Token expirado o inválido".to_string())
            })?;

            // 3. Cargar permisos desde el servicio (con caché)
            let permissions = state
                .services
                .permission
                .get_permissions_for_profile(token_data.claims.idpef)
                .await
                .unwrap_or_else(|e| {
                    tracing::error!("Error al cargar permisos: {:?}", e);
                    HashMap::new()
                });

            // 4. Retornar AuthUser con permisos cargados
            Ok(AuthUser {
                idper: token_data.claims.idper,
                nomper: token_data.claims.nomper,
                idpef: token_data.claims.idpef,
                nompef: token_data.claims.nompef,
                is_super_admin: token_data.claims.idpef == 1,
                permissions,
            })
        }
    }
}
