use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

/// Error personalizado de la aplicación
/// thiserror genera automáticamente el trait Error para nosotros
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Solicitud incorrecta: {0}")]
    BadRequest(String),

    #[error("No implementado: {0}")]
    NotImplemented(String),

    #[error("No encontrado: {0}")]
    NotFound(String),

    #[error("No autorizado: {0}")]
    Unauthorized(String),

    #[error("Prohibido: {0}")]
    Forbidden(String),

    #[error("Validación fallida: {0}")]
    Validation(String),

    #[error("Error interno: {0}")]
    Internal(String),
}

/// Implementación para convertir nuestros errores en respuestas HTTP
/// Esto es lo que hace Rust tan poderoso: type safety hasta en los errores
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error de base de datos".to_string(),
                )
            }
            AppError::NotImplemented(msg) => {
                tracing::warn!("Not implemented: {}", msg);
                (StatusCode::NOT_IMPLEMENTED, msg)
            }
            AppError::BadRequest(msg) => {
                tracing::warn!("Bad request: {}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::NotFound(msg) => {
                tracing::warn!("Not found: {}", msg);
                (StatusCode::NOT_FOUND, msg)
            }
            AppError::Unauthorized(msg) => {
                tracing::warn!("Unauthorized: {}", msg);
                (StatusCode::UNAUTHORIZED, msg)
            }
            AppError::Forbidden(msg) => {
                tracing::warn!("Forbidden: {}", msg);
                (StatusCode::FORBIDDEN, msg)
            }
            AppError::Validation(msg) => {
                tracing::warn!("Validation error: {}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error interno del servidor".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

/// Tipo de resultado que usaremos en toda la app
/// En lugar de Result<T, E> escribimos AppResult<T>
pub type AppResult<T> = Result<T, AppError>;
