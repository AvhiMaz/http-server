use crate::{
    client::get_rpc_client,
    types::{ApiResponse, BalanceResponse},
};
use axum::{extract::Path, response::Json};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub async fn get_balance(Path(address): Path<String>) -> Json<ApiResponse<BalanceResponse>> {
    let client = get_rpc_client();

    match Pubkey::from_str(&address) {
        Ok(pubkey) => {
            match client.get_balance(&pubkey) {
                Ok(balance_lamports) => {
                    let balance_sol = balance_lamports as f64 / 1_000_000_000.0;

                    let balance_data = BalanceResponse {
                        address,
                        balance_lamports,
                        balance_sol,
                    };

                    Json(ApiResponse::success(balance_data))
                }
                Err(e) => Json(ApiResponse::error(format!("Failed to get balance: {}", e))),
            }
        }
        Err(_) => Json(ApiResponse::error("Invalid public key format".to_string())),
    }
}
