use serde::Serialize;

// Generic API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: u64,
    pub solana_cluster: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub address: String,
    pub balance_lamports: u64,
    pub balance_sol: f64,
}

#[derive(Serialize)]
pub struct AccountInfoResponse {
    pub address: String,
    pub lamports: u64,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Serialize)]
pub struct ValidatorsResponse {
    pub current_validators: usize,
    pub delinquent_validators: usize,
    pub total_stake: u64,
    pub validator_info: Vec<ValidatorInfo>,
}

#[derive(Serialize)]
pub struct ValidatorInfo {
    pub vote_pubkey: String,
    pub node_pubkey: String,
    pub activated_stake: u64,
    pub commission: u8,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub signature: String,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub confirmations: Option<usize>,
}
