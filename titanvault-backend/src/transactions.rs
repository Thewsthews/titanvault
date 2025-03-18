use alloy_core::tx::{TxEnvelope, U256};
use alloy_primitives::{Address, hex};
use serde::{Deserialize, Serialize};
use std::error::Error;

// Structure for creating a basic transaction request
#[derive(Deserialize, Serialize)]
pub struct TransactionRequest {
    pub to: String,          // Destination address as hex string
    pub value: String,       // Value in wei as decimal string
    pub gas_limit: u64,      // Gas limit
    pub gas_price: String,   // Gas price in wei as decimal string
    pub nonce: u64,          // Transaction nonce
    pub data: Option<String>, // Optional transaction data as hex string
}

// Structure for a signed transaction response
#[derive(Serialize)]
pub struct SignedTransaction {
    pub raw: String,         // Raw transaction bytes as hex
    pub hash: String,        // Transaction hash as hex
}

// Creates a TxEnvelope from a TransactionRequest
pub fn create_transaction(request: TransactionRequest) -> Result<TxEnvelope, Box<dyn Error>> {
    let to = Address::parse_checksummed(&request.to, None)
        .map_err(|e| format!("Invalid 'to' address: {}", e))?;
    
    let value = U256::from_str_radix(&request.value, 10)
        .map_err(|e| format!("Invalid value: {}", e))?;
    
    let gas_price = U256::from_str_radix(&request.gas_price, 10)
        .map_err(|e| format!("Invalid gas price: {}", e))?;
    
    let data = request.data
        .map(|d| hex::decode(d.trim_start_matches("0x")).map_err(|e| format!("Invalid data: {}", e)))
        .transpose()?
        .unwrap_or_default();

    // Creating a legacy transaction for simplicity
    let tx = alloy_core::primitives::TxLegacy {
        to: alloy_core::primitives::TxKind::Call(to),
        value,
        gas_limit: request.gas_limit.into(),
        gas_price,
        nonce: request.nonce.into(),
        data: data.into(),
        chain_id: Some(1), // Assuming mainnet, could be configurable
    };

    Ok(TxEnvelope::Legacy(tx))
}

// Formats a signed transaction envelope into a response
pub fn format_signed_transaction(tx: TxEnvelope) -> Result<SignedTransaction, Box<dyn Error>> {
    let encoded = alloy_core::rlp::encode(&tx);
    let hash = alloy_primitives::keccak256(&encoded);
    
    Ok(SignedTransaction {
        raw: format!("0x{}", hex::encode(encoded)),
        hash: format!("0x{}", hex::encode(hash)),
    })
}