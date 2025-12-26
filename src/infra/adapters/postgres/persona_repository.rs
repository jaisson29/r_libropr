use crate::core::models::Persona;
/// Implementación del Puerto PersonaRepository usando SQLx
/// Este archivo muestra cómo un repositorio real cumple con el contrato del puerto
use crate::core::ports::PersonaRepository;
use crate::errors::AppError;
use sqlx::PgPool;

/// Repositorio de Persona que usa PostgreSQL a través de SQLx
pub struct PersonaRepositoryPg {
    db: PgPool,
}

impl PersonaRepositoryPg {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl PersonaRepository for PersonaRepositoryPg {
    async fn get_by_idper(&self, idper: i64) -> Result<Option<Persona>, AppError> {
        let persona = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM personas 
             WHERE idper = $1"
        )
        .bind(idper)
        .fetch_optional(&self.db)
        .await?;

        Ok(persona)
    }

    async fn get_by_ndocper(&self, ndocper: &str) -> Result<Option<Persona>, AppError> {
        let persona = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM personas WHERE ndocper = $1"
        )
        .bind(ndocper)
        .fetch_optional(&self.db)
        .await?;

        Ok(persona)
    }

    async fn get_by_emaper(&self, emaper: &str) -> Result<Option<Persona>, AppError> {
        let persona = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM personas WHERE emaper = $1"
        )
        .bind(emaper)
        .fetch_optional(&self.db)
        .await?;

        Ok(persona)
    }

    async fn get_all(&self, limit: i64, offset: i64) -> Result<Vec<Persona>, AppError> {
        let personas = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM personas ORDER BY idper LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        Ok(personas)
    }

    async fn get_all_by_idpef(&self, idpef: i64) -> Result<Vec<Persona>, AppError> {
        let personas = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM personas WHERE idpef = $1 AND actper = 1"
        )
        .bind(idpef)
        .fetch_all(&self.db)
        .await?;

        Ok(personas)
    }

    async fn create(&self, persona: Persona) -> Result<Persona, AppError> {
        let resultado = sqlx::query_as::<_, Persona>(
            "INSERT INTO personas (ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             RETURNING idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper"
        )
        .bind(&persona.ndocper)
        .bind(persona.tdocper)
        .bind(&persona.nomper)
        .bind(&persona.apeper)
        .bind(&persona.dirper)
        .bind(&persona.telper)
        .bind(persona.codubi)
        .bind(persona.idpef)
        .bind(&persona.pass)
        .bind(&persona.emaper)
        .bind(persona.actper)
        .fetch_one(&self.db)
        .await?;

        Ok(resultado)
    }

    async fn update(&self, idper: i64, persona: Persona) -> Result<Persona, AppError> {
        let resultado = sqlx::query_as::<_, Persona>(
            "UPDATE personas 
             SET ndocper = $1, tdocper = $2, nomper = $3, apeper = $4, dirper = $5, 
                 telper = $6, codubi = $7, idpef = $8, emaper = $9, actper = $10
             WHERE idper = $11
             RETURNING idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper"
        )
        .bind(&persona.ndocper)
        .bind(persona.tdocper)
        .bind(&persona.nomper)
        .bind(&persona.apeper)
        .bind(&persona.dirper)
        .bind(&persona.telper)
        .bind(persona.codubi)
        .bind(persona.idpef)
        .bind(&persona.emaper)
        .bind(persona.actper)
        .bind(idper)
        .fetch_one(&self.db)
        .await
        .map_err(|_| AppError::NotFound("Persona no encontrada".to_string()))?;

        Ok(resultado)
    }

    async fn delete(&self, idper: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE personas SET actper = 0 WHERE idper = $1")
            .bind(idper)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn activate(&self, idper: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE personas SET actper = 1 WHERE idper = $1")
            .bind(idper)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn deactivate(&self, idper: i64) -> Result<(), AppError> {
        sqlx::query("UPDATE personas SET actper = 0 WHERE idper = $1")
            .bind(idper)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn exists(&self, idper: i64) -> Result<bool, AppError> {
        let resultado =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM personas WHERE idper = $1)")
                .bind(idper)
                .fetch_one(&self.db)
                .await?;

        Ok(resultado)
    }

    async fn change_password(&self, idper: i64, new_password: &str) -> Result<(), AppError> {
        sqlx::query("UPDATE personas SET pass = $1 WHERE idper = $2")
            .bind(new_password)
            .bind(idper)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn count(&self) -> Result<i64, AppError> {
        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM personas")
            .fetch_one(&self.db)
            .await?;

        Ok(total)
    }

    async fn get_active(&self, limit: i64, offset: i64) -> Result<Vec<Persona>, AppError> {
        let personas = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM personas WHERE actper = 1 ORDER BY idper LIMIT $1 OFFSET $2"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        Ok(personas)
    }
}
