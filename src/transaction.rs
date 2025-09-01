use crate::Result;
use crate::models::{TokenTransferParams, OfflineSigningParams, TransactionResult, VerificationParams};
use crate::client::XrplClient;
use crate::wallet::{XrplWallet, seed_to_private_key};
use serde_json::json;

pub async fn send_token(params: TokenTransferParams) -> Result<TransactionResult> {
    let client = XrplClient::new();
    
    let private_key = seed_to_private_key(&params.sender_secret)?;
    
    let wallet = XrplWallet::from_secret(&private_key)?;
    
    
    let tx_json = if params.currency_code == "XRP" {
        json!({
            "TransactionType": "Payment",
            "Destination": params.recipient_address,
            "Amount": params.amount, 
            "Fee": "12" 
        })
    } else {
        json!({
            "TransactionType": "Payment",
            "Destination": params.recipient_address,
            "Amount": {
                "currency": params.currency_code,
                "issuer": params.issuer_address,
                "value": params.amount
            },
            "Fee": "12" 
        })
    };
    
    let signed_blob = wallet.sign_transaction(&tx_json, &client, &params.sender_address).await?;
    
    let result = client.submit(&signed_blob).await?;
    
    let success = result["engine_result"].as_str() == Some("tesSUCCESS");
    let hash = result["tx_json"]["hash"]
        .as_str()
        .or_else(|| result["hash"].as_str())
        .unwrap_or("unknown")
        .to_string();
    
    Ok(TransactionResult { hash, success })
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

pub async fn sign_transfer_offline(params: OfflineSigningParams) -> Result<String> {
    let private_key = seed_to_private_key(&params.sender_secret)?;
    
    let wallet = XrplWallet::from_secret(&private_key)?;
    
    let tx_json = json!({
        "TransactionType": "Payment",
        "Account": params.sender_address,
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
    
    let client = XrplClient::new();
    wallet.sign_transaction(&tx_json, &client, &params.sender_address).await
}

pub async fn submit_signed_transaction(signed_tx_blob: &str) -> Result<TransactionResult> {
    let client = XrplClient::new();
    
    let result = client.submit(signed_tx_blob).await?;
    
    let success = result["engine_result"].as_str() == Some("tesSUCCESS");
    let hash = result["tx_json"]["hash"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    
    Ok(TransactionResult { hash, success })
}