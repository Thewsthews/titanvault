use ethers::{signers::{LocalWallet, MnemonicBuilder, Signer}, types::transaction::eip2718::TypedTransaction};
use std::str::FromStr;
use serde::Serialize;

#[derive(Serialize)]
pub struct WalletResponse {
    pub address: String,
    pub private_key: String,
    pub mnemonic: String,
}

pub fn generate_wallet() -> WalletResponse {
    // Generate a mnemonic
    let mnemonic = MnemonicBuilder::default()
        .word_count(12)
        .build()
        .unwrap();

    // Derive a wallet from the mnemonic
    let wallet = mnemonic.clone()
        .generate()
        .unwrap();

    WalletResponse {
        address: wallet.address().to_string(),
        private_key: wallet.signer().to_string(),
        mnemonic: mnemonic.to_string(),
    }
}

pub async fn sign_transaction(
    private_key: &str,
    transaction: &TypedTransaction,
) -> Result<String, Box<dyn std::error::Error>> {
    let wallet = LocalWallet::from_str(private_key)?;
    let signed_tx = wallet.sign_transaction(transaction).await?;
    Ok(format!("{:?}", signed_tx))
}
