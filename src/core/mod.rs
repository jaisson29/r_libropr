/// Core Domain Layer
/// Contiene modelos de dominio, puertos (abstracciones) y servicios

pub mod models;
pub mod ports;
pub mod services;

// Exportar los principales para acceso directo
pub use models::*;
pub use ports::*;
