use std::sync::Arc;

use axum::{
    Router,
    routing::{get},
};

use crate::{
    api::handlers::{
        get_persona,
        list_personas,
        create_persona,
        update_persona,
        delete_persona,
        get_persona_by_document,
        get_persona_by_email,
    },
    infra::AppState,
};

/// Rutas del módulo Persona
/// Retorna Router<AppState> para que sea compatible con el state global
pub fn persona_routes() -> Router<Arc<AppState>> {
    Router::new()
        // Rutas de búsqueda específica (deben ir primero para evitar conflictos)
        .route("/by-document/{ndocper}", get(get_persona_by_document))
        .route("/by-email/{emaper}", get(get_persona_by_email))
        // Rutas CRUD estándar
        .route("/", get(list_personas).post(create_persona))
        .route("/{idper}", get(get_persona).put(update_persona).delete(delete_persona))
}
