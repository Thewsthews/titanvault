use axum::{routing::post, Router};
use k256::sha2::digest::Key;
use crate::handlers::{create_wallet, sign_transaction};

//Assuming these are needed for the handlers

use alloy::{
    signers::{local::PrivateKeySigner, Signer},
    primitives::hex,
};

pub fn create_router() -> Router{
    Router::new()
    .route("/wallet", post(create_wallet))
    .route("/sign", post(sign_transaction))
}

pub async fn sign_transaction(
    axum::extract::Json(payload): axum::extract::Json<SignRequest>,
) -> Result<String, axum::http::StatusCode>{
    let signer = PrivateKeySigner::from_bytes(
        &hex::decode(&payload.private_key).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?
    ).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    let signature = signer.sign_message(payload.transaction_data.as_bytes())
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(format!("0x{}", hex::encode(signature.to_vec()))
}

#[derive(serde::Deserialize)]
struct SignRequest{
    private_key: String,
    transaction_data: String,
}