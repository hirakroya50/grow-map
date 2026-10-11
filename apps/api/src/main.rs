use axum::{Json, Router, routing::get};
use grow_map_contract::models::Health;
use tokio::net::TcpListener;

fn app() -> Router {
    Router::new().route("/health/live", get(liveness))
}

async fn liveness() -> Json<Health> {
    // Deserialize into the generated DTO, including its status enum.
    // This avoids guessing the generator's enum naming.
    let response: Health = serde_json::from_value(serde_json::json!({
        "status": "ok"
    }))
    .expect("Liveness response must match the generated Health DTO");

    Json(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Grow Map API: http://127.0.0.1:8080");

    axum::serve(listener, app())
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    #[tokio::test]
    async fn liveness_returns_expected_health_response() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/health/live")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/json"
        );

        let body = to_bytes(response.into_body(), 1024).await.unwrap();

        // Ensure the response can be read as the generated Rust DTO.
        let _: Health = serde_json::from_slice(&body).unwrap();

        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json, serde_json::json!({ "status": "ok" }));
    }

    #[test]
    fn health_dto_rejects_invalid_status() {
        let result = serde_json::from_value::<Health>(serde_json::json!({ "status": "invalid" }));

        assert!(result.is_err());
    }
}
