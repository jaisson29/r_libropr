use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::Pagper;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,       // Subject (User ID)
    pub exp: usize,     // Expiración
    pub idper: i64,     // ID de la persona
    pub nomper: String, // Nombre de la persona
    pub idpef: i64,     // ID del perfil
    pub nompef: String, // Nombre del perfil
    pub emaper: String, // Email de la persona
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthUser {
    pub idper: i64,                           // ID de la persona
    pub nomper: String,                       // Nombre de la persona
    pub idpef: i64,                           // ID del perfil (rol)
    pub nompef: String,                       // Nombre del perfil (Admin, Usuario, etc.)
    pub is_super_admin: bool,                 // Indicador de super administrador
    pub permissions: HashMap<String, Pagper>, // Permisos cargados desde el servicio
}

impl AuthUser {
    /// Verifica si el usuario tiene un perfil específico
    pub fn has_profile(&self, nombre_perfil: &str) -> bool {
        self.nompef.eq_ignore_ascii_case(nombre_perfil)
    }

    /// Verifica si el usuario es administrador
    pub fn is_admin(&self) -> bool {
        self.has_profile("super_admin") || self.has_profile("admin")
    }

    /// Verifica si el usuario es super administrador
    pub fn is_super_admin(&self) -> bool {
        self.has_profile("super_admin")
    }
}
