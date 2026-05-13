use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::AppState;
use crate::models::empresa::RegistroEmpresaReq;

#[derive(Serialize)]
pub struct RespuestaRegistro {
    pub mensaje: String,
}

#[derive(Deserialize)]
struct TurnstileResp {
    success: bool,
}

pub async fn registrar_empresa(
    State(state): State<AppState>,
    Json(payload): Json<RegistroEmpresaReq>,
) -> Result<Json<RespuestaRegistro>, (StatusCode, String)> {

    // 1. Verificar CAPTCHA con Cloudflare Turnstile
    let secret_key = std::env::var("TURNSTILE_SECRET_KEY")
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "TURNSTILE_SECRET_KEY no configurada".to_string()))?;

    let client = reqwest::Client::new();
    let resp = client
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .form(&[("secret", secret_key.as_str()), ("response", payload.captcha_token.as_str())])
        .send()
        .await
        .map_err(|_| (StatusCode::SERVICE_UNAVAILABLE, "No se pudo verificar el CAPTCHA".to_string()))?;

    let turnstile: TurnstileResp = resp
        .json()
        .await
        .map_err(|_| (StatusCode::SERVICE_UNAVAILABLE, "Respuesta inválida del CAPTCHA".to_string()))?;

    if !turnstile.success {
        return Err((StatusCode::BAD_REQUEST, "CAPTCHA inválido. Intenta de nuevo.".to_string()));
    }

    // 2. Normalizar dominio
    let dominio = payload.dominio.trim().to_lowercase();
    let email = payload.email_admin.trim().to_lowercase();

    // 3. Validar que el correo pertenezca al dominio
    if !email.ends_with(&format!("@{}", dominio)) {
        return Err((
            StatusCode::BAD_REQUEST,
            "El correo del administrador debe pertenecer al dominio institucional indicado".to_string(),
        ));
    }

    // 4. Insertar empresa (el UNIQUE en dominio devuelve error si ya existe)
    let empresa = sqlx::query!(
        "INSERT INTO empresas (nombre, dominio) VALUES ($1, $2) RETURNING id",
        payload.nombre_empresa.trim(),
        dominio
    )
    .fetch_one(&state.pool_creador)
    .await
    .map_err(|e| {
        let msg = e.to_string();
        if msg.contains("unique") || msg.contains("duplicate") || msg.contains("empresas_dominio_key") {
            (StatusCode::CONFLICT, "Ya existe una empresa registrada con ese dominio".to_string())
        } else {
            (StatusCode::INTERNAL_SERVER_ERROR, msg)
        }
    })?;

    // 5. Hashear contraseña
    let mut hasher = Sha256::new();
    hasher.update(payload.password_admin.as_bytes());
    let password_hash: String = hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect();

    // 6. Insertar usuario administrador de la empresa
    sqlx::query!(
        r#"
        INSERT INTO usuarios (nombre, email, rol, password_hash, empresa_id)
        VALUES ($1, $2, 'administrador'::rol_usuario, $3, $4)
        "#,
        payload.nombre_admin.trim(),
        email,
        password_hash,
        empresa.id
    )
    .execute(&state.pool_creador)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RespuestaRegistro {
        mensaje: "Empresa registrada exitosamente. Ya puedes iniciar sesión.".to_string(),
    }))
}
