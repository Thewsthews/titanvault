use ethers::{
    signers::{LocalWallet, MnemonicBuilder},
    types::transaction::eip2718::TypedTransaction, utils::hex::hex,
};
use k256::{ecdsa::{signature::Signer as k256Signer, SigningKey, VerifyingKey}, elliptic_curve::rand_core};
use rand_core::OsRng;
use std::{str::FromStr, error::Error};
use serde::Serialize;

#[derive(Serialize)]
pub struct WalletResponse{
    pub address: String,
    pub private_key: String,
    pub mnemonic: String,
}


pub fn generate_wallet() -> WalletResponse{
    //This is reponsible for generation of a mnemonic
    let mnemonic = MnemonicBuilder::default()
    .word_count(12)
    .build()
    .unwrap();

    //Derives a wallet from the mnemonic
    let wallet = mnemonic.clone()
    .build()
    .unwrap();

    WalletResponse{
        address: wallet.address().to_string(),
        private_key: wallet.private_key().unwrap().to_string(),
        mnemonic: mnemonic.phrase(),
    }
}
pub async fn sign_transaction(
    private_key: &str,
    transaction: &TypedTransaction,
)-> Result<String, Box<dyn Error>>{
    let wallet = LocalWallet::from_str(private_key)?;
    let signature = wallet.sign(transaction).await?;
    Ok(signature.to_string())
}

//This generates a new secp256k1 private key using the k256 protocol
pub fn generate_k256_key()->(String, String){
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    (
        hex::encode (signing_key.to_bytes().as_ref::<[u8; 32]>()),
        hex::encode(verifying_key.to_sec1_bytes().as_ref()),
    )
}

///This is the part that signs a message using k256
pub fn sign_message_k256(private_key: &str, message: &[u8]) -> Result<String, Box<dyn Error>>{
    let signing_key = SigningKey::from_bytes(&hex::decode(private_key)?)?;
    let signature = signing_key.sign(message);
    Ok(hex::encode(signature.to_bytes().as_ref()))
}