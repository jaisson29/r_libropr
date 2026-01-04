use std::sync::Arc;

use crate::{domain::{Persona, db::PersonaRepository}, errors::AppError};

/// Servicio de dominio para Persona
/// Contiene la lógica de negocio y orquesta operaciones del repositorio
pub struct PersonaService {
    persona_repository: Arc<dyn PersonaRepository>,
}

impl PersonaService {
    pub fn new(persona_repository: Arc<dyn PersonaRepository>) -> Self {
        Self { persona_repository }
    }

    /// Listar personas con paginación
    pub async fn list(&self, limit: i64, offset: i64) -> Result<Vec<Persona>, AppError> {
        // Aquí podrías agregar validaciones de negocio
        if limit > 1000 {
            return Err(AppError::BadRequest("Límite máximo de 1000 registros".to_string()));
        }
        
        self.persona_repository.get_all(limit, offset).await
    }

    /// Obtener persona por ID
    pub async fn get_by_id(&self, idper: i64) -> Result<Option<Persona>, AppError> {
        self.persona_repository.get_by_idper(idper).await
    }

    /// Obtener persona por documento
    pub async fn get_by_document(&self, ndocper: &str) -> Result<Option<Persona>, AppError> {
        self.persona_repository.get_by_ndocper(ndocper).await
    }

    /// Obtener persona por email
    pub async fn get_by_email(&self, emaper: &str) -> Result<Option<Persona>, AppError> {
        self.persona_repository.get_by_emaper(emaper).await
    }

    /// Crear nueva persona
    pub async fn create(&self, mut persona: Persona) -> Result<Persona, AppError> {
        // Validaciones de negocio
        persona.nomper = persona.nomper.trim().to_string();
        persona.apeper = persona.apeper.trim().to_string();
        persona.emaper = persona.emaper.trim().to_lowercase();

        if persona.nomper.is_empty() {
            return Err(AppError::BadRequest("El nombre es requerido".to_string()));
        }

        if persona.emaper.is_empty() {
            return Err(AppError::BadRequest("El email es requerido".to_string()));
        }

        // Verificar si el email ya existe
        if let Some(_) = self.persona_repository.get_by_emaper(&persona.emaper).await? {
            return Err(AppError::BadRequest("El email ya está registrado".to_string()));
        }

        self.persona_repository.create(persona).await
    }

    /// Actualizar persona
    pub async fn update(&self, idper: i64, mut persona: Persona) -> Result<Persona, AppError> {
        // Validaciones de negocio
        persona.nomper = persona.nomper.trim().to_string();
        persona.apeper = persona.apeper.trim().to_string();

        if persona.nomper.is_empty() {
            return Err(AppError::BadRequest("El nombre es requerido".to_string()));
        }

        self.persona_repository.update(idper, persona).await
    }

    /// Eliminar persona (soft delete)
    pub async fn delete(&self, idper: i64) -> Result<(), AppError> {
        // Verificar que existe
        if !self.persona_repository.exists(idper).await? {
            return Err(AppError::NotFound("Persona no encontrada".to_string()));
        }

        self.persona_repository.delete(idper).await
    }

    /// Listar personas activas
    pub async fn list_active(&self, limit: i64, offset: i64) -> Result<Vec<Persona>, AppError> {
        self.persona_repository.get_active(limit, offset).await
    }

    /// Contar total de personas
    pub async fn count(&self) -> Result<i64, AppError> {
        self.persona_repository.count().await
    }
}
