use axum::{extract::Path, response::Json};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;
use crate::{
    client::get_rpc_client,
    types::{ApiResponse, TransactionResponse},
};

pub async fn get_transaction(Path(signature): Path<String>) -> Json<ApiResponse<TransactionResponse>> {
    let client = get_rpc_client();

    match Signature::from_str(&signature) {
        Ok(sig) => {
            match client.get_transaction_with_config(
                &sig,
                solana_client::rpc_config::RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Json),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: Some(0),
                },
            ) {
                Ok(transaction) => {
                    let tx_data = TransactionResponse {
                        signature,
                        slot: transaction.slot,
                        block_time: transaction.block_time,
                        confirmations: None, // Would need additional call to calculate
                    };

                    Json(ApiResponse::success(tx_data))
                }
                Err(e) => Json(ApiResponse::error(format!("Failed to get transaction: {}", e))),
            }
        }
        Err(_) => Json(ApiResponse::error("Invalid signature format".to_string())),
    }
}