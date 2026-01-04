pub mod adapters;

use std::{sync::Arc, fmt::Debug};
use sqlx::PgPool;

use crate::core::services::persona::PersonaService;
use crate::core::services::permission::PermissionService;
use crate::domain::db::{PagperRepository, PersonaRepository};
use crate::infra::adapters::db::postgres::{PersonaRepositoryPg, PagperRepositoryPg};

/// Agregador de repositorios para inyección de dependencias
pub struct Repos {
    pub persona: Arc<dyn PersonaRepository>,
    pub pagper: Arc<dyn PagperRepository>,
    // Agregar más repos aquí conforme crezca el proyecto
}

impl Debug for Repos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Repos").finish()
    }
}

impl Clone for Repos {
    fn clone(&self) -> Self {
        Self {
            persona: self.persona.clone(),
            pagper: self.pagper.clone(),
        }
    }
}

/// Agregador de servicios para inyección de dependencias
pub struct Services {
    pub persona: Arc<PersonaService>,
    pub permission: Arc<PermissionService>,
    // Agregar más servicios aquí conforme crezca el proyecto
}

impl Debug for Services {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Services").finish()
    }
}

impl Clone for Services {
    fn clone(&self) -> Self {
        Self {
            persona: self.persona.clone(),
            permission: self.permission.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    // pub db: PgPool,
    pub db: PgPool,
    pub jwt_secret: String,
    pub repos: Arc<Repos>,
    pub services: Arc<Services>,
}

impl AppState {
    /// Constructor que inicializa todos los repositorios y servicios una sola vez
    pub fn new(db: PgPool, jwt_secret: String) -> Self {
        // 1. Construir repositorios
        let persona_repo = Arc::new(PersonaRepositoryPg::new(db.clone())) as Arc<dyn PersonaRepository>;
        let pagper_repo = Arc::new(PagperRepositoryPg::new(db.clone())) as Arc<dyn PagperRepository>;
        
        let repos = Arc::new(Repos {
            persona: persona_repo.clone(),
            pagper: pagper_repo.clone(),
        });

        // 2. Construir servicios inyectando repos
        let persona_service = Arc::new(PersonaService::new(persona_repo));
        let permission_service = Arc::new(PermissionService::new(pagper_repo));
        
        let services = Arc::new(Services {
            persona: persona_service,
            permission: permission_service,
        });

        // 3. Retornar AppState completo
        Self {
            db,
            jwt_secret,
            repos,
            services,
        }
    }
}
