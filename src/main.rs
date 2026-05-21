use axum::routing::{get, post};
use axum::Router;
use config::health::health_response;
use config::server;
use serde_json::json;

async fn health() -> axum::Json<serde_json::Value> {
    health_response("moderation")
}

async fn create_report() -> axum::Json<serde_json::Value> {
    axum::Json(json!({ "status": "stub", "message": "report creation not implemented" }))
}

async fn list_reports() -> axum::Json<serde_json::Value> {
    axum::Json(json!({ "status": "stub", "reports": [] }))
}

async fn take_action() -> axum::Json<serde_json::Value> {
    axum::Json(json!({ "status": "stub", "message": "moderation action not implemented" }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    server::serve("moderation", Router::new()
        .route("/health", get(health))
        .route("/reports", post(create_report).get(list_reports))
        .route("/action", post(take_action))
    ).await
}
