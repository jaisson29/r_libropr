use std::sync::Arc;

use crate::{core::services::persona::PersonaService, domain::db::PersonaRepository};

pub struct AuthService {
    pub persona_service: PersonaService,
}

impl AuthService {
    pub fn new(persona_repo: Arc<dyn PersonaRepository>) -> Self {
        let persona_service = PersonaService::new(persona_repo);
        Self { persona_service }
    }

    pub fn get_permissions(&self, idpef: i64) -> Vec<String> {
        // Lógica para obtener permisos basada en el idper
        // Por simplicidad, retornamos permisos estáticos
        vec!["read".to_string(), "write".to_string()]
    }
}
