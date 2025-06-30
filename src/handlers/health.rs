use crate::types::{ApiResponse, HealthResponse};
use axum::response::Json;

pub async fn health_check() -> Json<ApiResponse<HealthResponse>> {
    let health_data = HealthResponse {
        status: "healthy".to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        solana_cluster: "devnet".to_string(),
    };

    Json(ApiResponse::success(health_data))
}
