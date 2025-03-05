use axum::Json;
use serde::{Deserialize, Serialize};
use alloy_core::primitives::TxEnvelope;
use alloy_primitive::hex;

use crate::wallet; //Created in the previous base

#[derive(Deserialize)]
struct WalletResponse {
    address: String,
    private_key: String,
    mnemonic: String,
}

pub async fn create_wallet() -> Json<WalletResponse> {
    let wallet_date = wallet::generate_wallet();
    Json(WalletResponse{
        address: wallet_data.address,
        private_key: wallet_data.private_key,
        mnemonic: wallet_data.mnemonic,
    })
}

#[derive(Deserialize)]
struct SignRequest{
    private_key: String,
    transaction_date: String,
}

#[derive(Serialize)]
struct Signresponse{
    signed_transaction: String,
}

pub async fn sign_transaction(Json(payload): Json<SignRequest>) -> Json<SignResponse>{
    // This part is responsible for the parsing of transaction data into a TxEnvelope
    let tx_envelope: TxEnvelope = serde_json::from_str(&payload.transaction_data)
    .expect("Failed to parse transaction data");

    //Signs the transaction into the wallet module
    let signed_transaction = wallet::sign_transaction(&payload.private_key, &tx_envelope)
    .await.expect("Failed to sign transaction");
Json(SignResponse{
    signed_transaction,})
}