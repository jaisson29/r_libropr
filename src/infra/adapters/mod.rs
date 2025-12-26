/// Adapters - Implementaciones concretas de los Puertos
/// Aquí van los repositorios reales que cumplen con los contratos definidos en los puertos

pub mod persona_repository;

pub use persona_repository::PersonaRepositoryPg;
