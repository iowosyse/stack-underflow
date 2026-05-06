use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use sha2::{Sha256, Digest};

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
    State(pool): State<PgPool>,
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
        WHERE email = $1 AND password_hash = $2
        "#,
        payload.email,
        password_hasheada // <--- AQUÍ ES DONDE SE IMPLEMENTA
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 3. Manejamos el resultado
    match usuario {
        Some(u) => {
            let nuevo_token = Uuid::new_v4().to_string();

            sqlx::query!(
                "UPDATE usuarios SET token = $1 WHERE id = $2",
                nuevo_token,
                u.id
            )
            .execute(&pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Error al guardar token: {}", e)))?;

            Ok(Json(LoginResponse {
                token: nuevo_token,
                rol: u.rol,       
                nombre: u.nombre, 
            }))
        }
        None => Err((StatusCode::UNAUTHORIZED, "Credenciales incorrectas".to_string())),
    }
}