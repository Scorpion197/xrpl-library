use crate::{Error, Result};
use secp256k1::{Secp256k1, SecretKey, PublicKey, Message, ecdsa::Signature};
use sha2::{Sha256, Sha512, Digest};
use ripemd::{Ripemd160};
use serde_json::{json, Value};
use std::collections::BTreeMap;

const XRPL_SIGNING_PREFIX: &[u8] = b"STX\x00";

pub struct XrplWallet {
    private_key: SecretKey,
    public_key: PublicKey,
    address: String,
}

impl XrplWallet {
    /// Create wallet from hex-encoded private key (seed)
    pub fn from_secret(secret_hex: &str) -> Result<Self> {
        let secret_bytes = hex::decode(secret_hex)
            .map_err(|e| Error::ValidationError(format!("invalid secret hex: {}", e)))?;
        
        if secret_bytes.len() != 32 {
            return Err(Error::ValidationError("secret must be 32 bytes".to_string()));
        }

        let secp = Secp256k1::new();
        let private_key = SecretKey::from_slice(&secret_bytes)
            .map_err(|e| Error::ValidationError(format!("invalid private key: {}", e)))?;
        
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let address = Self::public_key_to_address(&public_key)?;

        Ok(XrplWallet {
            private_key,
            public_key,
            address,
        })
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    /// Convert public key to XRPL address
    fn public_key_to_address(public_key: &PublicKey) -> Result<String> {
        let pub_key_bytes = public_key.serialize();
        
        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&pub_key_bytes);
        let sha256_result = sha256_hasher.finalize();

        let mut ripemd_hasher = Ripemd160::new();
        ripemd_hasher.update(&sha256_result);
        let ripemd_result = ripemd_hasher.finalize();

        let mut versioned = vec![0x00];
        versioned.extend_from_slice(&ripemd_result);

        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&versioned);
        let first_hash = sha256_hasher.finalize();

        let mut sha256_hasher = Sha256::new();
        sha256_hasher.update(&first_hash);
        let second_hash = sha256_hasher.finalize();

        versioned.extend_from_slice(&second_hash[0..4]);

        Ok(bs58::encode(&versioned).with_alphabet(bs58::Alphabet::RIPPLE).into_string())
    }

    pub async fn sign_transaction(&self, tx_json: &Value, client: &crate::client::XrplClient, account_address: &str) -> Result<String> {
        let mut tx = tx_json.clone();
        
        if let Value::Object(ref mut map) = tx {
            map.insert("Account".to_string(), json!(account_address));
            
            if !map.contains_key("Sequence") {
                let account_info = client.get_account_info(account_address).await?;
                if let Some(sequence) = account_info["account_data"]["Sequence"].as_u64() {
                    map.insert("Sequence".to_string(), json!(sequence));
                }
            }
        }

        let canonical_json = self.canonicalize_json(&tx)?;
        
        let signing_hash = self.create_signing_hash(&canonical_json)?;
        
        let secp = Secp256k1::new();
        let message = Message::from_digest_slice(&signing_hash)
            .map_err(|e| Error::ValidationError(format!("invalid message: {}", e)))?;
        
        let signature = secp.sign_ecdsa(&message, &self.private_key);
        
        self.create_signed_blob(&tx, &signature)
    }

    fn canonicalize_json(&self, tx: &Value) -> Result<String> {
        fn sort_json(value: &Value) -> Value {
            match value {
                Value::Object(map) => {
                    let sorted: BTreeMap<String, Value> = map
                        .iter()
                        .map(|(k, v)| (k.clone(), sort_json(v)))
                        .collect();
                    Value::Object(serde_json::Map::from_iter(sorted))
                }
                Value::Array(arr) => {
                    Value::Array(arr.iter().map(sort_json).collect())
                }
                _ => value.clone(),
            }
        }

        let sorted_tx = sort_json(tx);
        serde_json::to_string(&sorted_tx)
            .map_err(|e| Error::SerializationError(format!("failed to serialize transaction: {}", e)))
    }

    /// Create signing hash for transaction
    fn create_signing_hash(&self, canonical_json: &str) -> Result<[u8; 32]> {
        let mut hasher = Sha512::new();
        hasher.update(XRPL_SIGNING_PREFIX);
        hasher.update(canonical_json.as_bytes());
        let hash = hasher.finalize();
        
        let mut result = [0u8; 32];
        result.copy_from_slice(&hash[0..32]);
        Ok(result)
    }

    fn create_signed_blob(&self, tx: &Value, signature: &Signature) -> Result<String> {
        
        let mock_tx_data = format!("{:?}{}", tx, hex::encode(signature.serialize_compact()));
        let hash = Sha256::digest(mock_tx_data.as_bytes());
        let mock_blob = hex::encode(hash);
        
        println!("mock transaction created");
        println!("transaction details: {}", serde_json::to_string_pretty(tx)?);
        
        Ok(mock_blob)
    }
}

pub fn seed_to_private_key(seed: &str) -> Result<String> {
    if seed.len() == 64 {
        Ok(seed.to_string())
    } else if seed.starts_with('s') {
        match bs58::decode(seed).with_alphabet(bs58::Alphabet::RIPPLE).into_vec() {
            Ok(decoded) => {
                if decoded.len() >= 21 {
                    let entropy = &decoded[1..17]; 
                    let mut hasher = Sha512::new();
                    hasher.update(entropy);
                    hasher.update(&[0u8, 0u8, 0u8, 0u8]); // sequence number 0
                    let result = hasher.finalize();
                    let private_key = &result[0..32];
                    Ok(hex::encode(private_key))
                } else {
                    Err(Error::ValidationError("nvalid seed length after decoding".to_string()))
                }
            }
            Err(_) => Err(Error::ValidationError("invalid base58 seed format".to_string()))
        }
    } else {
        hex::decode(seed)
            .map_err(|_| Error::ValidationError("nvalid seed format - must be hex or XRPL base58 seed".to_string()))
            .and_then(|bytes| {
                if bytes.len() == 32 {
                    Ok(hex::encode(bytes))
                } else {
                    Err(Error::ValidationError("hex seed must be 32 bytes".to_string()))
                }
            })
    }
}
