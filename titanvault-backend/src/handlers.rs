use axum::Json;
use serde::Serialize;
use alloy_core::TxEnvelope;
use crate::{wallet, transactions};

#[derive(Serialize)]
pub struct WalletResponse {
    address: String,
    
}

pub async fn create_wallet() -> Json<WalletResponse> {
    let wallet_data = wallet::generate_wallet();
    Json(WalletResponse {
        address: wallet_data.address,
    })
}

pub async fn sign_transaction(Json(payload): Json<routes::SignRequest>) -> Result<Json<routes::SignResponse>, axum::http::StatusCode> {
    // Create TxEnvelope from TransactionRequest
    let tx_envelope: TxEnvelope = transactions::create_transaction(payload.transaction_data)
        .map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    
    // Sign the transaction
    let signed_tx = wallet::sign_transaction(&payload.private_key, &tx_envelope)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Format the signed transaction
    let formatted_tx = transactions::format_signed_transaction(signed_tx)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(routes::SignResponse { signed_transaction: formatted_tx }))
}