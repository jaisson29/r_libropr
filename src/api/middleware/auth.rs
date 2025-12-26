use crate::errors::AppError;
use axum::extract::FromRequestParts;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::future::{Future, ready};

/// Claims del JWT - Datos que se almacenan en el token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64,       // Subject (User ID)
    pub exp: usize,     // Expiración
    pub idper: u32,     // ID de la persona
    pub nomper: String, // Nombre de la persona
    pub idpef: u32,     // ID del perfil
    pub nompef: String, // Nombre del perfil
}

/// Información del usuario autenticado extraída del token JWT
/// Este struct se puede usar como parámetro en cualquier handler para
/// acceder a la información del usuario autenticado
#[derive(Debug, Clone, Serialize)]
pub struct AuthUser {
    pub idper: u32,     // ID de la persona
    pub nomper: String, // Nombre de la persona
    pub idpef: u32,     // ID del perfil (rol)
    pub nompef: String, // Nombre del perfil (Admin, Usuario, etc.)
}

impl AuthUser {
    /// Verifica si el usuario tiene un perfil específico
    pub fn has_profile(&self, nombre_perfil: &str) -> bool {
        self.nompef.eq_ignore_ascii_case(nombre_perfil)
    }

    /// Verifica si el usuario es administrador
    pub fn is_admin(&self) -> bool {
        self.has_profile("Administrador") || self.has_profile("Admin")
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let result = (|| {
            let auth_header = parts
                .headers
                .get(axum::http::header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .ok_or_else(|| AppError::Unauthorized("Token no encontrado".to_string()))?;

            if !auth_header.starts_with("Bearer ") {
                return Err(AppError::Unauthorized(
                    "Formato Bearer requerido".to_string(),
                ));
            }
            let token = &auth_header[7..];

            // Decodificar y Validar (Usar variable de entorno para el SECRET)
            let secret = std::env::var("JWT_SECRET")
                .map_err(|_| AppError::Unauthorized("JWT_SECRET no configurado".to_string()))?;

            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::new(Algorithm::HS256),
            )
            .map_err(|_| AppError::Unauthorized("Token expirado o inválido".to_string()))?;

            // Éxito: Retornamos el AuthUser
            Ok(AuthUser {
                idper: token_data.claims.idper,
                nomper: token_data.claims.nomper,
                idpef: token_data.claims.idpef,
                nompef: token_data.claims.nompef,
            })
        })();

        ready(result)
    }
}
