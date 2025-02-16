use ethers::signers::{LocalWallet, MnemonicBuilder};
use serde::Serialize;

#[derive(Serialize)]
pub struct WalletResponse{
    pub address: String,
    pub private_key: String,
    pub mnemonic: String,
}