use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "grow-map-api",
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/api/health", get(health));

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Grow Map API: http://127.0.0.1:8080");

    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;

    Ok(())
}
