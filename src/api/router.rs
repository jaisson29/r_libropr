use std::sync::Arc;

use axum::{
    Router,
    http::{
        Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use tower_http::cors::{Any, CorsLayer};

use crate::{api::{auth_routes, persona_routes}, infra::AppState};

/// Router principal de la aplicación
pub fn app_router(state: Arc<AppState>) -> Router {
    // Configuración de CORS
    let cors = CorsLayer::new()
        .allow_origin(Any) // En producción: especificar orígenes exactos
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    // Combinar todas las rutas
    Router::new()
        .nest("/api/v1", api_routes())
        .layer(cors)
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state)
}

fn api_routes() -> Router<Arc<AppState>> {
    Router::new()
        .nest("/persona", persona_routes())
        .nest("/auth", auth_routes())
}
