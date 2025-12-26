/// Adapters - Implementaciones concretas de los Puertos
/// Aquí van los repositorios reales que cumplen con los contratos definidos en los puertos

mod postgres;
pub mod mysql;

pub use postgres::PersonaRepositoryPg;
pub use mysql::PersonaRepositoryMySQL;
