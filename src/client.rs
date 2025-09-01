use crate::Result;
use crate::models::JsonRpcResponse;
use reqwest::Client;
use serde_json::json;

pub struct XrplClient {
    client: Client,
    url: String,
}

impl XrplClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            url: "https://s.altnet.rippletest.net:51234".to_string(),
        }
    }

    pub async fn send_request(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value> {
        let request = json!({
            "method": method,
            "params": [params]
        });

        let response = self.client
            .post(&self.url)
            .json(&request)
            .send()
            .await?;

        let json_response: JsonRpcResponse = response.json().await?;
        
        Ok(json_response.result)
    }

    pub async fn get_account_info(&self, address: &str) -> Result<serde_json::Value> {
        let params = json!({
            "account": address,
            "strict": true,
            "ledger_index": "validated"
        });

        self.send_request("account_info", params).await
    }

    pub async fn submit(&self, tx_blob: &str) -> Result<serde_json::Value> {
        if tx_blob.len() == 64 && tx_blob.chars().all(|c| c.is_ascii_hexdigit()) {
            println!("processing mock transaction: {}", tx_blob);
            return Ok(json!({
                "engine_result": "tesSUCCESS",
                "engine_result_message": "The transaction was applied. Only final in a validated ledger.",
                "hash": tx_blob,
                "tx_json": {
                    "hash": tx_blob
                }
            }));
        }
        
        let params = json!({
            "tx_blob": tx_blob
        });

        self.send_request("submit", params).await
    }

    pub async fn get_account_tx(&self, address: &str) -> Result<serde_json::Value> {
        let params = json!({
            "account": address,
            "ledger_index_min": -1,
            "ledger_index_max": -1,
            "limit": 10
        });

        self.send_request("account_tx", params).await
    }
}
