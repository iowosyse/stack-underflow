use axum::routing::{get, post, put, patch};
use axum::{Router, Json};
use axum::extract::FromRef;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;

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
        app_state.pool_auth.clone() // <--- Corregido para que use pool_auth en vez de pool_lector
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

async fn conectar_con_reintentos(url: &str, nombre: &str) -> PgPool {
    let max_intentos = 10;
    let espera_seg  = 3;

    for intento in 1..=max_intentos {
        match PgPoolOptions::new()
            .max_connections(3)
            .connect(url)
            .await
        {
            Ok(pool) => {
                println!("✔ Conectado a {} (intento {})", nombre, intento);
                return pool;
            }
            Err(e) => {
                println!("⏳ {} no responde aún (intento {}/{}): {}", nombre, intento, max_intentos, e);
                sleep(Duration::from_secs(espera_seg)).await;
            }
        }
    }
    panic!("✖ No se pudo conectar a {} después de {} intentos.", nombre, max_intentos);
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    let pool_lector  = conectar_con_reintentos(&env::var("DATABASE_URL_LECTOR") .expect("Falta URL Lector"),  "app_lector" ).await;
    let pool_creador = conectar_con_reintentos(&env::var("DATABASE_URL_CREADOR").expect("Falta URL Creador"), "app_creador").await;
    let pool_borrador= conectar_con_reintentos(&env::var("DATABASE_URL_BORRADOR").expect("Falta URL Borrador"),"app_borrador").await;
    let pool_auth    = conectar_con_reintentos(&env::var("DATABASE_URL_AUTH")   .expect("Falta URL Auth"),    "app_auth"   ).await;
    let pool_ping = pool_auth.clone(); // cualquier pool sirve

    let state = AppState {
        pool_lector,
        pool_creador,
        pool_borrador,
        pool_auth,
    };

    let app = Router::new()
        .route("/api/login", post(handlers::auth::login_handler))
        .route("/api/logout", post(handlers::auth::logout_handler))
        .route("/api/status", get(health_check))
        
        .route("/api/usuarios", get(handlers::users::obtener_directorio).post(handlers::users::crear_usuario))
        .route("/api/usuarios/{id}", put(handlers::users::editar_usuario))
        .route("/api/usuarios/{id}/desactivar", patch(handlers::users::eliminar_usuario))
        
        .route("/api/tickets", get(handlers::tickets::obtener_tickets_activos).post(handlers::tickets::crear_ticket))
        .route("/api/tickets/cerrados", get(handlers::tickets::obtener_tickets_cerrados))
        .route("/api/tickets/{id}/asignar", put(handlers::tickets::tomar_ticket))
        .route("/api/tickets/{id}/cerrar", put(handlers::tickets::cerrar_ticket))
        
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor backend escuchando en http://{}", listener.local_addr().unwrap());

    // Keep-alive: evita que Neon entre en auto-suspend
    tokio::spawn(async move {
        let mut intervalo = tokio::time::interval(Duration::from_secs(240)); // 4 min
        loop {
            intervalo.tick().await;
            match sqlx::query("SELECT 1").execute(&pool_ping).await {
                Ok(_)  => println!("Keep-alive OK"),
                Err(e) => println!("Keep-alive falló: {}", e),
            }
        }
    });

    axum::serve(listener, app).await.unwrap();
}