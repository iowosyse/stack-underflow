use axum::{extract::State, http::StatusCode, response::Json};
use sqlx::PgPool;
use serde::Serialize;
use crate::middleware::UsuarioLogueado; // Tu extractor de seguridad

#[derive(Serialize)]
pub struct UsuarioLista {
    pub id: i32,
    pub full_name: String,
    pub role: String,
    pub email: Option<String>,
}

pub async fn obtener_directorio(
    State(pool): State<PgPool>,
    usuario_auth: UsuarioLogueado,
) -> Result<Json<Vec<UsuarioLista>>, (StatusCode, String)> {
    
    if usuario_auth.rol != "administrador" && usuario_auth.rol != "soporte" {
        return Err((StatusCode::FORBIDDEN, "No tienes permiso para ver el directorio".to_string()));
    }

    let usuarios = sqlx::query_as!(
        UsuarioLista,
        r#"
        SELECT id, nombre as "full_name!", rol::text as "role!", email 
        FROM usuarios 
        ORDER BY nombre ASC
        "#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(usuarios))
}