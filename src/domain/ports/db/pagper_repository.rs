use async_trait::async_trait;

use crate::{domain::Pagper, errors::AppResult};

/// Puerto (interface) para el repositorio de permisos página-perfil
#[async_trait]
pub trait PagperRepository: Send + Sync {
    /// Obtiene todos los permisos de un perfil específico
    /// Devuelve HashMap: clave = código de página, valor = Pagper
    async fn find_by_perfil(&self, idpef: i64) -> AppResult<Vec<Pagper>>;
    
    /// Verifica si un perfil tiene un permiso específico en una página
    async fn has_permission(
        &self,
        idpef: i64,
        codpag: &str,
        action: &str,
    ) -> AppResult<bool>;
}
