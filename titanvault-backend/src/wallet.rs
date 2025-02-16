use ethers::signers::{LocalWallet, MnemonicBuilder};
use serde::Serialize;

#[derive(Serialize)]
pub struct WalletResponse{
    pub address: String,
    pub private_key: String,
    pub mnemonic: String,
}

pub fn generate_wallet() -> WalletResponse{
    let mnemonic = MnemonicBuilder::default().word_count(12).generate().unwrap();
    let wallet = mnemonic.clone().into_wallet().unwrap();

    WalletResponse{
        address: wallet.address().to_string(),
        private_key: wallet.private_key().unwrap().to_string(),
        mnemonic: mnemonic.phrase(),
    }
}