use axum::Json;
use serde::{Deserialize, Serialize};
// use crate::wallet::Wallet; // Commented out as it is not used
use ethers::types::{transaction::eip2718::TypedTransaction, Transaction, TransactionRequest};

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
    let transaction: Transaction = serde_json::from_str(&payload.transaction_data).expect("Invalid transaction data");
    let transaction_request: TransactionRequest = transaction.into();
    let typed_transaction: TypedTransaction = TypedTransaction::Legacy(transaction_request);
    let signed_transaction = wallet::sign_transaction(&payload.private_key, &typed_transaction).await.expect("Failed to sign transaction");
    Json(SignResponse{signed_transaction})
}