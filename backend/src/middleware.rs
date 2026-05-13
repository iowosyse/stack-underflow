use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use sqlx::PgPool;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UsuarioLogueado {
    pub id: i32,
    pub nombre: String,
    pub rol: String,
    pub empresa_id: i32,
}

impl<S> FromRequestParts<S> for UsuarioLogueado
where
    PgPool: axum::extract::FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .filter(|value| value.starts_with("Bearer "))
            .map(|value| &value[7..]);

        let token = match auth_header {
            Some(t) => t,
            None => return Err((StatusCode::UNAUTHORIZED, "Falta el token de acceso")),
        };

        let pool = PgPool::from_ref(state);

        let usuario = sqlx::query_as!(
            UsuarioLogueado,
            r#"
            SELECT id, nombre, rol::text as "rol!", empresa_id
            FROM usuarios
            WHERE token = $1
            "#,
            token
        )
        .fetch_optional(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error conectando a la BD"))?;

        match usuario {
            Some(u) => Ok(u),
            None => Err((StatusCode::UNAUTHORIZED, "Token inválido o sesión expirada")),
        }
    }
}
