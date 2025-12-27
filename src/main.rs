use std::sync::Arc;

use axum::
    Router
;
use libropr_rust::{
    api::app_router,
    config::{Config, Database},
    infra::AppState,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    init_tracing();

    let config = Config::from_env().expect("No se pudo cargar la configuración del entorno");
    let pool = init_database(config.database_url.as_str())
        .await
        .expect("No se pudo inicializar la base de datos");
    
    // Composition root: construir state con repos y services una sola vez
    let state = Arc::new(AppState::new(pool, config.jwt_secret.clone()));
    let app: Router = app_router(state);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("No se pudo bindear el listener");

    tracing::info!("✅ Servidor corriendo en http://{}", addr);
    tracing::info!("📚 API disponible en: http://{}/api/v1", addr);


    axum::serve(listener, app).await.unwrap();
}

async fn init_database(database_url: &str) -> anyhow::Result<sqlx::PgPool> {
    let db = Database::new(database_url, 10).await?;
    db.ping().await?;
    Ok(db.pg_pool().clone())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                // Usa el nombre real del crate para que el filtro funcione
                .unwrap_or_else(|_| "libropr_rust=info,tower_http=debug,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
