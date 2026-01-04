use sqlx::MySqlPool;

use crate::errors::{AppError, AppResult};
use crate::{domain::Persona, domain::db::PersonaRepository};

pub struct PersonaRepositoryMySQL {
    db: MySqlPool,
}

impl PersonaRepositoryMySQL {
    pub fn new(db: MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait::async_trait]
impl PersonaRepository for PersonaRepositoryMySQL {
    async fn get_by_idper(&self, idper: i64) -> AppResult<Option<Persona>> {
        let persona = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM persona 
             WHERE idper = ?"
        )
        .bind(idper)
        .fetch_optional(&self.db)
        .await?;

        Ok(persona)
    }

    async fn get_by_ndocper(&self, ndocper: &str) -> AppResult<Option<Persona>> {
        let persona = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM persona WHERE ndocper = ?"
        )
        .bind(ndocper)
        .fetch_optional(&self.db)
        .await?;

        Ok(persona)
    }

    async fn get_by_emaper(&self, emaper: &str) -> AppResult<Option<Persona>> {
        let persona = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM persona WHERE emaper = ?"
        )
        .bind(emaper)
        .fetch_optional(&self.db)
        .await?;

        Ok(persona)
    }

    async fn get_all(&self, limit: i64, offset: i64) -> AppResult<Vec<Persona>> {
        let personas = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM persona ORDER BY idper LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        Ok(personas)
    }

    async fn get_all_by_idpef(&self, idpef: i64) -> AppResult<Vec<Persona>> {
        let personas = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM persona WHERE idpef = ? AND actper = 1"
        )
        .bind(idpef)
        .fetch_all(&self.db)
        .await?;

        Ok(personas)
    }

    async fn create(&self, persona: Persona) -> AppResult<Persona> {
        let resultado = sqlx::query_as::<_, Persona>(
            "INSERT INTO persona (ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

    async fn update(&self, idper: i64, persona: Persona) -> AppResult<Persona> {
        let resultado = sqlx::query_as::<_, Persona>(
            "UPDATE persona 
             SET ndocper = ?, tdocper = ?, nomper = ?, apeper = ?, dirper = ?, 
                 telper = ?, codubi = ?, idpef = ?, emaper = ?, actper = ?
             WHERE idper = ?
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

    async fn delete(&self, idper: i64) -> AppResult<()> {
        sqlx::query("UPDATE persona SET actper = 0 WHERE idper = ?")
            .bind(idper)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn activate(&self, idper: i64) -> AppResult<()> {
        sqlx::query("UPDATE persona SET actper = 1 WHERE idper = ?")
            .bind(idper)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn deactivate(&self, idper: i64) -> AppResult<()> {
        sqlx::query("UPDATE persona SET actper = 0 WHERE idper = ?")
            .bind(idper)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn exists(&self, idper: i64) -> AppResult<bool> {
        let resultado =
            sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM persona WHERE idper = ?)")
                .bind(idper)
                .fetch_one(&self.db)
                .await?;

        Ok(resultado)
    }

    async fn change_password(&self, idper: i64, new_password: &str) -> AppResult<()> {
        sqlx::query("UPDATE persona SET pass = ? WHERE idper = ?")
            .bind(new_password)
            .bind(idper)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn count(&self) -> AppResult<i64> {
        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM persona")
            .fetch_one(&self.db)
            .await?;

        Ok(total)
    }

    async fn get_active(&self, limit: i64, offset: i64) -> AppResult<Vec<Persona>> {
        let personas = sqlx::query_as::<_, Persona>(
            "SELECT idper, ndocper, tdocper, nomper, apeper, dirper, telper, codubi, idpef, pass, emaper, actper 
             FROM persona WHERE actper = 1 ORDER BY idper LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await?;

        Ok(personas)
    }
}
