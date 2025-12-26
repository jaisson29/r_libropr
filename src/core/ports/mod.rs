/// Puertos (Traits) - Definen los contratos que los repositorios deben cumplir
/// Esto es parte de la arquitectura limpia, permitiendo abstracciones sin inyección de dependencias

pub mod persona;

pub use persona::PersonaRepository;
