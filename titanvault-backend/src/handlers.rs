use axum::Json;
use serde::Serialize;
use alloy_core::TxEnvelope;
use crate::{wallet, transactions};

#[derive(Serialize)]
pub struct WalletResponse {
    address: String,
    error: String,
}

pub async fn create_wallet() -> Json<WalletResponse> {
    let wallet_data = wallet::generate_wallet();
    Json(WalletResponse {
        address: wallet_data.address,
    })
}

pub async fn sign_transaction(Json(payload): Json<routes::SignRequest>) -> Result<Json<routes::SignResponse>, (StatusCode, Json<ErrorResponse>)> {
    let tx_envelope = transactions::create_transaction(payload.transaction_data)
        .map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse { error: e.to_string() })
        ))?;

    let signed_tx = wallet::sign_transaction(&payload.private_key, &tx_envelope)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e.to_string() })
        ))?;

    let formatted_tx = transactions::format_signed_transaction(signed_tx)
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse { error: e.to_string() })
        ))?;

    Ok(Json(routes::SignResponse { signed_transaction: formatted_tx }))
}