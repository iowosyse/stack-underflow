use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sha2::{Sha256, Digest};

use crate::AppState;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String, 
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub rol: String,
    pub nombre: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    
    // 1. Convertimos la contraseña a un Hash SHA-256
    let mut hasher = Sha256::new();
    hasher.update(payload.password.as_bytes());
    let password_hasheada = hasher.finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    // 2. Buscamos al usuario usando el HASH generado
    let usuario = sqlx::query!(
        r#"
        SELECT id, nombre, rol::text as "rol!" 
        FROM usuarios 
        WHERE email = $1 AND password_hash = $2 AND activo = TRUE
        "#,
        payload.email,
        password_hasheada 
    )
    .fetch_optional(&state.pool_auth)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 3. Manejamos el resultado
    match usuario {
        Some(u) => {
            let nuevo_token = Uuid::new_v4().to_string();

            sqlx::query!("UPDATE usuarios SET token = $1 WHERE email = $2 AND password_hash = $3", 
                nuevo_token, payload.email, password_hasheada)
                .execute(&state.pool_auth) // Usa pool_auth
                .await
                .map_err(|e| (StatusCode::UNAUTHORIZED, "Error de credenciales".to_string()))?;

            Ok(Json(LoginResponse {
                token: nuevo_token,
                rol: u.rol,       
                nombre: u.nombre, 
            }))
        }
        None => Err((StatusCode::UNAUTHORIZED, "Credenciales incorrectas".to_string())),
    }
}