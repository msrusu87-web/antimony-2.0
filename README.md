# ANTIMONY COIN 2.0 🪨
**High-Performance Payment & Settlement Layer**

[![Status](https://img.shields.io/badge/Status-Testnet%20Live-success)](https://explorer.carphatian.ro)
[![Consensus](https://img.shields.io/badge/Consensus-SHA--256d-blue)](https://github.com/msrusu87-web/antimony-2.0)
[![Language](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-40%2F40%20passing-brightgreen)](https://github.com/msrusu87-web/antimony-2.0)
[![Storage](https://img.shields.io/badge/Storage-RocksDB-red)](https://rocksdb.org/)

## 🚀 Quick Start
**Live Testnet:**
- 🌐 **Mining Pool:** [miningpool.carphatian.ro](https://miningpool.carphatian.ro)
- 🔍 **Block Explorer:** [explorer.carphatian.ro](https://explorer.carphatian.ro)
- 💼 **Web Wallet:** [explorer.carphatian.ro/web-wallet.html](https://explorer.carphatian.ro/web-wallet.html)
- 🖥️ **Windows Wallet:** [explorer.carphatian.ro/wallet-download.html](https://explorer.carphatian.ro/wallet-download.html)
- 📚 **Documentation:** [explorer.carphatian.ro/docs.html](https://explorer.carphatian.ro/docs.html)

## What is ATMN 2.0?
Antimony Coin 2.0 is a high-performance blockchain designed for:

- ⚡ **Fast Settlements** - 12-second blocks, 0.0001 ATMN transaction fees
- 🪙 **Pure Proof-of-Work** - SHA-256d consensus (proven by Bitcoin 15+ years)
- 📊 **High Throughput** - Scalable payment settlement layer
- 🔒 **Security First** - Difficulty adjustment + double-spend prevention
- 💾 **Persistent Storage** - RocksDB with block/transaction indexing
- 💼 **Enterprise Ready** - Rosetta API v1.4.13 for exchanges
- 🌐 **Live Infrastructure** - Mining pool, explorer, and wallets deployed
- 🛡️ **UTXO Model** - Bitcoin-style unspent transaction outputs with validation

## 🏗️ Current Architecture

### ✅ TESTNET LIVE (Phase 3 Complete + Infrastructure)

**Core Components:**
- ✅ SHA-256d hashing engine
- ✅ Difficulty adjustment algorithm (every 2,016 blocks)
- ✅ Rosetta API v1.4.13 HTTP server (port 8080)
- ✅ RocksDB persistent storage (5 column families)
- ✅ Genesis block auto-initialization
- ✅ Block queries by height/hash
- ✅ Transaction indexing with metadata
- ✅ UTXO set management with double-spend prevention
- ✅ Continuous mining with auto-restart
- ✅ Transaction input/output tracking

**Live Infrastructure (Testnet):**

**✅ Mining Pool Backend** (Rust/Actix-web)
- Stratum protocol support
- Worker registration & management
- Share submission & validation
- Payout system with fee collection (2% pool fee)
- Real-time pool statistics

**✅ RESTful API Server** (Port 8000)
- Authentication with optional 2FA (TOTP)
- Wallet management endpoints
- Transaction creation & tracking
- Master wallet management (50M ATMN premine)
- Fee collection & distribution system
- **UTXO verification & double-spend prevention**

**✅ Windows Wallet v2.0** (NEW!)
- Full-featured desktop wallet with GUI (eframe/egui)
- **Integrated mining controls** (start/stop with one click)
- **Real-time hash rate monitoring** (updates every 2 seconds)
- **Configurable mining** (threads: 4-8 recommended, difficulty presets)
- 6-tab interface: Overview, Send, Receive, Transactions, Mining, Settings
- Balance checking & transaction history
- Auto-refresh wallet data
- Professional UI with status bar
- Cross-compiled for Windows 10/11 64-bit
- Download: [explorer.carphatian.ro/wallet-download.html](https://explorer.carphatian.ro/wallet-download.html)

**✅ Web Wallet Interface**
- User registration & login
- Multiple wallet support
- Send/Receive ATMN transactions
- Transaction history
- QR code generation
- Optional 2FA security (Google Authenticator)

**✅ Block Explorer**
- Real-time block visualization
- Transaction lookup
- Address balance checking
- Network statistics

**✅ Database Layer**
- SQLite with 15+ tables
- User authentication & wallets
- Transaction tracking (inputs/outputs)
- **UTXO table with spent tracking**
- Mining worker management
- Fee transaction audit trail
- Address balance caching

## 🎯 Recent Updates (December 5, 2025)

### ✅ Phase 3.3: Windows Wallet v2.0 (COMPLETE)
- Built full-featured Windows wallet with integrated mining
- 6 professional tabs with real-time updates
- Mining tab with start/stop controls, thread configuration
- Hash rate monitoring (updates every 2 seconds)
- Difficulty presets (Easy: 520093695, Normal: 486604799)
- Status bar showing balance, transactions, mining state
- Built with Rust/eframe/egui for native performance
- **Size:** 4.2 MB executable, 2.1 MB compressed
- **Tests:** Builds successfully, ready for user testing

### ✅ Phase 3.2: Double-Spend Prevention (COMPLETE)
- Implemented UTXO verification in `db.rs`
- Added `check_utxo_exists()` - Verify UTXO is unspent
- Added `verify_transaction_inputs()` - Validate all inputs
- Added `check_transaction_exists()` - Prevent replay attacks
- Added `mark_utxo_spent()` - Atomic UTXO spending
- Added `create_utxo()` - Generate new UTXOs
- Added `process_block_transactions()` - Process blocks atomically
- Enhanced `submit_block` handler with validation
- **Protection:** Double-spend attacks, replay attacks, invalid inputs
- **Status:** Deployed and running

### ✅ Phase 3.1: Transaction Indexing (COMPLETE)
- Created `transactions` table with full metadata
- Created `transaction_inputs` table with prev_tx references
- Created `transaction_outputs` table with addresses/amounts
- Added indexing in block submission handler
- Enables querying by hash, address, or block height

### ✅ Phase 3.0: Continuous Mining (COMPLETE)
- Auto-restart mining on block found
- Query database for latest block hash
- Continue mining on next height automatically
- No manual intervention needed

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

## 📈 Test Summary
**Total Tests Passing: 40/40 (100%)**

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
- UTXO validation working: ✅
- Double-spend prevention: ✅
- Windows wallet builds: ✅

## 🗺️ Development Roadmap

### ✅ Phase 2: Consensus & Storage (COMPLETE)
- ✅ 2a: SHA-256d consensus engine
- ✅ 2b: Rosetta API server
- ✅ 2c: RocksDB storage layer
- ✅ 2d: Account balance APIs & mempool
- ✅ 2e: Mining pool + Explorer + Web wallet infrastructure

### ✅ Phase 3: Transaction System (COMPLETE - December 5, 2025)
- ✅ 3.0: Continuous mining with auto-restart
- ✅ 3.1: Transaction indexing and storage
- ✅ 3.2: Double-spend prevention with UTXO validation
- ✅ 3.3: Windows Wallet v2.0 with integrated mining

### 🚧 Phase 4: Advanced Features (IN PROGRESS)
**Next Priority:**
- 🚧 Coinbase transaction validation (reward verification)
- 🚧 Blockchain Query API endpoints
- 🚧 Multi-threaded mining optimization (2-4x speedup)
- 🚧 Difficulty adjustment (every 2016 blocks)

**ETA:** 2-3 weeks

### 📋 Phase 5: P2P Networking (PLANNED)
- Node discovery protocol
- Block propagation layer
- Transaction broadcasting
- Full node implementation
- Multi-node consensus testing

**ETA:** 4-6 weeks

### 📋 Phase 6: Mainnet Preparation (PLANNED)
- Security audit (core + API)
- Stress testing (10,000+ TPS target)
- Exchange integration (Rosetta API)
- Public testnet with community miners
- Bug bounty program
- Mainnet genesis parameters

**ETA:** 8-12 weeks

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
cargo test --lib consensus    # Consensus tests
cargo test --lib storage      # Storage tests
cargo test --lib genesis      # Genesis tests
```

### Run Servers
```bash
# Rosetta API Server
cd atmn-rosetta
cargo run
# Server starts on http://0.0.0.0:8080

# Wallet/Mining API Server
cd atmn-api
cargo run --release
# Server starts on http://127.0.0.1:8000
```

## 🌐 Live Testnet Access

### Mining Pool
**URL:** https://miningpool.carphatian.ro
- Real-time pool statistics
- Worker management
- Share tracking
- Automatic payouts
- Fee transparency (2%)

### Block Explorer
**URL:** https://explorer.carphatian.ro
- Live block feed
- Transaction search
- Address lookup
- Network statistics
- Mining pool stats

### Web Wallet
**URL:** https://explorer.carphatian.ro/web-wallet.html
- Create account (email + password)
- Optional 2FA (Google Authenticator)
- Send/Receive ATMN
- Multiple wallet support
- Transaction history
- QR code generation

### Windows Wallet (NEW!)
**URL:** https://explorer.carphatian.ro/wallet-download.html
- Full desktop wallet with GUI
- **Integrated mining** (no separate miner needed)
- Real-time hash rate monitoring
- Configurable mining (threads/difficulty)
- Balance and transaction history
- Professional 6-tab interface
- Native Windows 10/11 application

## 🔧 Optimization Suggestions

### Performance
1. **Multi-threaded Mining** (Priority: HIGH)
   - Split nonce range across CPU cores
   - Expected 2-4x hash rate improvement
   - Implement work stealing for load balancing

2. **Database Optimization**
   - Add connection pooling (currently single connection)
   - Implement batch UTXO updates
   - Add indexes on frequently queried fields
   - Consider PostgreSQL for production

3. **API Caching**
   - Cache block data (immutable after 6 confirmations)
   - Cache address balances with TTL
   - Implement Redis for distributed caching

4. **Memory Management**
   - Limit mempool size (currently unbounded)
   - Implement UTXO set pruning for old blocks
   - Add block header-only mode for light clients

### Security
1. **Rate Limiting** (Priority: HIGH)
   - Add API rate limiting (100 req/min per IP)
   - Implement DDoS protection
   - Add CAPTCHA for wallet registration

2. **Input Validation**
   - Sanitize all user inputs
   - Add request size limits (10MB max)
   - Validate transaction structure before processing

3. **Audit & Testing**
   - Professional security audit needed
   - Fuzz testing for consensus code
   - Penetration testing for API endpoints

### Scalability
1. **WebSocket Updates**
   - Real-time block notifications
   - Live transaction updates
   - Reduce polling overhead

2. **Microservices Architecture**
   - Separate mining pool from API server
   - Dedicated block indexer service
   - Load balancer for horizontal scaling

3. **CDN Integration**
   - Serve static assets (explorer/wallet) from CDN
   - Reduce server bandwidth
   - Improve global access speed

### User Experience
1. **Mobile Wallets**
   - iOS app (Swift/SwiftUI)
   - Android app (Kotlin/Jetpack Compose)
   - React Native for cross-platform

2. **Hardware Wallet Support**
   - Ledger integration
   - Trezor support
   - BIP44 HD wallet standard

3. **Documentation**
   - Video tutorials
   - API playground (Swagger/OpenAPI)
   - Interactive mining guide

## 📊 Mainnet Checklist

### Core Requirements ✅/🚧/❌
- ✅ SHA-256d consensus working
- ✅ Block generation & validation
- ✅ Difficulty adjustment algorithm
- ✅ RocksDB persistent storage
- ✅ Genesis block initialization
- ✅ Transaction indexing
- ✅ UTXO set management
- ✅ Double-spend prevention
- ✅ Continuous mining
- ✅ Rosetta API (data endpoints)
- 🚧 Coinbase validation
- 🚧 Multi-threaded mining
- 🚧 Blockchain query APIs
- 🚧 P2P networking layer
- 🚧 Full mempool implementation
- 🚧 Rosetta Construction API
- ❌ Multi-node testing complete
- ❌ Security audit passed

### Infrastructure Requirements ✅/🚧/❌
- ✅ Mining pool operational
- ✅ Block explorer functional
- ✅ Web wallet with security
- ✅ Windows wallet with mining
- ✅ API server deployed
- ✅ Database layer stable
- ✅ Fee collection automated
- ✅ Master wallet management
- 🚧 Mobile wallet apps
- 🚧 Hardware wallet support
- 🚧 Rate limiting & DDoS protection
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

**Estimated Mainnet Launch:** Q1 2026

## 📄 License
MIT License - See LICENSE.md

## 🌟 Community
- **Website:** https://miningpool.carphatian.ro
- **Explorer:** https://explorer.carphatian.ro
- **Repository:** https://github.com/msrusu87-web/antimony-2.0
- **Issues:** https://github.com/msrusu87-web/antimony-2.0/issues
- **Discord:** Coming soon
- **Twitter:** Coming soon

---

**Built with ❤️ in Carpathia**

*Last Updated: December 5, 2025*  
*Testnet Live - 40/40 Core Tests ✅ + Full Infrastructure Deployed ✅ + Windows Wallet v2.0 ✅ + Double-Spend Prevention ✅*

**Repository:** https://github.com/msrusu87-web/antimony-2.0
