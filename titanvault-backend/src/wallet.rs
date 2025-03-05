use alloy::consensus::transaction;
use alloy_primitives::{hex, Address, B256};
use alloy_signer::{LocalWallet, Signer as AlloySigner};
use alloy_signer_wallet::{MnemonicBuilder, Wallet};
use ethers::signers::coins_bip39::{mnemonic, Mnemonic};
use k256::{
    ecdsa::{signature::Signer as k256Signer, SigningKey, VerifyingKey},
    elliptic_curve::{generic_array::GenericArray,rand_core},
};
use rand_core::OsRng;
use serde::Serialize;
use std::{error::Error, str::FromStr};

#[derive(Serialize)]
pub struct WalletResponse {
    address: String,
    private_key: String,
    mnemonic: String,
}

pub fn generate_wallet() -> WalletResponse{
    
    //This part is responsible for the generation of a mnemonic and also derives a wallet
    let mnemonic = MnemonicBuilder::default()
    .word_count(12)
    .build()
    .expect("Failed to generate mnemonic");

    //Derives wallet from mnemonic
    let wallet: LocalWallet = mnemonic.into();

    WalletResponse {
        address: format!("{:?}", wallet.address()), //This address is an alloy_primitives::Address as used above
        private_key: hex::encode(wallet.private_key().to_bytes()),
        mnemonic: wallet.mnemonic().unwrap().phrase().to_string(),
    }   
}

pub async fn sign_transaction(
    private_key: &str,
    transaction: &alloy_core::primitives::TxEnvelope,
) -> Result<String, Box<dyn Error>>{
    //Parsing of the private key into a localwallet
let wallet = LocalWallet::from_str(private_key)?;
//Signing the transaction
let signature = wallet.sign_transaction(transaction).await?;
Ok(format!("{:?}", signature)) //This is responsible for the convertion of the signature to a String
}

//This is followed by the generation of a new secp265k1 key pair using k256
pub fn generate_k256_key() -> (String, String){
    let signing_key = SigningKey::random(&mut OsRng);
    let verifying_key = VerifyingKey::from(&signing_key);
    (
        hex::encode(verifying_key.to_sec1_bytes()), // Public key
        hex::encode(signing_key.to_bytes()), // Private key
    )
}

//This is the part that handles the signing of a message using k256
pub fn sign_message_k256(private_key: &str, message: &[u8]) -> Result<String, Box<dyn Error>>{
    let decoded_key = hex::decode(private_key)?;
    let signing_key = SigningKey::from_bytes(&GenericArray::clone_from_slice(&decoded_key))?;
    let signature = k256::ecdsa::Signature=signing_key.sign(message);
    Ok(hex::encode(signature.to_bytes()))
}