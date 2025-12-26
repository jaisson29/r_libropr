use crate::errors::AppResult;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

/// Pool de conexiones a MySQL
/// En Rust, el pool maneja las conexiones de forma segura y eficiente
/// No hay memory leaks ni conexiones colgadas
#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

impl Database {
    /// Crea una nueva conexión a la base de datos
    pub async fn new(database_url: &str, max: u32) -> AppResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max)
            .connect(database_url)
            .await
            .map_err(|e| {
                crate::errors::AppError::Internal(format!(
                    "No se pudo conectar a la base de datos: {}",
                    e
                ))
            })?;

        tracing::info!("✅ Conexión a base de datos establecida");

        Ok(Self { pool })
    }

    /// Obtiene el pool interno
    /// En Rust, esto se llama "consumir" el objeto
    pub fn pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    /// Verifica que la conexión esté funcionando
    pub async fn ping(&self) -> AppResult<()> {
        sqlx::query("SELECT 1").fetch_one(&self.pool).await?;
        Ok(())
    }
}
