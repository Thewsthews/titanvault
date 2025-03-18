use alloy::{
    signers::{local::PrivateKeySigner, Signer},
    primitives::hex,
};

pub async fn sign_transaction(private_key: String, transaction_data: String)-> String{
     //This part creates a signer from teh private key
     let signer = PrivateKeySigner::from_bytes(
        &hex::decode(&private_key).expect("Invalid private key")
    ).expect("Failed to create signer");
    //This part signs the transaction
    let signature = signer.sign_message(transaction_data.as_bytes())
    .await
    .expect("Failed to sign transaction");

    //Formats the signature as a hex string with 0x prefix
    format!("0x{}", hex::encode(signature.to_vec()))
     
}