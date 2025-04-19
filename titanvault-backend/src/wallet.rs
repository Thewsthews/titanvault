use alloy_primitives::hex;
use alloy_signer::LocalWallet;
use alloy_signer_wallet::MnemonicBuilder;
use alloy_primitives::TxEnvelope;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
pub struct WalletResponse {
    pub address: String,
    pub private_key: String,
    pub mnemonic: String,
}

pub fn generate_wallet() -> WalletResponse {
    let mnemonic = MnemonicBuilder::default()
        .word_count(12)
        .build()
        .expect("Failed to generate mnemonic");

    let wallet: LocalWallet = mnemonic.into();
    
    WalletResponse {
        address: format!("0x{}", wallet.address()),
        private_key: hex::encode(wallet.signer().to_bytes()),
        mnemonic: wallet.mnemonic().unwrap().phrase().to_string(),
    }
}


pub async fn sign_transaction(
    private_key: &str,
    transaction: &TxEnvelope,
) -> Result<TxEnvelope, Box<dyn Error>> {
    let private_key = private_key.trim_start_matches("0x");
    if private_key.len() != 64 || !private_key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err("Invalid private key format: must be 32-byte hex string".into());
    }

    let wallet = LocalWallet::from_str(&format!("0x{}", private_key))?;
    let signature = wallet.sign_transaction(transaction).await?;
    Ok(signature) 
}