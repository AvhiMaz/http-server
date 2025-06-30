use crate::handlers::{
    docs, get_account_info, get_balance, get_latest_slot, get_transaction, get_validators,
    health_check,
};
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(docs))
        .route("/health", get(health_check))
        .route("/balance/:address", get(get_balance))
        .route("/account/:address", get(get_account_info))
        .route("/transaction/:signature", get(get_transaction))
        .route("/validators", get(get_validators))
        .route("/slot", get(get_latest_slot))
}
