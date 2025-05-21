use alloy_core::tx::{TxEnvelope, U256};
use alloy_primitives::{Address, hex};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Deserialize, Serialize)]
pub struct TransactionRequest {
    pub to: String,          
    pub value: String,       
    pub gas_limit: u64,      
    pub gas_price: String,   
    pub nonce: u64,          
    pub data: Option<String>, 
    pub chain_id: Option<u64>,
}


#[derive(Serialize)]
pub struct SignedTransaction {
    pub raw: String,         
    pub hash: String,        
}


pub fn create_transaction(request: TransactionRequest) -> Result<TxEnvelope, Box<dyn Error>> {
    let to = Address::parse_checksummed(&request.to, None)
        .map_err(|e| format!("Invalid 'to' address: {}", e))?;
    
    let value = U256::from_str_radix(&request.value, 10)
        .map_err(|e| format!("Invalid value: {}", e))?;
    
    let gas_price = U256::from_str_radix(&request.gas_price, 10)
        .map_err(|e| format!("This is an invalidly declared gas price: {}", e))?;
    
    let data = request.data
        .map(|d| hex::decode(d.trim_start_matches("0x")).map_err(|e| format!("Invalid data: {}", e)))
        .transpose()?
        .unwrap_or_default();

    
    let tx = alloy_core::primitives::TxLegacy {
        to: alloy_core::primitives::TxKind::Call(to),
        value,
        gas_limit: request.gas_limit.into(),
        gas_price,
        nonce: request.nonce.into(),
        data: data.into(),
        chain_id: Some(1),
    };
    let tx = alloy_core::primitives::TxLegacy{
        to: alloy_core::primitives::TxKind::Call(to),
        value,
        gas_limit: request.gas_limit.into(),
        gas_price,
        nonce: request.nonce.into(),
        data: data.into(),
        chain_id: Some(request.chain_id.unwrap_or(1)), // This line ensures that it defaults to mainnet if not provided and at all if there is any issue with the set id provided.
    }

    Ok(TxEnvelope::Legacy(tx))
}


pub fn format_signed_transaction(tx: TxEnvelope) -> Result<SignedTransaction, Box<dyn Error>> {
    let encoded = alloy_core::rlp::encode(&tx);
    let hash = alloy_primitives::keccak256(&encoded);
    
    Ok(SignedTransaction {
        raw: format!("0x{}", hex::encode(encoded)),
        hash: format!("0x{}", hex::encode(hash)),
    })
}