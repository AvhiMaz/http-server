use axum::response::Json;
use crate::{
    client::get_rpc_client,
    types::{ApiResponse, ValidatorsResponse, ValidatorInfo},
};

pub async fn get_validators() -> Json<ApiResponse<ValidatorsResponse>> {
    let client = get_rpc_client();

    match client.get_vote_accounts() {
        Ok(vote_accounts) => {
            let current_validators = vote_accounts.current.len();
            let delinquent_validators = vote_accounts.delinquent.len();

            // Calculate total stake and collect validator info
            let mut total_stake = 0u64;
            let mut validator_info = Vec::new();

            // Take first 10 validators for the response (to avoid huge payloads)
            for (_i, validator) in vote_accounts.current.iter().take(10).enumerate() {
                total_stake += validator.activated_stake;
                validator_info.push(ValidatorInfo {
                    vote_pubkey: validator.vote_pubkey.clone(),
                    node_pubkey: validator.node_pubkey.clone(),
                    activated_stake: validator.activated_stake,
                    commission: validator.commission,
                });
            }

            let validators_data = ValidatorsResponse {
                current_validators,
                delinquent_validators,
                total_stake,
                validator_info,
            };

            Json(ApiResponse::success(validators_data))
        }
        Err(e) => Json(ApiResponse::error(format!("Failed to get validators info: {}", e))),
    }
}