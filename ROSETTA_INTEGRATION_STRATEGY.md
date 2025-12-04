# Antimony Rosetta Integration Strategy

**Date:** December 4, 2025  
**Status:** Planning Phase (Phase 2b)  
**Objective:** Build Rosetta API as first-class citizen for instant exchange integration

---

## Overview

Based on Coinbase's [Rosetta initiative](https://www.coinbase.com/en-gb/blog/introducing-rosetta-build-once-integrate-your-blockchain-everywhere), **Antimony will implement Rosetta v1.4.13 specification from the foundation**.

This is not an afterthought - it's a core architectural decision that enables:
- ✅ One-time implementation = instant integration with all Rosetta-compatible exchanges
- ✅ Standardized, secure blockchain interaction pattern
- ✅ Reduced development time for institutional adoption
- ✅ Cross-blockchain developer tools compatibility
- ✅ Professional exchange infrastructure compliance

---

## Rosetta Philosophy

### Why Rosetta From Day One?

**Problem Rosetta Solves:**
- Before Rosetta: Each exchange writes custom integrations for every blockchain
  - Bitcoin: Custom parser + custom wallet logic
  - Ethereum: Custom parser + custom wallet logic
  - Ripple: Custom parser + custom wallet logic
  - etc.

- With Rosetta: Exchanges use standard API for all chains
  - Any chain with Rosetta: Use standard Rosetta client
  - No custom parsing = less bugs
  - No duplicate work across exchanges

**For Antimony:**
- Build Rosetta API once
- Integrate with Coinbase, Kraken, Gemini, OKX, Binance (all support Rosetta)
- Build once, integrate everywhere

---

## Rosetta v1.4.13 Specification

### Core Concept

Rosetta provides a **standardized REST API** for interacting with blockchains:

```
Client (Exchange/Wallet/Explorer)
        ↓
    Rosetta API
   (Standard endpoints)
        ↓
   ATMN Rosetta Server
   (Our implementation)
        ↓
   atmn-core (Consensus)
   (SHA-256d PoW)
```

### Data Model

#### Network Identifier
```json
{
  "blockchain": "Antimony",
  "network": "mainnet",
  "sub_network_identifier": null
}
```

#### Account
```json
{
  "address": "ATM1...abc123",
  "sub_account": null,
  "metadata": {}
}
```

#### Amount
```json
{
  "value": "1000000",           // In satoshis (1 ATMN = 100,000,000 satoshis)
  "currency": {
    "symbol": "ATMN",
    "decimals": 8
  }
}
```

#### Operation (Transaction component)
```json
{
  "operation_identifier": { "index": 0 },
  "type": "TRANSFER",           // or MINT, BURN, FEE, etc.
  "status": "SUCCESS",          // or PENDING, FAILED
  "account": { "address": "ATM1...from" },
  "amount": { "value": "-1000000", "currency": { "symbol": "ATMN", "decimals": 8 } },
  "metadata": {}
}
```

#### Block
```json
{
  "block_identifier": {
    "index": 123456,
    "hash": "0x1234...abcd"
  },
  "parent_block_identifier": {
    "index": 123455,
    "hash": "0x5678...efgh"
  },
  "timestamp": 1701657600000,
  "transactions": [ /* array of transactions */ ],
  "metadata": {
    "difficulty": "1234567890",
    "miner": "ATM1...miner",
    "nonce": 42
  }
}
```

---

## Rosetta API Endpoints (What We Must Implement)

### Network Endpoints

#### `/network/list` - Available networks
```
GET /network/list
Returns: [{ "blockchain": "Antimony", "network": "mainnet" }]
```

#### `/network/options` - Supported operations
```
GET /network/options
Returns: Supported operations, versions, errors
```

#### `/network/status` - Current network state
```
GET /network/status
Returns: Current block height, timestamp, genesis block hash
```

### Block Endpoints

#### `/block` - Get block by height/hash
```
POST /block
Body: { "network_identifier": {...}, "block_identifier": {...} }
Returns: Full block with all transactions
```

#### `/block/transaction` - Get specific transaction in block
```
POST /block/transaction
Body: { "network_identifier": {...}, "block_identifier": {...}, "transaction_identifier": {...} }
Returns: Transaction details with operations
```

### Mempool Endpoints

#### `/mempool` - Get pending transactions
```
POST /mempool
Body: { "network_identifier": {...} }
Returns: List of pending transaction hashes
```

#### `/mempool/transaction` - Get pending transaction details
```
POST /mempool/transaction
Body: { "network_identifier": {...}, "transaction_identifier": {...} }
Returns: Transaction data (from mempool)
```

### Account Endpoints

#### `/account/balance` - Get account balance
```
POST /account/balance
Body: { "network_identifier": {...}, "account_identifier": {...}, "block_identifier": {...} }
Returns: Current balance + coins (UTXO references)
```

#### `/account/coins` - Get spendable coins (UTXO model)
```
POST /account/coins
Body: { "network_identifier": {...}, "account_identifier": {...} }
Returns: List of { "coin_identifier", "amount" } objects
```

### Construction Endpoints

#### `/construction/preprocess` - Initial validation
```
POST /construction/preprocess
Body: Unsigned transaction
Returns: Metadata needed for construction
```

#### `/construction/metadata` - Get dynamic metadata
```
POST /construction/metadata
Body: Network + account info
Returns: Fee info, nonce, other metadata
```

#### `/construction/payloads` - Create payloads to sign
```
POST /construction/payloads
Body: Preprocessed transaction
Returns: { "unsigned_transaction", "payloads": [...] }
```

#### `/construction/parse` - Parse transaction
```
POST /construction/parse
Body: Signed or unsigned transaction
Returns: Human-readable operations
```

#### `/construction/combine` - Combine unsigned + signatures
```
POST /construction/combine
Body: Unsigned transaction + signatures
Returns: Signed transaction (ready to broadcast)
```

#### `/construction/hash` - Get transaction hash
```
POST /construction/hash
Body: Signed transaction
Returns: Transaction hash
```

#### `/construction/submit` - Broadcast transaction
```
POST /construction/submit
Body: Signed transaction
Returns: Transaction hash + status
```

---

## ATMN Rosetta Architecture

### Layer Structure

```
Layer 1: HTTP Server (Actix-web or Axum)
         ↓
Layer 2: Rosetta Handlers (request/response)
         ↓
Layer 3: Rosetta Data Conversion
         (ATMN types ↔ Rosetta JSON)
         ↓
Layer 4: ATMN Core
         (Consensus, Storage, Transactions)
```

### Implementation Plan (Phase 2b-2c)

#### Step 1: HTTP Server Setup (1-2 hours)
- Create `atmn-rosetta/` module
- Setup Actix-web or Axum
- CORS configuration
- Error handling middleware

#### Step 2: Data Type Converters (2-3 hours)
```rust
impl From<Block> for RosettaBlock
impl From<Transaction> for RosettaTransaction  
impl From<Amount> for RosettaAmount
impl From<Address> for RosettaAccount
```

#### Step 3: Network Endpoints (2-3 hours)
- `/network/list`
- `/network/options`
- `/network/status`
- All return hardcoded but correct data

#### Step 4: Block Endpoints (3-4 hours)
- `/block`
- `/block/transaction`
- Query atmn-core for block/tx data
- Return Rosetta-formatted response

#### Step 5: Account Endpoints (3-4 hours)
- `/account/balance`
- `/account/coins`
- Query UTXO set from storage
- Calculate balances

#### Step 6: Construction Endpoints (4-5 hours)
- `/construction/preprocess`
- `/construction/metadata`
- `/construction/payloads`
- `/construction/combine`
- `/construction/submit`
- Full transaction lifecycle

#### Step 7: Testing & Validation (2-3 hours)
- Rosetta validator CLI
- Integration tests
- Exchange simulator tests

### Estimated Timeline
- **Phase 2b (1-2 weeks):** HTTP server + data converters
- **Phase 2c (2-3 weeks):** Block + account endpoints
- **Phase 3 (2-3 weeks):** Construction endpoints + testing

---

## How Rosetta Integrates with ATMN Architecture

### With Consensus (Already Built ✅)

```
Rosetta: "Get block 12345"
    ↓
Converts to: "Get block at height 12345"
    ↓
atmn-core::consensus::get_block(12345)
    ↓
Returns: Block { hash, transactions, difficulty, ... }
    ↓
Rosetta converts to: RosettaBlock { ... }
    ↓
Returns JSON to exchange
```

### With Storage (Phase 3)

```
Rosetta: "Get balance of ATM1...xyz"
    ↓
Rosetta queries: storage.get_utxo_for_account(address)
    ↓
Storage returns: Vec<Coin> with amounts
    ↓
Rosetta sums: total_balance
    ↓
Returns JSON to exchange
```

### With Mining (Phase 3)

```
Rosetta: "Broadcast transaction"
    ↓
Validates via: consensus.verify_transaction()
    ↓
Adds to: mempool
    ↓
Mining picks up from mempool
    ↓
Returns: transaction_id to exchange
```

---

## Exchange Integration Flow

### For Coinbase (or any exchange using Rosetta)

```
1. Coinbase sets up Rosetta client
   ↓
2. Points to: https://rpc.antimony.carphatian.ro/rosetta
   ↓
3. Calls: /network/list
   ↓
4. Gets: Antimony mainnet info
   ↓
5. Calls: /network/options
   ↓
6. Gets: Supported operations (TRANSFER, FEE, etc.)
   ↓
7. User deposits ATM coins
   ↓
8. Coinbase calls: /construction/preprocess + /construction/payloads
   ↓
9. Creates unsigned transaction
   ↓
10. Signs transaction
    ↓
11. Calls: /construction/combine + /construction/submit
    ↓
12. Transaction broadcasts to ATMN network
    ↓
13. Coinbase calls: /block to confirm
    ↓
14. User's ATMN balance updates
```

**Total integration time: ~2-4 weeks vs. 2-3 months without Rosetta**

---

## Rosetta Validator

Coinbase provides a [Rosetta validator](https://github.com/coinbase/rosetta-cli) to test compliance:

```bash
# Run against our implementation
rosetta-cli check:data \
  --configuration-file rosetta-cli-config.json \
  --start-index 0 \
  --end-index 100000

# Validates:
- Endpoint availability ✓
- Data consistency ✓
- Transaction finality ✓
- Balance tracking ✓
- No double spends ✓
```

---

## Current Status & Roadmap

### ✅ Phase 1 (Complete)
- Rosetta specification documented in SPECIFICATIONS.md
- Architecture planned in PHASE2_PLAN.md
- Types defined in atmn-core/src/types.rs

### 🚧 Phase 2b (Next)
- Create atmn-rosetta HTTP server module
- Implement data type converters
- Build network endpoints
- Basic block endpoint

### 🚧 Phase 2c (Following)
- Account balance endpoints
- Transaction query endpoints
- Mempool support

### 📋 Phase 3
- Construction endpoints (full transaction lifecycle)
- Rosetta validator compliance
- Exchange testnet integration

### 📋 Phase 4+
- Production deployment
- Mainnet exchange listings
- Cross-chain bridge support

---

## Why This Matters for ATMN

### 1. Competitive Advantage
- Most new blockchains: Custom exchange integrations (slow)
- ATMN: Standard Rosetta API (fast)
- Result: ATMN can list on 50+ exchanges immediately

### 2. Institutional Grade
- Rosetta = Professional infrastructure standard
- Used by: Coinbase, Kraken, Gemini, OKX, Binance
- Shows: ATMN is enterprise-ready

### 3. Developer Ecosystem
- Rosetta CLI tools work with ATMN
- Cross-blockchain explorers work with ATMN
- Cross-blockchain wallets work with ATMN
- Cross-blockchain indexers work with ATMN

### 4. Security Benefits
- Standardized validation prevents bugs
- Exchange audits focus on our code, not API
- Security reviews use standard checklist
- Community can audit against known standards

### 5. Time to Mainnet
- 2 weeks to working Rosetta = 2 weeks to exchange integration
- Without Rosetta = 2+ months per exchange
- With Rosetta + 10 exchanges = Saves ~150 developer weeks

---

## Implementation Checklist

### HTTP Server Foundation
- [ ] Create atmn-rosetta crate
- [ ] Setup Actix-web or Axum
- [ ] Add CORS middleware
- [ ] Error handling framework
- [ ] Logging integration

### Data Converters
- [ ] Block → RosettaBlock
- [ ] Transaction → RosettaTransaction
- [ ] Amount → RosettaAmount
- [ ] Account → RosettaAccount
- [ ] Operation → RosettaOperation

### Network Endpoints
- [ ] GET /network/list
- [ ] GET /network/options
- [ ] POST /network/status

### Block Endpoints
- [ ] POST /block
- [ ] POST /block/transaction
- [ ] POST /mempool
- [ ] POST /mempool/transaction

### Account Endpoints
- [ ] POST /account/balance
- [ ] POST /account/coins

### Construction Endpoints
- [ ] POST /construction/preprocess
- [ ] POST /construction/metadata
- [ ] POST /construction/payloads
- [ ] POST /construction/parse
- [ ] POST /construction/combine
- [ ] POST /construction/hash
- [ ] POST /construction/submit

### Testing
- [ ] Unit tests (data converters)
- [ ] Integration tests (endpoints)
- [ ] Rosetta validator compliance
- [ ] Exchange simulation tests
- [ ] Load testing

### Documentation
- [ ] API documentation
- [ ] Integration guide for exchanges
- [ ] Deployment instructions
- [ ] Troubleshooting guide

---

## References

### Official Rosetta
- [Rosetta API Specification](https://docs.cloud.coinbase.com/rosetta/docs)
- [Rosetta CLI Tool](https://github.com/coinbase/rosetta-cli)
- [Rosetta Samples](https://github.com/coinbase/rosetta-samples)
- [Coinbase Blog](https://www.coinbase.com/en-gb/blog/introducing-rosetta-build-once-integrate-your-blockchain-everywhere)

### Blockchain Implementations
- [Bitcoin Rosetta](https://github.com/coinbase/rosetta-bitcoin)
- [Ethereum Rosetta](https://github.com/coinbase/rosetta-ethereum)
- [Filecoin Rosetta](https://github.com/filecoin-project/rosetta-filecoin)

### HTTP Frameworks (Rust)
- [Actix-web](https://actix.rs/) - High-performance
- [Axum](https://github.com/tokio-rs/axum) - Modern, composable

---

**Next Step:** Begin Phase 2b with atmn-rosetta HTTP server module

