use xrpl_library::{
    Result,
    OfflineSigningParams, 
    VerificationParams,
    verify_token_sent,
    sign_transfer_offline,
};

#[tokio::main]
async fn main() -> Result<()> {
        
    // for these tests, we can get addresses and secrets from testnet faucet
    // link you can finde it here https://xrpl.org/testnet-faucet.html
    let user1_secret = "";
    let user1_address = "";
    let user2_address = "";
    let issuer_address = "";
    
    let verify_params = VerificationParams {
        sender_address: user1_address.to_string(),
        recipient_address: user2_address.to_string(),
        issuer_address: issuer_address.to_string(),
        currency_code: "TST".to_string(),
        amount: "100".to_string(),
        tx_hash: None,
    };
    
    match verify_token_sent(verify_params).await {
        Ok(found) => println!("verification completed transactio nfound: {}", found),
        Err(e) => println!("error occured while verifying: {}", e),
    }
    
    let offline_params = OfflineSigningParams {
        sender_secret: user1_secret.to_string(),
        recipient_address: user2_address.to_string(),
        issuer_address: issuer_address.to_string(),
        currency_code: "TST".to_string(),
        amount: "50".to_string(),
        sequence: 1,
        fee: "12".to_string(),
        last_ledger_sequence: Some(99999),
    };
    
    match sign_transfer_offline(offline_params) {
        Ok(blob) => println!("transaction has been created: {}", &blob[..50.min(blob.len())]),
        Err(e) => println!("error occured while signing: {}", e),
    }
    
    Ok(())
}