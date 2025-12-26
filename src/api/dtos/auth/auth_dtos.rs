use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequestDTO {
  pub email: String,
  pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponseDTO {
    pub token: String,
}
