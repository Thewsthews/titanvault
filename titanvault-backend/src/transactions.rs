use ethers::{signers::{LocalWallet, Signer}, utils::hex};

pub async fn sign_transaction(private_key: String, transaction_data: String) -> String{
    let wallet: LocalWallet = private_key.parse().unwrap();
    let signed_tx = wallet.sign_message(transaction_data).await.unwrap();
    format!("0x{}", hex::encode(signed_tx.to_vec()))
}