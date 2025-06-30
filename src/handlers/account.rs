use crate::{
    client::get_rpc_client,
    types::{AccountInfoResponse, ApiResponse},
};
use axum::{extract::Path, response::Json};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub async fn get_account_info(
    Path(address): Path<String>,
) -> Json<ApiResponse<AccountInfoResponse>> {
    let client = get_rpc_client();

    match Pubkey::from_str(&address) {
        Ok(pubkey) => match client.get_account(&pubkey) {
            Ok(account) => {
                let account_data = AccountInfoResponse {
                    address,
                    lamports: account.lamports,
                    owner: account.owner.to_string(),
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                };

                Json(ApiResponse::success(account_data))
            }
            Err(e) => Json(ApiResponse::error(format!(
                "Failed to get account info: {}",
                e
            ))),
        },
        Err(_) => Json(ApiResponse::error("Invalid public key format".to_string())),
    }
}
