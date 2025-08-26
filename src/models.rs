use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct TokenTransferParams {
    pub sender_secret: String,
    pub recipient_address: String,
    pub issuer_address: String,
    pub currency_code: String,
    pub amount: String,
}

#[derive(Debug, Clone)]
pub struct OfflineSigningParams {
    pub sender_secret: String,
    pub recipient_address: String,
    pub issuer_address: String,
    pub currency_code: String,
    pub amount: String,
    pub sequence: u32,
    pub fee: String,
    pub last_ledger_sequence: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct TransactionResult {
    pub hash: String,
    pub success: bool,
}

#[derive(Debug, Clone)]
pub struct VerificationParams {
    pub sender_address: String,
    pub recipient_address: String,
    pub issuer_address: String,
    pub currency_code: String,
    pub amount: String,
    pub tx_hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub method: String,
    pub params: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub result: serde_json::Value,
}