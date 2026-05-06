use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// Lo que recibe de Vue
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub contrasena: String,
}

// Lo que le responde a Vue
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
    
    // 1. Buscamos al usuario y casteamos el ENUM a texto directamente en Postgres
    let usuario = sqlx::query!(
        r#"
        SELECT id, nombre, rol::text as "rol!" 
        FROM usuarios 
        WHERE email = $1 AND password_hash = $2
        "#,
        payload.email,
        payload.contrasena // Usamos el nombre que viene desde Vue
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            // 4. Como no son nulos, simplemente pasamos las variables directas
            Ok(Json(LoginResponse {
                token: nuevo_token,
                rol: u.rol,       // ¡Sin unwrap!
                nombre: u.nombre, // ¡Sin unwrap!
            }))
        }
        None => Err((StatusCode::UNAUTHORIZED, "Credenciales incorrectas".to_string())),
    }
}