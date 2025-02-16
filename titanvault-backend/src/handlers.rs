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
}
