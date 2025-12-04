# ANTIMONY COIN 2.0 🪨

> **High-Performance Payment & Settlement Layer**

[![Status](https://img.shields.io/badge/Status-Phase%202%20Development-orange)](https://github.com/msrusu87-web/antimony-2.0)
[![Consensus](https://img.shields.io/badge/Consensus-Pure%20PoW-blue)](./CONSENSUS_IMPLEMENTATION.md)
[![Language](https://img.shields.io/badge/Language-Rust%20%26%20TypeScript-brightgreen)](https://github.com/msrusu87-web/antimony-2.0)
[![Tests](https://img.shields.io/badge/Tests-26%2F26%20Passing-success)](./PHASE2A_COMPLETION_REPORT.md)

## Quick Start

### What is ATMN 2.0?

Antimony Coin 2.0 is a high-performance blockchain designed for:

- **⚡ Fast Settlements** - 12-second blocks, 0.0001 ATMN transaction fees
- **🪙 Pure Proof-of-Work** - SHA-256d consensus (proven by Bitcoin 15+ years)
- **📊 High Throughput** - Scalable payment settlement layer
- **🔒 Security First** - Difficulty adjustment prevents manipulation
- **🏗️ Modular Architecture** - Foundation for future features (EVM, AI, bridges)
- **💼 Enterprise Ready** - Rosetta API compatible for exchanges

## Current Architecture

```
ATMN 2.0 - Phase 2: Consensus Foundation

✅ COMPLETE:
- SHA-256d hashing engine
- Difficulty adjustment algorithm
- Block validation framework
- Proof-of-Work verification

🚧 IN PROGRESS:
- Merkle tree implementation
- Full block validation pipeline
- Storage integration (RocksDB)

📋 PLANNED:
- EVM compatibility layer (Phase 3)
- Cross-chain bridges (Phase 4)
- AI inference module (Phase 4+)
- DAO governance (Phase 4+)
```

## Key Specifications

```
Total Supply:          500,000,000 ATMN
Block Time:            12 seconds
Block Reward:          50 ATMN (Year 1) → 25 → 12.5 → 6.25+ ATMN
PoW Algorithm:         SHA-256d (Bitcoin-compatible)
Consensus:             Pure Proof-of-Work (100% to miners)
Difficulty Adjustment: Every 2,016 blocks (~4 hours)
Network Type:          Mainnet / Testnet / Regtest
Address Format:        BIP32/BIP39/BIP44 (HD wallets)
Ports:                 (Configurable per network)
```

## Project Structure

```
atmn-2.0/
├── atmn-core/                      # Core blockchain (ACTIVE - Phase 2)
│   ├── src/
│   │   ├── lib.rs                  # Main entry point
│   │   ├── types.rs                # BlockHash, TxHash, Amount, etc.
│   │   ├── error.rs                # Error types
│   │   ├── chain_params.rs         # Network parameters (mainnet/testnet/regtest)
│   │   ├── block.rs                # Block structures and validation
│   │   ├── transaction.rs          # Transaction types
│   │   ├── consensus.rs            # SHA-256d PoW engine ✅ IMPLEMENTED
│   │   ├── storage.rs              # Database abstraction (RocksDB)
│   │   ├── network.rs              # P2P networking skeleton
│   │   └── rosetta.rs              # Rosetta API support (preparation)
│   ├── Cargo.toml
│   └── tests/
│
├── SPECIFICATIONS.md               # Complete technical specifications (19 sections)
├── CONSENSUS_IMPLEMENTATION.md     # SHA-256d algorithm documentation
├── PHASE2_PLAN.md                  # Phase 2 implementation roadmap
├── PHASE2A_COMPLETION_REPORT.md    # Current phase status & achievements
├── QUICKSTART.md                   # Developer quick-start guide
├── CONTRIBUTING.md                 # Contributing guidelines
├── LICENSE.md                      # MIT + non-aggressive patents
├── README.md                        # This file
└── .gitignore

Planned (Future Phases):
├── atmn-evm/          # EVM compatibility layer (Phase 3)
├── atmn-storage/      # Full storage implementation (Phase 3)
├── atmn-mining/       # Mining system & pool (Phase 3)
├── atmn-network/      # P2P networking (Phase 3)
├── atmn-wallet/       # Web/desktop wallets (Phase 4)
├── atmn-bridge/       # Cross-chain bridges (Phase 4)
└── atmn-explorer/     # Block explorer (Phase 4)
```

## Current Status

### Phase 2a: Consensus Engine ✅ COMPLETE
- ✅ SHA-256d hashing implementation
- ✅ Difficulty adjustment algorithm
- ✅ Block hash verification
- ✅ 26/26 tests passing (100%)
- ✅ Production-ready code
- ✅ Comprehensive documentation (410+ lines)

**See:** [PHASE2A_COMPLETION_REPORT.md](./PHASE2A_COMPLETION_REPORT.md)

### Phase 2b: Block Validation (IN PROGRESS)
- 🚧 Merkle tree implementation
- 🚧 Full block validation pipeline
- 🚧 Coinbase transaction validation
- 📅 Expected: 1-2 weeks

### Phase 3: Storage & Mining (PLANNED)
- 📋 RocksDB integration
- 📋 UTXO set management
- 📋 Mining nonce system
- 📋 Block template creation
- 📋 P2P networking
- 📅 Expected: 4-6 weeks

## Quick Links

- 📋 [Full Technical Specifications](./SPECIFICATIONS.md) - 19 sections, complete feature set
- 📖 [Consensus Implementation](./CONSENSUS_IMPLEMENTATION.md) - SHA-256d details, security, performance
- 🧪 [Phase 2 Completion Report](./PHASE2A_COMPLETION_REPORT.md) - Test results, metrics, roadmap
- 🚀 [Quick Start Guide](./QUICKSTART.md) - Developer setup
- 🤝 [Contributing Guidelines](./CONTRIBUTING.md) - How to contribute
- ⚖️ [License](./LICENSE.md) - MIT with patents clause

## Development Roadmap

### Phase 1: Core Blockchain (COMPLETE ✅)
- [x] Project initialization
- [x] Technical specifications (19 sections)
- [x] Pure PoW architecture (simplified from hybrid)
- [x] Modular Rust project structure
- [x] GitHub repository setup

### Phase 2: Consensus Foundation (IN PROGRESS 🚧)
- [x] **2a: SHA-256d PoW Engine** ✅
  - [x] SHA-256d hashing
  - [x] Difficulty adjustment algorithm
  - [x] Block validation framework
  - [x] 26/26 tests passing
  - [x] Comprehensive documentation
  
- 🚧 **2b: Block Validation** (1-2 weeks)
  - Merkle tree implementation
  - Full block validation pipeline
  - Coinbase transaction validation
  
- 🚧 **2c: Storage & Mining** (2-4 weeks)
  - RocksDB integration
  - UTXO set management
  - Mining nonce system

### Phase 3: Infrastructure (PLANNED 📋)
- [ ] Block explorer (web-based)
- [ ] Web wallet (HD wallets, BIP39)
- [ ] Desktop wallets (Windows/macOS)
- [ ] Mining pool (Stratum protocol)
- [ ] P2P networking (peer discovery, block propagation)

### Phase 4: Advanced Features (PLANNED 📋)
- [ ] EVM compatibility layer (smart contracts)
- [ ] Cross-chain bridges
- [ ] AI inference module
- [ ] DAO governance
- [ ] RWA tokenization

### Phase 5: Launch & Ecosystem (PLANNED 📋)
- [ ] Security audits
- [ ] Mainnet genesis
- [ ] Exchange listings
- [ ] Community marketing

## Getting Started

### Development Prerequisites
```
- Rust 1.70+
- Cargo (comes with Rust)
- Git
- Linux/macOS or WSL on Windows
```

### Clone & Build
```bash
git clone https://github.com/msrusu87-web/antimony-2.0.git
cd atmn-2.0

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build core library
cd atmn-core
cargo build --release
```

### Run Tests
```bash
cd atmn-core

# Run all tests
cargo test

# Run specific module
cargo test --lib consensus

# Run with output
cargo test --lib -- --nocapture
```

### Documentation
```bash
# Generate Rust documentation
cargo doc --open

# View specifications
cat SPECIFICATIONS.md

# View consensus details
cat CONSENSUS_IMPLEMENTATION.md
```

## Security

### Cryptographic Foundation
- **Algorithm**: SHA-256d (proven by Bitcoin since 2009)
- **Hashing**: 256-bit cryptographic security level
- **Consensus**: Difficulty adjustment prevents manipulation
- **Implementation**: Pure Rust with type safety

### Planned Audits
- [ ] Code review (internal + community)
- [ ] Smart contract audit (when EVM layer added)
- [ ] Consensus mechanism review (academic)

### Security Contact
For security issues: security@antimony.carphatian.ro

## Architecture & Design

### Why Pure PoW?
- **Proven**: 15+ years of Bitcoin operation
- **Secure**: Resistant to 51% attacks with difficulty adjustment
- **Simple**: Reduced attack surface vs. hybrid models
- **Fair**: Equal opportunity for all miners
- **Scalable**: Foundation for future layers (EVM, bridges, AI)

### Simplification Strategy
- Removed masternode layer (reduced complexity)
- Removed governance DAO from consensus (can add via hardfork)
- Focused on fast, reliable settlement layer
- Pure PoW allows future features without consensus changes

## Contributing

We welcome contributions at all levels:

1. **Code Contributors**
   - Fork the repo
   - Create feature branch: `git checkout -b feature/my-feature`
   - Commit: `git commit -m 'Add my feature'`
   - Push: `git push origin feature/my-feature`
   - Open Pull Request

2. **Documentation Contributors**
   - Improve specs, guides, or comments
   - Submit via Pull Request

3. **Testers**
   - Report issues via GitHub Issues
   - Help with consensus testing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.

## License

MIT License with additional clauses:
- ✅ Commercial use allowed (with attribution)
- ✅ Modifications allowed
- ✅ Distribution allowed
- ⚠️ Non-aggressive patent clause (see LICENSE.md)

## Disclaimer

**⚠️ WARNING**: This project is in active development.

- Use at your own risk
- Security audits pending completion
- Blockchain technology is experimental
- Do not deploy production funds on testnet
- Testnet coins have zero value

## Community

- **GitHub**: [msrusu87-web/antimony-2.0](https://github.com/msrusu87-web/antimony-2.0)
- **Issues**: [GitHub Issues](https://github.com/msrusu87-web/antimony-2.0/issues)
- **Discussions**: [GitHub Discussions](https://github.com/msrusu87-web/antimony-2.0/discussions)

## Technical References

### Implemented Standards
- [Bitcoin PoW](https://en.bitcoin.it/wiki/Proof_of_work)
- [SHA-256 (FIPS 180-4)](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf)
- [BIP32/39/44 (Hierarchical Wallets)](https://github.com/bitcoin/bips)

### Planned Compatibility
- [Rosetta API](https://www.rosetta-api.org/) - Exchange integration
- [EIP-20 (Token Standard)](https://eips.ethereum.org/EIPS/eip-20) - When EVM added
- [BIP141 (Segregated Witness)](https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki)

## Project Status

| Component | Status | Progress | ETA |
|-----------|--------|----------|-----|
| Core PoW | ✅ Complete | 100% | Done |
| Block Validation | 🚧 In Progress | 50% | 1-2 weeks |
| Storage Layer | 📋 Planned | 0% | 2-4 weeks |
| Mining System | 📋 Planned | 0% | 3-5 weeks |
| EVM Layer | 📋 Future | 0% | Post-testnet |
| Bridges | 📋 Future | 0% | Post-testnet |

## Changelog

### Phase 2a - SHA-256d Consensus (December 4, 2025)
- ✅ Implemented SHA-256d hashing
- ✅ Difficulty adjustment algorithm
- ✅ Block hash verification
- ✅ 26/26 tests passing
- ✅ 410+ lines of documentation

---

**Built with ❤️ in Carpathia**

*Last Updated: December 4, 2025*
