use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use crate::wallet::Wallet;
use crate::transaction::Transaction;

#[derive(Serialize)]
struct WalletResponse{
    address: String,
    private_key: String,
    mnemonic: String,
}

pub async fn create_wallet() -> Json<WalletResponse>{
    let wallet_data = wallet::generate_wallet();
    Json(wallet_data)
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

pub async fn sigh_transaction(Json(payload: Json<SignRequest>)-> Json<SignResponse>{
    let transaction = Transaction::new(payload.private_key.clone(), payload.transaction_data.clone());
    let signed_transaction = transaction.sign();
    Json(SignResponse{signed_transaction})
}