use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Ticket {
    pub id: i32,
    pub autor_id: i32,
    pub asignado_a_id: Option<i32>, // Option maneja los valores NULL de SQL
    pub asunto: String,
    pub descripcion: String,
    pub categoria: String,
    pub estado: String,
    pub prioridad: String,
    pub activo: bool,
    pub creado_en: Option<DateTime<Utc>>,
    pub actualizado_en: Option<DateTime<Utc>>,
}