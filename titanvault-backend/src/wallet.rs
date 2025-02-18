use ethers::{signers::{LocalWallet, MnemonicBuilder, Signer}, types::Transaction};
use std::str::FromStr;
use serde::Serialize;


#[derive(Serialize)]
pub struct WalletResponse{
    pub address: String,
    pub private_key: String,
    pub mnemonic: String,
}

pub fn generate_wallet() -> WalletResponse{
    let mnemonic = MnemonicBuilder::default().word_count(12).build().unwrap();
    let wallet = MnemonicBuilder::default().build().unwrap();

    WalletResponse{
        address: wallet.address().to_string(),
        private_key: format!("{:?}", wallet),
        mnemonic: format!("{:?}", mnemonic),
    }
}


pub async fn sign_transaction(private_key: &str, transaction: &Transaction) -> Result<String, Box<dyn std::error::Error>> {
    let wallet = LocalWallet::from_str(private_key)?;
    let typed_tx: Transaction = transaction.clone().into();
    let signed_tx = wallet.sign_transaction(&typed_tx).await?;
    Ok(signed_tx.to_string())
}