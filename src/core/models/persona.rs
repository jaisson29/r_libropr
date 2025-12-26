use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Persona {
    pub idper: i64,
    pub emaper: String,
    pub ndocper: Option<i64>,
    pub tdocper: i64,
    pub nomper: String,
    pub apeper: String,
    pub dirper: Option<String>,
    pub telper: String,
    pub codubi: i64,
    pub idpef: i64,
    pub pass: Option<String>,
    pub actper: i64,
}
