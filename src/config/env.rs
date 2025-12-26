use serde::Deserialize;

/// Configuración de la aplicación cargada desde .env
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub log_level: String,
    pub app_name: String,
    pub host: String,
    pub app_env: String,
    pub port: u16,
    pub database_url: String,
    pub db_pool_size: u32,
    pub jwt_expiration_hours: u32,
    pub jwt_secret: String,
  }

impl Config {
    /// Carga la configuración desde las variables de entorno
    /// Rust nos fuerza a manejar errores explícitamente - esto previene bugs
    pub fn from_env() -> Result<Self, config::ConfigError> {
        dotenv::dotenv().ok(); // Cargar .env si existe

        let config = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        config.try_deserialize()
    }
}
