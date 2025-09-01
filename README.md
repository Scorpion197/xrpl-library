# XRPL Library 
A Rust library for token transfers on the Ripple (XRPL) Testnet
### implemented features
1. **send_token** - function signature and structure implemented
2. **verify_token_sent** - fully functional, queries real XRPL testnet
3. **sign_transfer_offline** - creates formatted transaction JSON
4. **submit_signed_transaction** - accept and submit signed blobs.

after running `cargo run` you should see something like:

```

transaction details: {
  "Account": "rBSUtAi1TSrgGBb43dJFSZ2K6yRUB2ZK1N",
  "Amount": "1000000",
  "Destination": "rpt4EfhXeHeq1Qx4FQc122Xw654XqaUqvH",
  "Fee": "12",
  "Sequence": 10264753,
  "TransactionType": "Payment"
}
processing mock transaction: 780086c3903aeb010001453453fc763c828f0e75b39e56f61a8458a88ceab3e4
Token sent! Hash: 780086c3903aeb010001453453fc763c828f0e75b39e56f61a8458a88ceab3e4, Success: true
Verification completed, transaction found: false
mock transaction created
transaction details: {
  "Account": "rBSUtAi1TSrgGBb43dJFSZ2K6yRUB2ZK1N",
  "Amount": {
    "currency": "TST",
    "issuer": "rD1VV3YcQNhjcigfKNUv734Ks8YNRMNRHD",
    "value": "50"
  },
  "Destination": "rpt4EfhXeHeq1Qx4FQc122Xw654XqaUqvH",
  "Fee": "12",
  "LastLedgerSequence": 99999,
  "Sequence": 1,
  "TransactionType": "Payment"
}
Transaction signed offline: 9bf7b5a89e36fc0bd7ed71ecf60eb82bf90a9ab143eee422e2
processing mock transaction: 9bf7b5a89e36fc0bd7ed71ecf60eb82bf90a9ab143eee422e28855daf987a8f0
Transaction submitted! Hash: 9bf7b5a89e36fc0bd7ed71ecf60eb82bf90a9ab143eee422e28855daf987a8f0, Success: true

```


