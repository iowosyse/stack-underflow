use axum::{routing::post, Router};
use axum::{routing::get, Json};
use sqlx::postgres::PgPoolOptions;
use std::env;
use serde::Serialize;

mod handlers;
mod models;

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
    
    let db_url = env::var("DATABASE_URL")
        .expect("La variable DATABASE_URL_ADMIN no se encontró en el archivo .env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("No se pudo conectar a la base de datos de Neon");

    let app = Router::new()
        .route("/api/login", post(handlers::auth::login_usuario))
        .route("/api/status", get(health_check))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Servidor backend escuchando en http://{}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}