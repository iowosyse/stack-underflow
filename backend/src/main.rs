// Proyecto: Sistema de Tickets

use axum::{routing::get, Router, Json};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

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
    // Configuración de permisos CORS para que Vue no sea bloqueado
    let cors = CorsLayer::new()
        .allow_origin(Any) // Permite conexiones desde cualquier IP local
        .allow_methods(Any)
        .allow_headers(Any);

    // Definición de rutas (Endpoints)
    let app = Router::new()
        .route("/api/status", get(health_check))
        .layer(cors);

    let direccion = "0.0.0.0:3000";
    println!("🚀 Backend en Rust inicializado en http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind(direccion).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
