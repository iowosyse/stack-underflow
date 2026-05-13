use axum::{extract::{State, Path}, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::AppState;
use crate::middleware::UsuarioLogueado;

#[derive(Serialize)]
pub struct UsuarioLista {
    pub id: i32,
    pub full_name: String,
    pub role: String,
    pub email: Option<String>,
}

#[derive(Deserialize)]
pub struct NuevoUsuarioReq {
    pub full_name: String,
    pub email: String,
    pub role: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct EditarUsuarioReq {
    pub full_name: String,
    pub email: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct RespuestaUsuario {
    pub mensaje: String,
}

pub async fn obtener_directorio(
    State(state): State<AppState>,
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
        WHERE activo = TRUE
          AND empresa_id = $1
        ORDER BY nombre ASC
        "#,
        usuario_auth.empresa_id
    )
    .fetch_all(&state.pool_lector)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(usuarios))
}

pub async fn crear_usuario(
    State(state): State<AppState>,
    usuario_auth: UsuarioLogueado,
    Json(payload): Json<NuevoUsuarioReq>,
) -> Result<Json<RespuestaUsuario>, (StatusCode, String)> {

    if usuario_auth.rol != "administrador" && usuario_auth.rol != "soporte" {
        return Err((StatusCode::FORBIDDEN, "Sin permiso".to_string()));
    }

    // Validar que el email pertenezca al dominio de la empresa
    let empresa = sqlx::query!(
        "SELECT dominio FROM empresas WHERE id = $1",
        usuario_auth.empresa_id
    )
    .fetch_one(&state.pool_lector)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "No se pudo verificar el dominio de la empresa".to_string()))?;

    let email_lower = payload.email.trim().to_lowercase();
    if !email_lower.ends_with(&format!("@{}", empresa.dominio)) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("El correo debe pertenecer al dominio @{}", empresa.dominio),
        ));
    }

    let mut hasher = Sha256::new();
    hasher.update(payload.password.as_bytes());
    let password_hasheada: String = hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect();

    sqlx::query!(
        r#"
        INSERT INTO usuarios (nombre, email, rol, password_hash, empresa_id)
        VALUES ($1, $2, $3::text::rol_usuario, $4, $5)
        "#,
        payload.full_name,
        email_lower,
        payload.role,
        password_hasheada,
        usuario_auth.empresa_id
    )
    .execute(&state.pool_creador)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RespuestaUsuario { mensaje: "Usuario creado correctamente".to_string() }))
}

pub async fn editar_usuario(
    State(state): State<AppState>,
    usuario_auth: UsuarioLogueado,
    Path(id): Path<i32>,
    Json(payload): Json<EditarUsuarioReq>,
) -> Result<Json<RespuestaUsuario>, (StatusCode, String)> {

    if usuario_auth.rol != "administrador" {
        return Err((StatusCode::FORBIDDEN, "Solo ejecutivos pueden editar usuarios".to_string()));
    }

    // Validar dominio del nuevo email
    let empresa = sqlx::query!(
        "SELECT dominio FROM empresas WHERE id = $1",
        usuario_auth.empresa_id
    )
    .fetch_one(&state.pool_lector)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "No se pudo verificar el dominio de la empresa".to_string()))?;

    let email_lower = payload.email.trim().to_lowercase();
    if !email_lower.ends_with(&format!("@{}", empresa.dominio)) {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("El correo debe pertenecer al dominio @{}", empresa.dominio),
        ));
    }

    sqlx::query!(
        r#"
        UPDATE usuarios SET nombre = $1, email = $2, rol = $3::text::rol_usuario
        WHERE id = $4 AND activo = TRUE AND empresa_id = $5
        "#,
        payload.full_name,
        email_lower,
        payload.role,
        id,
        usuario_auth.empresa_id
    )
    .execute(&state.pool_borrador)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RespuestaUsuario { mensaje: "Usuario actualizado".to_string() }))
}

pub async fn eliminar_usuario(
    State(state): State<AppState>,
    usuario_auth: UsuarioLogueado,
    Path(id): Path<i32>,
) -> Result<Json<RespuestaUsuario>, (StatusCode, String)> {

    if usuario_auth.rol != "administrador" {
        return Err((StatusCode::FORBIDDEN, "Solo ejecutivos pueden realizar esta acción".to_string()));
    }

    sqlx::query!(
        "UPDATE usuarios SET activo = FALSE WHERE id = $1 AND empresa_id = $2",
        id,
        usuario_auth.empresa_id
    )
    .execute(&state.pool_borrador)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RespuestaUsuario { mensaje: "Usuario desactivado correctamente".to_string() }))
}
