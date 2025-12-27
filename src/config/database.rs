use crate::errors::AppResult;
use sqlx::{PgPool, postgres::PgPoolOptions};

/// Pool de conexiones a MySQL
/// En Rust, el pool maneja las conexiones de forma segura y eficiente
/// No hay memory leaks ni conexiones colgadas
#[derive(Clone)]
pub struct Database {
    pg_pool: PgPool,
    // mysql_pool: MySqlPool,
}

impl Database {
    /// Crea una nueva conexión a la base de datos
    pub async fn new(database_url: &str, max: u32) -> AppResult<Self> {
        let pg_pool = PgPoolOptions::new()
            .max_connections(max)
            .connect(database_url)
            .await
            .map_err(|e| {
                crate::errors::AppError::Internal(format!(
                    "No se pudo conectar a la base de datos: {}",
                    e
                ))
            })?;

        tracing::info!("✅ Conexión a base de datos pg establecida");

        // let mysql_pool = MySqlPoolOptions::new()
        //     .max_connections(max)
        //     .connect(database_url)
        //     .await
        //     .map_err(|e| {
        //         crate::errors::AppError::Internal(format!(
        //             "No se pudo conectar a la base de datos MySQL: {}",
        //             e
        //         ))
        //     })?;

        Ok(Self { pg_pool })
        // Ok(Self { mysql_pool })
    }

    /// Obtiene el pool interno
    /// En Rust, esto se llama "consumir" el objeto
    pub fn pg_pool(&self) -> &PgPool {
        &self.pg_pool
    }

    // pub fn mysql_pool(&self) -> &MySqlPool {
    //     &self.mysql_pool
    // }

    /// Verifica que la conexión esté funcionando
    pub async fn ping(&self) -> AppResult<()> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pg_pool)
            .await
            .map_err(|e| {
                tracing::error!("❌ PostgreSQL ping failed: {}", e);
                crate::errors::AppError::Internal(format!("PostgreSQL connection failed: {}", e))
            })?;

        // sqlx::query("SELECT 1")
        //     .fetch_one(&self.mysql_pool)
        //     .await
        //     .map_err(|e| {
        //         tracing::error!("❌ MySQL ping failed: {}", e);
        //         crate::errors::AppError::Internal(format!("MySQL connection failed: {}", e))
        //     })?;

        tracing::info!("✅ Database connections verified");
        Ok(())
    }
}
