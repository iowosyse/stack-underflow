use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::middleware::UsuarioLogueado;

#[derive(Deserialize)]
pub struct NuevoTicket {
    pub asunto: String,
    pub categoria: String,
    pub descripcion: String,
}

#[derive(Serialize)]
pub struct RespuestaTicket {
    pub mensaje: String,
}

pub async fn crear_ticket(
    State(state): State<AppState>, 
    usuario: UsuarioLogueado, // El middleware nos da el ID real
    Json(payload): Json<NuevoTicket>,
) -> Result<Json<RespuestaTicket>, (StatusCode, String)> {
    
    // Usamos pool_creador y dejamos que Postgres ponga los DEFAULT en estado, prioridad y fechas
    sqlx::query!(
        r#"
        INSERT INTO tickets (autor_id, asunto, categoria, descripcion)
        VALUES ($1, $2, $3::text::categoria_ticket, $4)
        "#,
        usuario.id,
        payload.asunto,
        payload.categoria, 
        payload.descripcion
    )
    .execute(&state.pool_creador)
    .await
    .map_err(|e| {
        eprintln!("Error SQL: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Error al guardar el ticket en Neon".to_string())
    })?;

    Ok(Json(RespuestaTicket {
        mensaje: "Ticket creado correctamente".to_string(),
    }))
}
