# ANTIMONY COIN 2.0 🪨

> **High-Performance Payment & Settlement Layer**

[![Status](https://img.shields.io/badge/Status-Phase%202d%20Account%20APIs%20Complete-brightgreen)](https://github.com/msrusu87-web/antimony-2.0)
[![Consensus](https://img.shields.io/badge/Consensus-Pure%20PoW-blue)](./CONSENSUS_IMPLEMENTATION.md)
[![Language](https://img.shields.io/badge/Language-Rust-brightgreen)](https://github.com/msrusu87-web/antimony-2.0)
[![Tests](https://img.shields.io/badge/Tests-42%2F42%20Passing-success)](./PHASE2D_SESSION_REPORT.md)
[![Storage](https://img.shields.io/badge/Storage-RocksDB%20✅-success)](./PHASE2C_SESSION_REPORT.md)

## Quick Start

### What is ATMN 2.0?

Antimony Coin 2.0 is a high-performance blockchain designed for:

- **⚡ Fast Settlements** - 12-second blocks, 0.0001 ATMN transaction fees
- **🪙 Pure Proof-of-Work** - SHA-256d consensus (proven by Bitcoin 15+ years)
- **📊 High Throughput** - Scalable payment settlement layer
- **🔒 Security First** - Difficulty adjustment prevents manipulation
- **💾 Persistent Storage** - RocksDB with block/transaction indexing
- **💼 Enterprise Ready** - Rosetta API v1.4.13 for exchanges

## Current Architecture

```
ATMN 2.0 - Phase 2: Consensus + Storage Foundation ✅

✅ COMPLETE (Phase 2a-2d):
- SHA-256d hashing engine
- Difficulty adjustment algorithm
- Rosetta API v1.4.13 HTTP server (port 8080)
- RocksDB persistent storage (6 column families)
- Genesis block auto-initialization
- Block queries by height/hash
- Transaction indexing with metadata
- UTXO set management with address indexing
- Account balance endpoints (/account/balance, /account/coins)

🚧 IN PROGRESS (Phase 2e):
- Mempool transaction pool
- Account transaction history
- Construction API (signing, submission)

📋 PLANNED (Phase 3+):
- Mining system with nonce
- P2P networking
- Full Rosetta Construction API
- EVM compatibility layer (Phase 4)
- Cross-chain bridges (Phase 5)
- AI inference module (Phase 5+)
```

## Key Specifications

```
Total Supply:          500,000,000 ATMN
Block Time:            12 seconds
Block Reward:          50 ATMN (Year 1) → 25 → 12.5 → 6.25+ ATMN
PoW Algorithm:         SHA-256d (Bitcoin-compatible)
Consensus:             Pure Proof-of-Work (100% to miners)
Difficulty Adjustment: Every 2,016 blocks (~4 hours)
Storage:               RocksDB with 6 column families
API:                   Rosetta v1.4.13 (Axum HTTP)
API Port:              8080 (configurable)
Network Type:          Mainnet / Testnet / Regtest
Address Format:        BIP32/BIP39/BIP44 (HD wallets)
Genesis Hash:          Configurable per network
```

## Current Status - Phase 2 Complete!

### ✅ Phase 2a: Consensus Engine (COMPLETE)
- SHA-256d hashing implementation
- Difficulty adjustment algorithm
- Block hash verification
- **26/26 tests passing** (100%)
- [Read Phase 2a Report](./PHASE2A_COMPLETION_REPORT.md)

### ✅ Phase 2b: Rosetta API Server (COMPLETE)
- Axum HTTP server on port 8080
- Rosetta v1.4.13 specification (15 endpoints)
- Full data conversion (atmn-core ↔ Rosetta)
- CORS support for explorer integration
- **6/6 tests passing** (100%)
- [Read Phase 2b Report](./PHASE2B_SESSION_REPORT.md)

### ✅ Phase 2c: RocksDB Storage & Genesis (COMPLETE)
- RocksDB persistent storage (6 column families)
- Block persistence by height & hash
- Transaction indexing with metadata
- Genesis block auto-initialization
- Rosetta API integration with real storage
- **8/8 tests passing** (5 storage + 3 genesis)
- Live verified: `/block` endpoint serves from database
- [Read Phase 2c Report](./PHASE2C_SESSION_REPORT.md)

### ✅ Phase 2d: Account Balance APIs & Address Indexing (COMPLETE)
- Address-to-UTXO indexing via CF_ADDRESS_INDEX column family
- `/account/balance` endpoint (query account balance)
- `/account/coins` endpoint (list spendable UTXOs)
- O(1) address lookup optimization
- **6/6 Rosetta tests passing** (11/11 total with storage)
- Live verified: Both account endpoints working with proper Rosetta format
- [Read Phase 2d Report](./PHASE2D_SESSION_REPORT.md)

### 📋 Phase 2e: Mempool & Transaction History (PLANNED)

```
Total Tests Passing: 42/42 (100%)

- Phase 2a (Consensus):  26/26 ✅
- Phase 2b (Rosetta):     6/6 ✅
- Phase 2c (Storage):     8/8 ✅ (5 storage + 3 genesis)
- Phase 2d (Accounts):    2/2 ✅ (live tested both endpoints)
```

## Development Roadmap

### Phase 2: Consensus & Storage (✅ COMPLETE)
- ✅ 2a: SHA-256d consensus engine
- ✅ 2b: Rosetta API server
- ✅ 2c: RocksDB storage layer
- ✅ 2d: Account balance APIs & address indexing

### Phase 2e: Mempool & Advanced Queries (🚧 NEXT)
- Mining system with nonce
- P2P networking
- Full Rosetta Construction API
- **ETA**: 2-4 weeks

### Phase 4+: Enterprise (📋 PLANNED)
- EVM compatibility layer
- Cross-chain bridges
- AI inference module
- DAO governance

## Getting Started

### Prerequisites
- Rust 1.70+
- Cargo
- Git
- RocksDB system deps (libz, libbz2, liblz4, libsnappy, libzstd)

### Clone & Build

```bash
git clone https://github.com/msrusu87-web/antimony-2.0.git
cd atmn-2.0

# Build core
cd atmn-core && cargo build --release

# Build Rosetta API
cd ../atmn-rosetta && cargo build --release
```

### Run Tests

```bash
cd atmn-core
cargo test --lib              # All tests
cargo test --lib consensus   # Consensus tests
cargo test --lib storage     # Storage tests
cargo test --lib genesis     # Genesis tests
```

### Run Rosetta API Server

```bash
cd atmn-rosetta
cargo run
# Server starts on http://0.0.0.0:8080
# Genesis block auto-initialized at ./data
```

### Test API

```bash
# Health check
curl http://localhost:8080/health

# Get genesis block
curl -X POST http://localhost:8080/block \
  -H "Content-Type: application/json" \
  -d '{
    "network_identifier":{"blockchain":"Antimony","network":"mainnet"},
    "block_identifier":{"index":0}
  }'
```

## Documentation

- [Technical Specifications](./SPECIFICATIONS.md) - 19 sections
- [Consensus Details](./CONSENSUS_IMPLEMENTATION.md) - SHA-256d
- [Phase 2a Report](./PHASE2A_COMPLETION_REPORT.md) - Consensus tests
- [Phase 2b Report](./PHASE2B_SESSION_REPORT.md) - Rosetta API details
- [Phase 2c Report](./PHASE2C_SESSION_REPORT.md) - Storage layer details ✅ NEW
- [Quick Start](./QUICKSTART.md) - Developer setup
- [Contributing](./CONTRIBUTING.md) - How to contribute

## Security

- **Algorithm**: SHA-256d (Bitcoin-proven since 2009)
- **Implementation**: Pure Rust with type safety
- **Storage**: Persistent RocksDB with file encryption ready
- **API**: Rosetta v1.4.13 (exchange-grade)

## Changelog

### Phase 2d (December 4, 2025) - Account Balance APIs ✅
- Implemented `/account/balance` endpoint
- Implemented `/account/coins` endpoint
- Added CF_ADDRESS_INDEX column family (6th CF)
- Address-to-UTXO indexing for O(1) lookups
- Fixed compilation error in address indexing
- All 6 Rosetta tests passing
- Both endpoints live-tested and working
- Total: 42/42 tests passing

### Phase 2c (Earlier) - RocksDB Storage ✅
- RocksDB integration (5 column families)
- Block persistence by height & hash
- Transaction indexing with metadata
- Genesis block auto-initialization
- 8/8 storage tests passing

### Phase 2b (Earlier) - Rosetta API ✅
- Axum HTTP server on port 8080
- 15 Rosetta Data API endpoints
- CORS support for explorers
- 6/6 handler tests passing

### Phase 2a (Earlier) - Consensus Engine ✅
- SHA-256d hashing implementation
- Difficulty adjustment algorithm
- 26/26 consensus tests passing

---

**Built with ❤️ in Carpathia**

Last Updated: December 4, 2025 (Phase 2d Complete - 42/42 Tests ✅)

Repository: https://github.com/msrusu87-web/antimony-2.0
