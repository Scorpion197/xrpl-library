use xrpl_library::{
    Result,
    TokenTransferParams,
    OfflineSigningParams, 
    VerificationParams,
    send_token,
    verify_token_sent,
    sign_transfer_offline,
    submit_signed_transaction,
};

#[tokio::main]
async fn main() -> Result<()> {
        
    // for these tests, we can get addresses and secrets from testnet faucet
    // link you can finde it here https://xrpl.org/testnet-faucet.html
    let user1_secret = "sEdV25jsgHp6XR122gZs9Xe4ncc7U4xJ";  
    let user1_address = "rBSUtAi1TSrgGBb43dJFSZ2K6yRUB2ZK1N";
    let user2_address = "rpt4EfhXeHeq1Qx4FQc122Xw654XqaUqvH";  
    let issuer_address = "rD1VV3YcQNhjcigfKNUv734Ks8YNRMNRHD"; 
    
    if !user1_secret.is_empty() {
        let send_params = TokenTransferParams {
            sender_secret: user1_secret.to_string(),
            sender_address: user1_address.to_string(),
            recipient_address: user2_address.to_string(),
            issuer_address: "".to_string(), 
            currency_code: "XRP".to_string(),
            amount: "1000000".to_string(), 
        };
        
        match send_token(send_params).await {
            Ok(result) => println!("Token sent! Hash: {}, Success: {}", result.hash, result.success),
            Err(e) => println!("Error sending token: {}", e),
        }
    }
    
    let verify_params = VerificationParams {
        sender_address: user1_address.to_string(),
        recipient_address: user2_address.to_string(),
        issuer_address: issuer_address.to_string(),
        currency_code: "TST".to_string(),
        amount: "100".to_string(),
        tx_hash: None,
    };
    
    match verify_token_sent(verify_params).await {
        Ok(found) => println!("Verification completed, transaction found: {}", found),
        Err(e) => println!("Error occurred while verifying: {}", e),
    }
    
    let offline_params = OfflineSigningParams {
        sender_secret: user1_secret.to_string(),
        sender_address: user1_address.to_string(),
        recipient_address: user2_address.to_string(),
        issuer_address: issuer_address.to_string(),
        currency_code: "TST".to_string(),
        amount: "50".to_string(),
        sequence: 1,
        fee: "12".to_string(),
        last_ledger_sequence: Some(99999),
    };
    
    if !user1_secret.is_empty() {
        match sign_transfer_offline(offline_params).await {
            Ok(blob) => {
                println!("Transaction signed offline: {}", &blob[..50.min(blob.len())]);
                
                match submit_signed_transaction(&blob).await {
                    Ok(result) => println!("Transaction submitted! Hash: {}, Success: {}", result.hash, result.success),
                    Err(e) => println!("Error submitting transaction: {}", e),
                }
            },
            Err(e) => println!("Error occurred while signing: {}", e),
        }
    }
    
    Ok(())
}