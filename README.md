# ANTIMONY COIN 2.0 🪨
**High-Performance Payment & Settlement Layer**

[![Status](https://img.shields.io/badge/Status-Testnet-yellow)](https://miningpool.carphatian.ro)
[![Consensus](https://img.shields.io/badge/Consensus-SHA--256d-green)](docs/consensus.md)
[![Language](https://img.shields.io/badge/Language-Rust-orange)](https://rust-lang.org)
[![Tests](https://img.shields.io/badge/Tests-36%2F36%20%E2%9C%85-brightgreen)](atmn-core/tests)
[![Storage](https://img.shields.io/badge/Storage-RocksDB-blue)](docs/storage.md)

---

## 🚀 Quick Start

**Live Testnet:**
- 🌐 Mining Pool: [miningpool.carphatian.ro](https://miningpool.carphatian.ro)
- 🔍 Block Explorer: [explorer.carphatian.ro](https://explorer.carphatian.ro)
- 💼 Web Wallet: [explorer.carphatian.ro/web-wallet.html](https://explorer.carphatian.ro/web-wallet.html)
- 📚 Documentation: [explorer.carphatian.ro/docs.html](https://explorer.carphatian.ro/docs.html)

---

## What is ATMN 2.0?

Antimony Coin 2.0 is a high-performance blockchain designed for:

- ⚡ **Fast Settlements** - 12-second blocks, 0.0001 ATMN transaction fees
- 🪙 **Pure Proof-of-Work** - SHA-256d consensus (proven by Bitcoin 15+ years)
- 📊 **High Throughput** - Scalable payment settlement layer
- 🔒 **Security First** - Difficulty adjustment prevents manipulation
- 💾 **Persistent Storage** - RocksDB with block/transaction indexing
- 💼 **Enterprise Ready** - Rosetta API v1.4.13 for exchanges
- 🌐 **Live Infrastructure** - Mining pool, explorer, and web wallet deployed

---

## 🏗️ Current Architecture

### ✅ TESTNET LIVE (Phase 2 Complete + Infrastructure)

**Core Components:**
- ✅ SHA-256d hashing engine
- ✅ Difficulty adjustment algorithm (every 2,016 blocks)
- ✅ Rosetta API v1.4.13 HTTP server (port 8080)
- ✅ RocksDB persistent storage (5 column families)
- ✅ Genesis block auto-initialization
- ✅ Block queries by height/hash
- ✅ Transaction indexing with metadata
- ✅ UTXO set management foundation

**Live Infrastructure (Testnet):**
- ✅ Mining Pool Backend (Rust/Actix-web)
  - Stratum protocol support
  - Worker registration & management
  - Share submission & validation
  - Payout system with fee collection (2% pool fee)
  - Real-time pool statistics
- ✅ RESTful API Server (Port 8000)
  - Authentication with optional 2FA (TOTP)
  - Wallet management endpoints
  - Transaction creation & tracking
  - Master wallet management (50M ATMN premine)
  - Fee collection & distribution system
- ✅ Web Wallet Interface
  - User registration & login
  - Multiple wallet support
  - Send/Receive ATMN transactions
  - Transaction history
  - QR code generation
  - Optional 2FA security (Google Authenticator)
- ✅ Block Explorer
  - Real-time block visualization
  - Transaction lookup
  - Address balance checking
  - Network statistics
- ✅ Database Layer
  - SQLite with 12 tables
  - User authentication & wallets
  - Transaction tracking
  - Mining worker management
  - Fee transaction audit trail

### 🚧 IN PROGRESS (Phase 2d):
- Account balance endpoints (/account/balance, /account/coins)
- Address-to-UTXO indexing optimization
- Mempool implementation
- Enhanced transaction lookup

### 📋 MAINNET REQUIREMENTS (Phase 3):
**Critical for Mainnet Launch:**
1. ✅ Mining pool operational (Complete)
2. ✅ Block explorer functional (Complete)
3. ✅ Web wallet with security (Complete)
4. 🚧 Full P2P networking layer
5. 🚧 Network consensus testing (multiple nodes)
6. 🚧 Security audit of core components
7. 🚧 Stress testing (transaction throughput)
8. 🚧 Final difficulty adjustment verification
9. 🚧 Mainnet genesis block parameters
10. 🚧 Exchange integration testing (Rosetta API)

**Nice-to-Have for Mainnet:**
- Mobile wallet applications
- Hardware wallet support
- Advanced pool features (merged mining)
- Governance proposals system

### 📋 PLANNED (Phase 4+):
- Full Rosetta Construction API
- Advanced mining optimizations
- Enhanced P2P protocols
- EVM compatibility layer
- Cross-chain bridges
- AI inference module
- DAO governance

---

## 📊 Key Specifications

| Parameter | Value |
|-----------|-------|
| **Total Supply** | 500,000,000 ATMN |
| **Premine** | 50,000,000 ATMN (10% - development fund) |
| **Block Time** | 12 seconds |
| **Block Reward** | 50 ATMN (Year 1) → Halving schedule |
| **PoW Algorithm** | SHA-256d (Bitcoin-compatible) |
| **Consensus** | Pure Proof-of-Work (100% to miners after premine) |
| **Difficulty Adjustment** | Every 2,016 blocks (~6.7 hours) |
| **Pool Fee** | 2% (auto-distributed to master wallet) |
| **Transaction Fee** | 0.0001 ATMN minimum |
| **Storage** | RocksDB with 5 column families |
| **API** | Rosetta v1.4.13 (Axum HTTP) |
| **Mining API** | RESTful + Stratum |
| **Wallet API** | RESTful with JWT auth |
| **API Port** | 8000 (wallet/mining), 8080 (Rosetta) |
| **Network Type** | Testnet (Mainnet TBD) |
| **Address Format** | BIP32/BIP39/BIP44 (HD wallets) |

---

## 🎯 Current Status - Testnet Active!

### ✅ Phase 2a: Consensus Engine (COMPLETE)
- SHA-256d hashing implementation
- Difficulty adjustment algorithm
- Block hash verification
- **26/26 tests passing (100%)**
- [Read Phase 2a Report](docs/phase-2a-report.md)

### ✅ Phase 2b: Rosetta API Server (COMPLETE)
- Axum HTTP server on port 8080
- Rosetta v1.4.13 specification (15 endpoints)
- Full data conversion (atmn-core ↔ Rosetta)
- CORS support for explorer integration
- **6/6 tests passing (100%)**
- [Read Phase 2b Report](docs/phase-2b-report.md)

### ✅ Phase 2c: RocksDB Storage & Genesis (COMPLETE)
- RocksDB persistent storage (5 column families)
- Block persistence by height & hash
- Transaction indexing with metadata
- Genesis block auto-initialization
- Rosetta API integration with real storage
- **8/8 tests passing (5 storage + 3 genesis)**
- Live verified: /block endpoint serves from database
- [Read Phase 2c Report](docs/phase-2c-report.md)

### ✅ Phase 2e: Live Infrastructure (COMPLETE - TESTNET)
- **Mining Pool Backend**
  - Stratum server for miners
  - Worker management & statistics
  - Share validation (SHA-256d)
  - Automatic payout system
  - Pool fee collection (2%)
  - Master wallet integration
- **RESTful API Server**
  - User authentication (SHA-256 password hashing)
  - Optional 2FA (TOTP/Google Authenticator)
  - Wallet CRUD operations
  - Transaction endpoints
  - Fee management system
  - Master wallet controls
- **Web Wallet**
  - Responsive UI (mobile-friendly)
  - Account registration/login
  - Multiple wallet support
  - Send/Receive functionality
  - Transaction history
  - QR code generation
  - Optional 2FA management
  - Security settings
- **Block Explorer**
  - Real-time block display
  - Transaction lookup
  - Address balance queries
  - Network statistics
  - Mining pool stats
- **Database Infrastructure**
  - 12 tables (users, wallets, transactions, etc.)
  - Master wallet tracking
  - Fee transaction audit log
  - Mining worker records
  - Pool statistics

### 🚧 Phase 2d: Account Balance APIs (IN PROGRESS)
- Address-to-UTXO indexing
- /account/balance endpoint
- /account/coins endpoint
- **ETA: 2-3 hours**

---

## 📈 Test Summary

**Total Tests Passing: 36/36 (100%)**

- Phase 2a (Consensus): 26/26 ✅
- Phase 2b (Rosetta): 6/6 ✅
- Phase 2c (Storage): 8/8 ✅ (5 storage + 3 genesis)

**Live Infrastructure Tests:**
- API endpoints responding: ✅
- User authentication working: ✅
- Wallet operations functional: ✅
- Mining pool active: ✅
- Block explorer operational: ✅
- Database connections stable: ✅
- Fee collection automated: ✅

---

## 🗺️ Development Roadmap

### Phase 2: Consensus & Storage (✅ COMPLETE - TESTNET LIVE)
- ✅ 2a: SHA-256d consensus engine
- ✅ 2b: Rosetta API server
- ✅ 2c: RocksDB storage layer
- 🚧 2d: Account balance APIs & mempool
- ✅ 2e: Mining pool + Explorer + Web wallet infrastructure

### Phase 3: Mainnet Preparation (🚧 IN PROGRESS)
**ETA: 4-6 weeks**
- Full P2P networking implementation
- Multi-node consensus testing
- Security audit (core + API)
- Stress testing (10,000+ TPS target)
- Exchange integration (Rosetta API verification)
- Mainnet genesis parameters finalization
- Public testnet with community miners
- Bug bounty program

### Phase 4: Enterprise Features (📋 PLANNED)
**ETA: 8-12 weeks after mainnet**
- Full Rosetta Construction API
- EVM compatibility layer
- Advanced mining features
- Hardware wallet integration
- Mobile applications (iOS/Android)
- Cross-chain bridges (BTC, ETH)
- Enhanced P2P protocols

### Phase 5+: Advanced Ecosystem (📋 FUTURE)
- AI inference module
- DAO governance system
- DeFi primitives
- NFT marketplace
- Decentralized exchange (DEX)
- Layer 2 scaling solutions

---

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+
- Cargo
- Git
- RocksDB system deps (libz, libbz2, liblz4, libsnappy, libzstd)
- SQLite3 (for wallet/mining APIs)

### Clone & Build
```bash
git clone https://github.com/msrusu87-web/antimony-2.0.git
cd atmn-2.0

# Build core
cd atmn-core && cargo build --release

# Build Rosetta API
cd ../atmn-rosetta && cargo build --release

# Build Mining Pool & API
cd ../atmn-api && cargo build --release
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

### Run Wallet/Mining API Server
```bash
cd atmn-api
cargo run --release
# Server starts on http://127.0.0.1:8000
# Requires DATABASE_URL environment variable
```

### Test APIs
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

# Test wallet API
curl -X POST https://explorer.carphatian.ro/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"password"}'
```

---

## 🌐 Live Testnet Access

### Mining Pool
**URL:** https://miningpool.carphatian.ro

**Features:**
- Real-time pool statistics
- Worker management
- Share tracking
- Automatic payouts
- Fee transparency (2%)

### Block Explorer
**URL:** https://explorer.carphatian.ro

**Features:**
- Live block feed
- Transaction search
- Address lookup
- Network statistics
- Mining pool stats

### Web Wallet
**URL:** https://explorer.carphatian.ro/web-wallet.html

**Features:**
- Create account (email + password)
- Optional 2FA (Google Authenticator)
- Send/Receive ATMN
- Multiple wallet support
- Transaction history
- QR code generation
- Secure (SHA-256 + optional TOTP)

### API Documentation
**URL:** https://explorer.carphatian.ro/docs.html

**Sections:**
- Quick Start Guide
- Mining Pool Setup
- API Reference
- Wallet Integration
- Node Operation
- Rosetta API Details

---

## 📚 Documentation

- [Technical Specifications](docs/specifications.md) - 19 sections
- [Consensus Details](docs/consensus.md) - SHA-256d
- [Phase 2a Report](docs/phase-2a-report.md) - Consensus tests
- [Phase 2b Report](docs/phase-2b-report.md) - Rosetta API details
- [Phase 2c Report](docs/phase-2c-report.md) - Storage layer details
- [API Documentation](docs/api.md) - RESTful endpoints
- [Mining Guide](docs/mining.md) - Pool setup & operation
- [Wallet Guide](docs/wallet.md) - Web wallet usage
- [Quick Start](docs/quickstart.md) - Developer setup
- [Contributing](CONTRIBUTING.md) - How to contribute

---

## 🔒 Security

- **Algorithm:** SHA-256d (Bitcoin-proven since 2009)
- **Implementation:** Pure Rust with type safety
- **Storage:** Persistent RocksDB with file encryption ready
- **API:** Rosetta v1.4.13 (exchange-grade)
- **Authentication:** SHA-256 password hashing + optional TOTP 2FA
- **Database:** SQLite with prepared statements (SQL injection prevention)
- **CORS:** Configured for security (same-origin policy)
- **HTTPS:** TLS 1.3 on all public endpoints
- **Rate Limiting:** API protection (coming soon)

---

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)

**Areas needing help:**
- P2P networking implementation
- Mobile wallet development
- Security auditing
- Performance optimization
- Documentation improvements
- Translation (i18n)

---

## 📊 Mainnet Checklist

### Core Requirements ✅/🚧/❌
- ✅ SHA-256d consensus working
- ✅ Block generation & validation
- ✅ Difficulty adjustment algorithm
- ✅ RocksDB persistent storage
- ✅ Genesis block initialization
- ✅ Transaction indexing
- ✅ Rosetta API (data endpoints)
- 🚧 P2P networking layer
- 🚧 Full mempool implementation
- 🚧 Rosetta Construction API
- ❌ Multi-node testing complete
- ❌ Security audit passed

### Infrastructure Requirements ✅/🚧/❌
- ✅ Mining pool operational
- ✅ Block explorer functional
- ✅ Web wallet with security
- ✅ API server deployed
- ✅ Database layer stable
- ✅ Fee collection automated
- ✅ Master wallet management
- 🚧 Mobile wallet apps
- 🚧 Hardware wallet support
- ❌ Exchange listings confirmed
- ❌ Community nodes running (10+)
- ❌ Public testnet stress tested

### Documentation Requirements ✅/🚧/❌
- ✅ Technical specifications complete
- ✅ API documentation published
- ✅ Mining guide available
- ✅ Wallet user guide
- ✅ Quick start tutorial
- 🚧 Node operator guide
- 🚧 Exchange integration guide
- 🚧 Security best practices
- ❌ White paper final version
- ❌ Tokenomics detailed analysis

### **Estimated Mainnet Launch: Q1 2026**

---

## 📄 License

MIT License - See [LICENSE.md](LICENSE.md)

---

## 🌟 Community

- **Website:** https://miningpool.carphatian.ro
- **Explorer:** https://explorer.carphatian.ro
- **Repository:** https://github.com/msrusu87-web/antimony-2.0
- **Issues:** https://github.com/msrusu87-web/antimony-2.0/issues
- **Discord:** Coming soon
- **Twitter:** Coming soon

---

**Built with ❤️ in Carpathia**

**Last Updated:** December 5, 2025 (Testnet Live - 36/36 Core Tests ✅ + Full Infrastructure Deployed ✅)

**Repository:** https://github.com/msrusu87-web/antimony-2.0
