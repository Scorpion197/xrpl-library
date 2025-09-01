# XRPL Library 
A Rust library for token transfers on the Ripple (XRPL) Testnet
### implemented features
1. **send_token** - function signature and structure implemented
2. **verify_token_sent** - fully functional, queries real XRPL testnet
3. **sign_transfer_offline** - creates formatted transaction JSON
4. **submit_signed_transaction** - accept and submit signed blobs.
   
Note that the current implementation does not include the cryptographic signing. For crytpographiuc signin we can use secp256k1 ECDSA algorithm 

