use ethers::signers::LocalWallet;
use ethers::core::types::TransactionRequest;

pub async fn sign_transaction(private_key: String, transaction_data: String) -> String{
    let wallet: LocalWallet = pricate_key.parse().unwrap();
    let signed_tx = wallet.sign_message(transaction_data).parse().unwrap();
    format!("0x{:x}", signed_tx)
}