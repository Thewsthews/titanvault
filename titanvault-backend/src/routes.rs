use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use crate::handlers::{create_wallet, sign_transaction};

pub fn create_router() -> Router {
    Router::new()
        .route("/wallet", post(create_wallet))
        .route("/sign", post(sign_transaction))
}

#[derive(Deserialize)]
pub struct SignRequest {
    pub private_key: String,
    pub transaction_data: crate::transactions::TransactionRequest,  // Updated to use TransactionRequest
}

#[derive(Serialize)]
pub struct SignResponse {
    pub signed_transaction: crate::transactions::SignedTransaction,  // Updated to use SignedTransaction
}