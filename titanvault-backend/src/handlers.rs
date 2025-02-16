use axum::{Json, Router};
use serde::{Deserialize, Serialize};
// use crate::wallet::Wallet; // Commented out as it is not used
use ethers::types::Transaction;

use crate::wallet;

#[derive(Serialize)]
struct WalletResponse{
    address: String,
    private_key: String,
    mnemonic: String,
}

pub async fn create_wallet() -> Json<WalletResponse>{
    let wallet_data = wallet::generate_wallet();
    Json(WalletResponse {
        address: wallet_data.address,
        private_key: wallet_data.private_key,
        mnemonic: wallet_data.mnemonic,
    })
}

#[derive(Deserialize)]
struct SignRequest{
    private_key: String,
    transaction_data: String,
}

#[derive(Serialize)]

struct SignResponse{
    signed_transaction: String,
}

pub async fn sign_transaction(Json(payload): Json<SignRequest>) -> Json<SignResponse> {
    let transaction = Transaction::new(payload.private_key.clone(), payload.transaction_data.clone());
    let signed_transaction = transaction.sign();
    Json(SignResponse{signed_transaction})
}