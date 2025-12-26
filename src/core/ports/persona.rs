/// Puerto para operaciones CRUD de Persona
/// Define el contrato que cualquier repositorio de Persona debe cumplir
use crate::core::models::Persona;
use crate::errors::AppError;

#[async_trait::async_trait]
pub trait PersonaRepository: Send + Sync {
    /// Get a person by their ID
    async fn get_by_idper(&self, idper: i64) -> Result<Option<Persona>, AppError>;

    /// Get a person by their document number
    async fn get_by_ndocper(&self, ndocper: &str) -> Result<Option<Persona>, AppError>;

    /// Get a person by their email
    async fn get_by_emaper(&self, emaper: &str) -> Result<Option<Persona>, AppError>;

    /// Get all persons (with possible pagination)
    async fn get_all(&self, limit: i64, offset: i64) -> Result<Vec<Persona>, AppError>;
    /// List persons by profile
    async fn get_all_by_idpef(&self, idpef: i64) -> Result<Vec<Persona>, AppError>;

    /// Create a new person
    async fn create(&self, persona: Persona) -> Result<Persona, AppError>;

    /// Update a person
    async fn update(&self, idper: i64, persona: Persona) -> Result<Persona, AppError>;

    /// Delete a person (logical deletion if actper = 0)
    async fn delete(&self, idper: i64) -> Result<(), AppError>;

    /// Activate a person
    async fn activate(&self, idper: i64) -> Result<(), AppError>;

    /// Deactivate a person
    async fn deactivate(&self, idper: i64) -> Result<(), AppError>;

    /// Check if a person exists
    async fn exists(&self, idper: i64) -> Result<bool, AppError>;

    /// Change a person's password
    async fn change_password(&self, idper: i64, new_password: &str) -> Result<(), AppError>;

    /// Get the total number of persons
    async fn count(&self) -> Result<i64, AppError>;

    /// Get active persons
    async fn get_active(&self, limit: i64, offset: i64) -> Result<Vec<Persona>, AppError>;
}
