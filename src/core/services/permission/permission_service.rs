use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::{
    domain::{Pagper, db::PagperRepository},
    errors::AppResult,
};

/// Servicio de permisos con caché en memoria
pub struct PermissionService {
    repo: Arc<dyn PagperRepository>,
    // Caché: clave = idpef (perfil), valor = HashMap<codpag, Permissions>
    cache: Arc<RwLock<HashMap<i64, HashMap<String, Pagper>>>>,
}

impl PermissionService {
    pub fn new(repo: Arc<dyn PagperRepository>) -> Self {
        Self {
            repo,
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Obtiene los permisos de un perfil (con caché)
    pub async fn get_permissions_for_profile(
        &self,
        idpef: i64,
    ) -> AppResult<HashMap<String, Pagper>> {
        // 1. Revisar caché
        {
            let cache_read = self.cache.read().await;
            if let Some(permissions) = cache_read.get(&idpef) {
                tracing::debug!("Permisos obtenidos desde caché para perfil {}", idpef);
                return Ok(permissions.clone());
            }
        }

        // 2. No está en caché, consultar DB
        tracing::debug!("Consultando permisos desde DB para perfil {}", idpef);
        let pagpers = self.repo.find_by_perfil(idpef).await?;

        // 3. Convertir a HashMap<String, Permissions>
        let mut permissions_map = HashMap::new();
        for pagper in pagpers {
            permissions_map.insert(
                pagper.idpag.to_string(),
                Pagper {
                    idpef: pagper.idpef,
                    idpag: pagper.idpag,
                    can_create: pagper.can_create,
                    can_read: pagper.can_read,
                    can_update: pagper.can_update,
                    can_delete: pagper.can_delete,
                },
            );
        }

        // 4. Guardar en caché
        {
            let mut cache_write = self.cache.write().await;
            cache_write.insert(idpef, permissions_map.clone());
        }

        tracing::info!("Permisos cargados y cacheados para perfil {}", idpef);
        Ok(permissions_map)
    }

    /// Limpia el caché de un perfil específico
    pub async fn clear_cache_for_profile(&self, idpef: i64) {
        let mut cache_write = self.cache.write().await;
        cache_write.remove(&idpef);
        tracing::info!("Caché limpiado para perfil {}", idpef);
    }

    /// Limpia todo el caché
    pub async fn clear_all_cache(&self) {
        let mut cache_write = self.cache.write().await;
        cache_write.clear();
        tracing::info!("Caché de permisos completamente limpiado");
    }

    /// Verifica si un perfil tiene un permiso específico
    pub async fn has_permission(&self, idpef: i64, codpag: &str, action: &str) -> AppResult<bool> {
        let permissions = self.get_permissions_for_profile(idpef).await?;

        if let Some(perms) = permissions.get(codpag) {
            let has_perm = match action {
                "create" => perms.can_create,
                "read" => perms.can_read,
                "update" => perms.can_update,
                "delete" => perms.can_delete,
                _ => false,
            };
            Ok(has_perm)
        } else {
            Ok(false)
        }
    }
}
