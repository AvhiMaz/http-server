use crate::{client::get_rpc_client, types::ApiResponse};
use axum::response::Json;

pub async fn get_latest_slot() -> Json<ApiResponse<u64>> {
    let client = get_rpc_client();

    match client.get_slot() {
        Ok(slot) => Json(ApiResponse::success(slot)),
        Err(e) => Json(ApiResponse::error(format!(
            "Failed to get latest slot: {}",
            e
        ))),
    }
}
