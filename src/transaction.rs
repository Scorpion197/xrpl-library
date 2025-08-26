use crate::{Error, Result};
use crate::models::{TokenTransferParams, OfflineSigningParams, TransactionResult, VerificationParams};
use crate::client::XrplClient;
use serde_json::json;

pub async fn send_token(_params: TokenTransferParams) -> Result<TransactionResult> {
    let _client = XrplClient::new();
    
    // for prod better use xrpl-rust for real signing
    Err(Error::ValidationError(
        "error occured while sending token".to_string()
    ))
}

pub async fn verify_token_sent(params: VerificationParams) -> Result<bool> {
    let client = XrplClient::new();
    
    let result = client.get_account_tx(&params.sender_address).await?;
    
    if let Some(transactions) = result["transactions"].as_array() {
        for tx in transactions {
            let tx_obj = &tx["tx"];
            
            if tx_obj["TransactionType"] != "Payment" {
                continue;
            }
            
            if tx_obj["Destination"] != params.recipient_address {
                continue;
            }
            
            if let Some(amount) = tx_obj["Amount"].as_object() {
                if amount["currency"] == params.currency_code &&
                   amount["issuer"] == params.issuer_address &&
                   amount["value"] == params.amount {
                    
                    if let Some(expected_hash) = &params.tx_hash {
                        if let Some(actual_hash) = tx_obj["hash"].as_str() {
                            return Ok(actual_hash == expected_hash);
                        }
                    }
                    return Ok(true);
                }
            }
        }
    }
    
    Ok(false)
}

pub fn sign_transfer_offline(params: OfflineSigningParams) -> Result<String> {
    let tx_json = json!({
        "TransactionType": "Payment",
        "Account": "rADDRESS_PLACEHOLDER", 
        "Destination": params.recipient_address,
        "Amount": {
            "currency": params.currency_code,
            "issuer": params.issuer_address,
            "value": params.amount
        },
        "Sequence": params.sequence,
        "Fee": params.fee,
        "LastLedgerSequence": params.last_ledger_sequence
    });
    
    Ok(tx_json.to_string())
}

pub async fn submit_signed_transaction(signed_tx_blob: &str) -> Result<TransactionResult> {
    let _client = XrplClient::new();
    
    let _parsed: serde_json::Value = serde_json::from_str(signed_tx_blob)?;
    
    Ok(TransactionResult { 
        hash: "MOCK_TRANSACTION_HASH".to_string(), 
        success: false 
    })
}