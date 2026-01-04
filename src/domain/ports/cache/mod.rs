use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

use crate::errors::AppResult;

#[async_trait]
pub trait CacheRepository: Send + Sync {
  async fn get<T>(&self, key: &str) -> AppResult<Option<T>>
  where
      T: DeserializeOwned + Send + Sync;  
  async fn set<T>(&self, key: &str, value: &T, ttl_seconds: usize) -> AppResult<()>
  where
      T: Serialize + Send + Sync;
  async fn delete(&self, key: &str) -> AppResult<()>;
}
