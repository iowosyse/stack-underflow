use axum::{extract::State, http::StatusCode, response::Json};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::middleware::UsuarioLogueado;

// ── Structs ───────────────────────────────────────────────────────────────────

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TicketRespuesta {
    pub id: i32,
    pub author: String,
    pub subject: String,
    pub category: String,
    pub priority: String,
    pub status: String,
    pub assigned_to: Option<String>,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// ── POST /api/tickets ─────────────────────────────────────────────────────────

pub async fn crear_ticket(
    State(state): State<AppState>,
    usuario: UsuarioLogueado,
    Json(payload): Json<NuevoTicket>,
) -> Result<Json<RespuestaTicket>, (StatusCode, String)> {

    let categoria_lower = payload.categoria.to_lowercase();

    sqlx::query!(
        r#"
        INSERT INTO tickets (autor_id, asunto, categoria, descripcion, empresa_id)
        VALUES ($1, $2, $3::text::categoria_ticket, $4, $5)
        "#,
        usuario.id,
        payload.asunto,
        categoria_lower,
        payload.descripcion,
        usuario.empresa_id
    )
    .execute(&state.pool_creador)
    .await
    .map_err(|e| {
        eprintln!("Error SQL al crear ticket: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Error al guardar el ticket".to_string())
    })?;

    Ok(Json(RespuestaTicket {
        mensaje: "Ticket creado correctamente".to_string(),
    }))
}

// ── GET /api/tickets ──────────────────────────────────────────────────────────

pub async fn obtener_tickets_activos(
    State(state): State<AppState>,
    usuario_auth: UsuarioLogueado,
) -> Result<Json<Vec<TicketRespuesta>>, (StatusCode, String)> {

    let tickets = match usuario_auth.rol.as_str() {
        "administrador" | "soporte" => {
            sqlx::query_as!(
                TicketRespuesta,
                r#"
                SELECT
                    t.id,
                    u_autor.nombre                          AS "author!",
                    t.asunto                                AS "subject!",
                    t.categoria::text                       AS "category!",
                    t.prioridad::text                       AS "priority!",
                    CASE t.estado
                        WHEN 'abierto'      THEN 'disponible'
                        WHEN 'en_progreso'  THEN 'asignado'
                        ELSE t.estado::text
                    END                                     AS "status!",
                    u_asig.nombre                           AS "assigned_to?",
                    t.descripcion                           AS "description!",
                    t.creado_en                             AS "created_at!"
                FROM tickets t
                JOIN  usuarios u_autor ON u_autor.id = t.autor_id
                LEFT JOIN usuarios u_asig  ON u_asig.id  = t.asignado_a_id
                WHERE t.estado IN ('abierto', 'en_progreso')
                  AND t.activo = TRUE
                  AND t.empresa_id = $1
                ORDER BY
                    CASE t.prioridad
                        WHEN 'urgente'  THEN 1
                        WHEN 'alta'     THEN 2
                        WHEN 'moderada' THEN 3
                        WHEN 'baja'     THEN 4
                    END,
                    t.creado_en ASC
                "#,
                usuario_auth.empresa_id
            )
            .fetch_all(&state.pool_lector)
            .await
        }

        _ => {
            sqlx::query_as!(
                TicketRespuesta,
                r#"
                SELECT
                    t.id,
                    u_autor.nombre                          AS "author!",
                    t.asunto                                AS "subject!",
                    t.categoria::text                       AS "category!",
                    t.prioridad::text                       AS "priority!",
                    CASE t.estado
                        WHEN 'abierto'      THEN 'disponible'
                        WHEN 'en_progreso'  THEN 'asignado'
                        ELSE t.estado::text
                    END                                     AS "status!",
                    u_asig.nombre                           AS "assigned_to?",
                    t.descripcion                           AS "description!",
                    t.creado_en                             AS "created_at!"
                FROM tickets t
                JOIN  usuarios u_autor ON u_autor.id = t.autor_id
                LEFT JOIN usuarios u_asig  ON u_asig.id  = t.asignado_a_id
                WHERE t.autor_id = $1
                  AND t.estado IN ('abierto', 'en_progreso')
                  AND t.activo = TRUE
                  AND t.empresa_id = $2
                ORDER BY t.creado_en DESC
                "#,
                usuario_auth.id,
                usuario_auth.empresa_id
            )
            .fetch_all(&state.pool_lector)
            .await
        }
    };

    tickets.map(Json).map_err(|e| {
        eprintln!("Error tickets: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
}

// ── GET /api/tickets/cerrados ─────────────────────────────────────────────────

pub async fn obtener_tickets_cerrados(
    State(state): State<AppState>,
    usuario_auth: UsuarioLogueado,
) -> Result<Json<Vec<TicketRespuesta>>, (StatusCode, String)> {

    let tickets = match usuario_auth.rol.as_str() {

        "administrador" => {
            sqlx::query_as!(
                TicketRespuesta,
                r#"
                SELECT
                    t.id,
                    u_autor.nombre      AS "author!",
                    t.asunto            AS "subject!",
                    t.categoria::text   AS "category!",
                    t.prioridad::text   AS "priority!",
                    'hecho'             AS "status!",
                    u_asig.nombre       AS "assigned_to",
                    t.descripcion       AS "description!",
                    t.creado_en         AS "created_at!"
                FROM tickets t
                JOIN  usuarios u_autor ON u_autor.id = t.autor_id
                LEFT JOIN usuarios u_asig  ON u_asig.id  = t.asignado_a_id
                WHERE t.estado IN ('resuelto', 'cerrado')
                  AND t.activo = TRUE
                  AND t.empresa_id = $1
                ORDER BY t.creado_en DESC
                "#,
                usuario_auth.empresa_id
            )
            .fetch_all(&state.pool_lector)
            .await
        }

        "soporte" => {
            sqlx::query_as!(
                TicketRespuesta,
                r#"
                SELECT
                    t.id,
                    u_autor.nombre      AS "author!",
                    t.asunto            AS "subject!",
                    t.categoria::text   AS "category!",
                    t.prioridad::text   AS "priority!",
                    'hecho'             AS "status!",
                    u_asig.nombre       AS "assigned_to",
                    t.descripcion       AS "description!",
                    t.creado_en         AS "created_at!"
                FROM tickets t
                JOIN  usuarios u_autor ON u_autor.id = t.autor_id
                LEFT JOIN usuarios u_asig  ON u_asig.id  = t.asignado_a_id
                WHERE t.estado IN ('resuelto', 'cerrado')
                  AND t.asignado_a_id = $1
                  AND t.activo = TRUE
                  AND t.empresa_id = $2
                ORDER BY t.creado_en DESC
                "#,
                usuario_auth.id,
                usuario_auth.empresa_id
            )
            .fetch_all(&state.pool_lector)
            .await
        }

        _ => {
            sqlx::query_as!(
                TicketRespuesta,
                r#"
                SELECT
                    t.id,
                    u_autor.nombre      AS "author!",
                    t.asunto            AS "subject!",
                    t.categoria::text   AS "category!",
                    t.prioridad::text   AS "priority!",
                    'hecho'             AS "status!",
                    u_asig.nombre       AS "assigned_to",
                    t.descripcion       AS "description!",
                    t.creado_en         AS "created_at!"
                FROM tickets t
                JOIN  usuarios u_autor ON u_autor.id = t.autor_id
                LEFT JOIN usuarios u_asig  ON u_asig.id  = t.asignado_a_id
                WHERE t.estado IN ('resuelto', 'cerrado')
                  AND t.autor_id = $1
                  AND t.activo = TRUE
                  AND t.empresa_id = $2
                ORDER BY t.creado_en DESC
                "#,
                usuario_auth.id,
                usuario_auth.empresa_id
            )
            .fetch_all(&state.pool_lector)
            .await
        }
    };

    tickets.map(Json).map_err(|e| {
        eprintln!("Error tickets: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })
}

// ── PUT /api/tickets/:id/asignar ─────────────────────────────────────────────

pub async fn tomar_ticket(
    State(state): State<AppState>,
    usuario_auth: UsuarioLogueado,
    Path(id): Path<i32>,
) -> Result<Json<RespuestaTicket>, (StatusCode, String)> {

    if usuario_auth.rol != "soporte" && usuario_auth.rol != "administrador" {
        return Err((StatusCode::FORBIDDEN, "Sin permiso".to_string()));
    }

    let resultado = sqlx::query!(
        r#"
        UPDATE tickets
        SET estado = 'en_progreso',
            asignado_a_id = $1
        WHERE id = $2
          AND estado = 'abierto'
          AND activo = TRUE
          AND empresa_id = $3
        "#,
        usuario_auth.id,
        id,
        usuario_auth.empresa_id
    )
    .execute(&state.pool_borrador)
    .await
    .map_err(|e| {
        eprintln!("Error al tomar ticket {}: {}", id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    if resultado.rows_affected() == 0 {
        return Err((StatusCode::CONFLICT, "El ticket no existe, ya fue tomado o no está disponible".to_string()));
    }

    Ok(Json(RespuestaTicket { mensaje: "Ticket asignado correctamente".to_string() }))
}

// ── PUT /api/tickets/:id/cerrar ───────────────────────────────────────────────

pub async fn cerrar_ticket(
    State(state): State<AppState>,
    usuario_auth: UsuarioLogueado,
    Path(id): Path<i32>,
) -> Result<Json<RespuestaTicket>, (StatusCode, String)> {

    if usuario_auth.rol != "soporte" && usuario_auth.rol != "administrador" {
        return Err((StatusCode::FORBIDDEN, "Sin permiso".to_string()));
    }

    let resultado = sqlx::query!(
        r#"
        UPDATE tickets
        SET estado = 'resuelto'
        WHERE id = $1
          AND asignado_a_id = $2
          AND estado = 'en_progreso'
          AND activo = TRUE
          AND empresa_id = $3
        "#,
        id,
        usuario_auth.id,
        usuario_auth.empresa_id
    )
    .execute(&state.pool_borrador)
    .await
    .map_err(|e| {
        eprintln!("Error al cerrar ticket {}: {}", id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    if resultado.rows_affected() == 0 {
        return Err((StatusCode::CONFLICT, "El ticket no existe, no está asignado a ti o ya fue resuelto".to_string()));
    }

    Ok(Json(RespuestaTicket { mensaje: "Ticket marcado como resuelto".to_string() }))
}
