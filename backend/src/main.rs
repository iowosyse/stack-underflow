use axum::{routing::post, Router};
use axum::{routing::get, Json};
use axum::extract::FromRef;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use serde::Serialize;

mod handlers;
mod models;
pub mod middleware;

#[derive(Clone)]
pub struct AppState {
    pub pool_lector: PgPool,
    pub pool_creador: PgPool,
    pub pool_borrador: PgPool,
    pub pool_auth: PgPool,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        // Usamos auth por defecto: permite leer tokens (middleware) y actualizarlos (login)
        app_state.pool_auth.clone()
    }
}

#[derive(Serialize)]
struct StatusServidor {
    estado: String,
    mensaje: String,
}

// Endpoint de prueba
async fn health_check() -> Json<StatusServidor> {
    Json(StatusServidor {
        estado: "OK".to_string(),
        mensaje: "El servidor Rust está vivo y aceptando peticiones.".to_string(),
    })
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let pool_lector = PgPoolOptions::new()
        .max_connections(3) 
        .connect(&env::var("DATABASE_URL_LECTOR").expect("Falta URL Lector"))
        .await
        .expect("No se pudo conectar a la base de datos como lector");

    let pool_creador = PgPoolOptions::new()
        .max_connections(3)
        .connect(&env::var("DATABASE_URL_CREADOR").expect("Falta URL Creador"))
        .await
        .expect("No se pudo conectar a la base de datos como creador");

    let pool_borrador = PgPoolOptions::new()
        .max_connections(2)
        .connect(&env::var("DATABASE_URL_BORRADOR").expect("Falta URL Borrador"))
        .await
        .expect("No se pudo conectar a la base de datos como borrador");

    let pool_auth = PgPoolOptions::new()
        .max_connections(3)
        .connect(&env::var("DATABASE_URL_AUTH").expect("Falta URL Auth"))
        .await
        .expect("No se pudo conectar a la base de datos como auth");

    let state = AppState {
        pool_lector,
        pool_creador,
        pool_borrador,
        pool_auth,
    };

    // 3. Pasamos el state a Axum en lugar del viejo pool
    let app = Router::new()
        .route("/api/login", post(handlers::auth::login_handler))
        .route("/api/status", get(health_check))
        .route("/api/usuarios", get(handlers::users::obtener_directorio))
        // .route("/api/tickets", post(handlers::tickets::crear_ticket))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor backend escuchando en http://{}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}
