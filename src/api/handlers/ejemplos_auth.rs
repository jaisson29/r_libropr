/// Ejemplos de Handlers Públicos vs Privados
/// 
/// DIFERENCIAS CLAVE:
/// 1. Handlers PÚBLICOS: NO tienen el parámetro `AuthUser`
/// 2. Handlers PRIVADOS: SÍ tienen el parámetro `AuthUser`

use std::sync::Arc;
use axum::{
    Json,
    extract::{State, Path},
};
use serde_json::json;

use crate::{
    api::middleware::auth::AuthUser,
    infra::AppState,
    errors::AppError,
};

// ============================================================
// RUTAS PÚBLICAS - Sin autenticación
// ============================================================

/// ✅ PÚBLICO: Health check - Cualquiera puede acceder
/// GET /health
pub async fn health_check() -> &'static str {
    "OK"
}

/// ✅ PÚBLICO: Login - No necesita token (aquí es donde se genera el token)
/// POST /api/auth/login
pub async fn login(
    State(_state): State<Arc<AppState>>,
    // body: Json<LoginDto>
) -> Result<Json<serde_json::Value>, AppError> {
    // Aquí validarías email/password y generarías el JWT
    Ok(Json(json!({
        "token": "jwt_token_aqui",
        "message": "Login exitoso"
    })))
}

/// ✅ PÚBLICO: Registro - No necesita token
/// POST /api/auth/registro
pub async fn registro(
    State(_state): State<Arc<AppState>>,
    // body: Json<RegistroDto>
) -> Result<Json<serde_json::Value>, AppError> {
    // Crear nuevo usuario en la BD
    Ok(Json(json!({
        "message": "Usuario registrado exitosamente"
    })))
}

// ============================================================
// RUTAS PRIVADAS - Requieren autenticación JWT
// ============================================================

/// 🔒 PRIVADO: Obtener perfil del usuario autenticado
/// GET /api/user/me
/// 
/// La presencia de `AuthUser` hace que Axum automáticamente:
/// 1. Verifique el header Authorization
/// 2. Valide el JWT
/// 3. Extraiga los datos del usuario
/// 4. Si algo falla, retorna 401 Unauthorized ANTES de entrar al handler
pub async fn obtener_mi_perfil(
    auth_user: AuthUser,  // ⬅️ Esto activa la autenticación
) -> Json<serde_json::Value> {
    Json(json!({
        "idper": auth_user.idper,
        "nomper": auth_user.nomper,
        "idpef": auth_user.idpef,
        "nompef": auth_user.nompef,
    }))
}

/// 🔒 PRIVADO: Actualizar perfil
/// PUT /api/user/me
pub async fn actualizar_mi_perfil(
    auth_user: AuthUser,  // ⬅️ Autenticación requerida
    State(_state): State<Arc<AppState>>,
    // body: Json<ActualizarPerfilDto>
) -> Result<Json<serde_json::Value>, AppError> {
    // Solo puede actualizar su propio perfil (auth_user.idper)
    Ok(Json(json!({
        "message": format!("Perfil de {} actualizado", auth_user.nomper)
    })))
}

/// 🔒 PRIVADO: Listar personas (solo usuarios autenticados)
/// GET /api/personas
pub async fn listar_personas(
    auth_user: AuthUser,  // ⬅️ Debe estar autenticado
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Dependiendo del perfil, mostrar diferentes datos
    if auth_user.es_admin() {
        // Admin ve todo
        Ok(Json(json!({
            "data": [],
            "mensaje": "Vista completa para administrador"
        })))
    } else {
        // Usuario normal ve solo su info
        Ok(Json(json!({
            "data": [],
            "mensaje": "Vista limitada para usuario"
        })))
    }
}

/// 🔒🛡️ PRIVADO + ADMIN: Dashboard de administrador
/// GET /api/admin/dashboard
/// 
/// Este handler tiene DOS niveles de protección:
/// 1. AuthUser verifica que esté autenticado
/// 2. es_admin() verifica que tenga permisos de admin
pub async fn admin_dashboard(
    auth_user: AuthUser,  // ⬅️ Nivel 1: Autenticación
    State(_state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Nivel 2: Verificar permisos
    if !auth_user.es_admin() {
        return Err(AppError::Forbidden(
            "Se requieren permisos de administrador".to_string()
        ));
    }

    // Solo llega aquí si está autenticado Y es admin
    Ok(Json(json!({
        "mensaje": "Bienvenido al panel de administrador",
        "admin": auth_user.nomper
    })))
}

/// 🔒 PRIVADO: Obtener una persona por ID
/// GET /api/personas/:id
pub async fn obtener_persona(
    auth_user: AuthUser,  // ⬅️ Autenticación requerida
    State(_state): State<Arc<AppState>>,
    Path(id): Path<u32>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Los usuarios solo pueden ver su propia info
    // Los admins pueden ver cualquier info
    if auth_user.idper != id && !auth_user.es_admin() {
        return Err(AppError::Forbidden(
            "No tienes permiso para ver esta información".to_string()
        ));
    }

    Ok(Json(json!({
        "id": id,
        "mensaje": "Persona encontrada"
    })))
}

// ============================================================
// RESUMEN DE DIFERENCIAS
// ============================================================

/*
┌─────────────────────────────────────────────────────────────┐
│                    RUTAS PÚBLICAS                           │
├─────────────────────────────────────────────────────────────┤
│ ✅ NO requieren token JWT                                   │
│ ✅ Cualquiera puede acceder                                 │
│ ✅ NO tienen parámetro `AuthUser`                           │
│ ✅ Ejemplos: /health, /login, /registro                     │
├─────────────────────────────────────────────────────────────┤
│ pub async fn handler(                                       │
│     State(state): State<Arc<AppState>>,                     │
│ ) -> Result<Json<T>, AppError> { ... }                      │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                    RUTAS PRIVADAS                           │
├─────────────────────────────────────────────────────────────┤
│ 🔒 SÍ requieren token JWT                                   │
│ 🔒 Solo usuarios autenticados                               │
│ 🔒 TIENEN parámetro `AuthUser`                              │
│ 🔒 Ejemplos: /api/user/me, /api/personas                    │
├─────────────────────────────────────────────────────────────┤
│ pub async fn handler(                                       │
│     auth_user: AuthUser,  ⬅️ ESTO ES LA DIFERENCIA         │
│     State(state): State<Arc<AppState>>,                     │
│ ) -> Result<Json<T>, AppError> { ... }                      │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                RUTAS PRIVADAS + PERMISOS                    │
├─────────────────────────────────────────────────────────────┤
│ 🔒🛡️ Requieren token JWT + permisos específicos            │
│ 🔒🛡️ Solo usuarios con ciertos roles                       │
│ 🔒🛡️ Tienen `AuthUser` + verificación de permisos          │
│ 🔒🛡️ Ejemplos: /api/admin/*, acciones privilegiadas        │
├─────────────────────────────────────────────────────────────┤
│ pub async fn handler(                                       │
│     auth_user: AuthUser,                                    │
│     State(state): State<Arc<AppState>>,                     │
│ ) -> Result<Json<T>, AppError> {                            │
│     if !auth_user.es_admin() {                              │
│         return Err(AppError::Forbidden(...));               │
│     }                                                        │
│     // ...                                                   │
│ }                                                            │
└─────────────────────────────────────────────────────────────┘

FLUJO DE AUTENTICACIÓN:
1. Cliente hace request con header: Authorization: Bearer <token>
2. Axum detecta que el handler tiene parámetro `AuthUser`
3. Axum ejecuta automáticamente `from_request_parts`
4. Se valida el JWT y extrae los datos
5. Si es válido: el handler recibe el AuthUser con los datos
6. Si es inválido: retorna 401 Unauthorized SIN ejecutar el handler

NO NECESITAS:
❌ Middleware explícito layer
❌ Guards o decoradores
❌ Configuración adicional

SOLO NECESITAS:
✅ Agregar `auth_user: AuthUser` como parámetro del handler
*/
