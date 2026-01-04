use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::{Pagper, db::PagperRepository},
    errors::{AppError, AppResult},
};

pub struct PagperRepositoryPg {
    pool: PgPool,
}

impl PagperRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PagperRepository for PagperRepositoryPg {
    async fn find_by_perfil(&self, idpef: i64) -> AppResult<Vec<Pagper>> {
        let pagpers = sqlx::query_as::<_, Pagper>(
            r#"
            SELECT 
                pp.idpef,
                pp.idpag,
                pp.can_create,
                pp.can_read,
                pp.can_update,
                pp.can_delete,
            FROM pagper AS pp
            INNER JOIN pagina p ON pp.idpag = p.idpag
            WHERE pp.idpef = $1
            "#,
        )
        .bind(idpef)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("Error al obtener permisos del perfil {}: {:?}", idpef, e);
            AppError::Database(e)
        })?;

        tracing::debug!(
            "Permisos cargados para perfil {}: {} registros",
            idpef,
            pagpers.len()
        );
        Ok(pagpers)
    }

    async fn has_permission(&self, idpef: i64, codpag: &str, action: &str) -> AppResult<bool> {
        let permission_field = match action {
            "create" => "can_create",
            "read" => "can_read",
            "update" => "can_update",
            "delete" => "can_delete",
            _ => return Ok(false),
        };

        let query = format!(
            r#"
            SELECT pp.{}
            FROM pagper pp
            INNER JOIN pagina p ON pp.idpag = p.idpag
            WHERE pp.idpef = $1 AND p.codpag = $2
            "#,
            permission_field
        );

        let has_perm: Option<bool> = sqlx::query_scalar(&query)
            .bind(idpef)
            .bind(codpag)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::Database)?;

        Ok(has_perm.unwrap_or(false))
    }
}
