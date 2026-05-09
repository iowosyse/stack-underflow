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
        ORDER BY nombre ASC
        "#
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

    let mut hasher = Sha256::new();
    hasher.update(payload.password.as_bytes());
    let password_hasheada = hasher.finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    sqlx::query!(
        r#"
        INSERT INTO usuarios (nombre, email, rol, password_hash)
        VALUES ($1, $2, $3::text::rol_usuario, $4)
        "#,
        payload.full_name,
        payload.email,
        payload.role,
        password_hasheada
    )
    .execute(&state.pool_creador)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RespuestaUsuario { mensaje: "Usuario creado correctamente".to_string() }))
}

// PUT /api/usuarios/{id}
pub async fn editar_usuario(
    State(state): State<AppState>,
    usuario_auth: UsuarioLogueado,
    Path(id): Path<i32>,
    Json(payload): Json<EditarUsuarioReq>,
) -> Result<Json<RespuestaUsuario>, (StatusCode, String)> {
    
    if usuario_auth.rol != "administrador" {
        return Err((StatusCode::FORBIDDEN, "Solo ejecutivos pueden editar usuarios".to_string()));
    }

    sqlx::query!(
        r#"
        UPDATE usuarios SET nombre = $1, email = $2, rol = $3::text::rol_usuario
        WHERE id = $4 AND activo = TRUE
        "#,
        payload.full_name, 
        payload.email, 
        payload.role, 
        id
    )
    .execute(&state.pool_borrador)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RespuestaUsuario { mensaje: "Usuario actualizado".to_string() }))
}

// PATCH /api/usuarios/{id}/desactivar
pub async fn eliminar_usuario(
    State(state): State<AppState>,
    usuario_auth: UsuarioLogueado,
    Path(id): Path<i32>,
) -> Result<Json<RespuestaUsuario>, (StatusCode, String)> {
    
    if usuario_auth.rol != "administrador" {
        return Err((StatusCode::FORBIDDEN, "Solo ejecutivos pueden realizar esta acción".to_string()));
    }

    // Soft delete: Actualizamos la bandera, nunca usamos el comando DELETE
    sqlx::query!("UPDATE usuarios SET activo = FALSE WHERE id = $1", id)
    .execute(&state.pool_borrador)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(RespuestaUsuario { mensaje: "Usuario desactivado correctamente" .to_string() }))
}