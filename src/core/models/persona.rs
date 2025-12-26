use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Persona {
    pub idper: i64,
    pub ndocper: String,
    pub tdocper: i64,
    pub nomper: String,
    pub apeper: String,
    pub dirper: String,
    pub telper: String,
    pub codubi: i64,
    pub idpef: i64,
    pub pass: String,
    pub emaper: String,
    pub actper: i64,
}
