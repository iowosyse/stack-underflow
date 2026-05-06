use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id: u32,
    pub nombre: String,
    pub email: String,
    pub password_hash: String,
    // Los ENUMs de Postgres los manejaremos como String en Rust por simplicidad
    pub rol: String, 
    pub activo: bool,
    pub creado_en: Option<DateTime<Utc>>,
    pub actualizado_en: Option<DateTime<Utc>>,
}

// Estructura para lo que recibe el endpoint de login
#[derive(Deserialize)]
pub struct CredencialesLogin {
    pub email: String,
    pub password: String,
}

// Estructura para lo que devuelve el endpoint de login
#[derive(Serialize)]
pub struct RespuestaLogin {
    pub id: i32,
    pub rol: String,
    pub token: String,
}