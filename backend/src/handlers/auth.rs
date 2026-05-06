use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::models::usuario::{CredencialesLogin, RespuestaLogin};

pub async fn login_usuario(
    State(pool): State<PgPool>,
    Json(credenciales): Json<CredencialesLogin>,
) -> Result<Json<RespuestaLogin>, StatusCode> {

    let usuario_encontrado = sqlx::query!(
        r#"
        SELECT id, rol::text as nombre_rol 
        FROM usuarios 
        WHERE email = $1 
          AND password_hash = encode(digest($2, 'sha256'), 'hex') 
          AND activo = true
        "#,
        credenciales.email,
        credenciales.password
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match usuario_encontrado {
        Some(usuario) => {
            let respuesta = RespuestaLogin {
                id: usuario.id,
                rol: usuario.nombre_rol.unwrap_or_default(),
                token: format!("token_acceso_{}", usuario.id), 
            };
            
            Ok(Json(respuesta))
        }
        None => Err(StatusCode::UNAUTHORIZED)
    }
}