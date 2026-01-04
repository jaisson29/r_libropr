use crate::{domain::cache::CacheRepository, errors::{AppError, AppResult}};
use async_trait::async_trait;
use moka::future::Cache;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

#[derive(Clone)]
pub struct MemoryCacheImpl {
    // Guardamos String (JSON) para ser agnósticos del tipo.
    // Esto simula cómo funciona Redis en la vida real.
    inner: Cache<String, String>, 
}

impl MemoryCacheImpl {
    pub fn new() -> Self {
        Self {
            inner: Cache::builder()
                .max_capacity(10_000)
                // Un TTL por defecto (aunque en el set lo podemos sobreescribir si usamos otra lógica)
                // Nota: Moka básico usa TTL global, para TTL por key se requiere configuración extra 
                // o usar Redis. Para este ejemplo simple usamos expiración global de 10 min.
                .time_to_live(Duration::from_secs(600)) 
                .build(),
        }
    }
}

#[async_trait]
impl CacheRepository for MemoryCacheImpl {
    async fn get<T>(&self, key: &str) -> AppResult<Option<T>>
    where
        T: DeserializeOwned + Send,
    {
        // 1. Buscamos el String crudo (JSON)
        if let Some(json_str) = self.inner.get(key).await {
            // 2. Deserializamos al tipo T que pidió el servicio
            let obj: T = serde_json::from_str(&json_str)
                .map_err(|e| AppError::Internal(format!("Error deserializando cache: {}", e)))?;
            
            Ok(Some(obj))
        } else {
            Ok(None)
        }
    }

    async fn set<T>(&self, key: &str, value: &T, _ttl_seconds: usize) -> AppResult<()>
    where
        T: Serialize + Send + Sync,
    {

        // 1. Serializamos el objeto a JSON
        let json_str = serde_json::to_string(value)
            .map_err(|e| AppError::Internal(format!("Error serializando para cache: {}", e)))?;

        // 2. Guardamos en memoria
        // (En una impl real de Redis, aquí usaríamos el comando SETEX con ttl_seconds)
        self.inner.insert(key.to_string(), json_str).await;
        
        Ok(())
    }

    async fn delete(&self, key: &str) -> AppResult<()> {
        self.inner.invalidate(key).await;
        Ok(())
    }
}
