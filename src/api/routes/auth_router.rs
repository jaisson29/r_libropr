use std::sync::Arc;

use axum::{Router, routing::post};

use crate::{api::handlers::auth::login_handler, infra::AppState};

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", post(login_handler))
    // Aquí se agregarán las rutas relacionadas con la autenticación
}
