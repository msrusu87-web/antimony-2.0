# ANTIMONY COIN 2.0 🪨

> **The Enterprise-Grade AI-Powered Payment & DeFi Chain**

[![Status](https://img.shields.io/badge/Status-Pre--Launch-orange)](https://github.com/msrusu87-web/antimony-2.0)
[![Network](https://img.shields.io/badge/Network-Live%20Testnet-blue)](https://explorer.carphatian.ro)
[![Version](https://img.shields.io/badge/Version-2.0-brightgreen)](https://github.com/msrusu87-web/antimony-2.0/releases)

## Quick Start

### What is ATMN 2.0?

Antimony Coin 2.0 is a next-generation blockchain combining:

- **⚡ EVM Compatibility** - Deploy Ethereum smart contracts instantly
- **🤖 AI Integration** - On-chain AI inference and model marketplace
- **🌉 Cross-Chain** - Bridges to Ethereum, Polygon, Solana, BSC
- **🏛️ Governance** - Community-driven DAO with real decision power
- **💰 Real Assets** - Tokenize real estate, commodities, stocks
- **⏱️ Lightning Fast** - 12-second blocks, 0.0001 ATMN transactions

## Key Specifications

```
Total Supply:          500,000,000 ATMN
Block Time:            12 seconds
Block Reward (Year 1): 50 ATMN (Year 1), 25 ATMN (Year 2), 12.5 ATMN (Year 3+)
PoW Algorithm:         PHI1612 (ASIC-resistant)
Consensus:             PoW + Masternode (100,000 ATMN collateral initially)
Smart Contracts:       Solidity (EVM-compatible)
Network Ports:         7676 (P2P), 7674 (RPC), 7673 (WebSocket)
Address Format:        Bech32 (A-prefix for mainnet, TA-prefix for testnet)
```

## Architecture

```
├── atmn-core/              # Core blockchain implementation
│   ├── consensus/          # PoW/PoS consensus engine
│   ├── network/            # P2P networking layer
│   ├── storage/            # LevelDB state management
│   └── ai-module/          # On-chain AI inference
│
├── atmn-evm/               # EVM compatibility layer
│   ├── contracts/          # Smart contract templates
│   ├── precompiles/        # EVM precompiled contracts
│   └── runtime/            # Execution environment
│
├── atmn-explorer/          # Block explorer
│   ├── frontend/           # React SPA
│   ├── backend/            # Node.js GraphQL API
│   └── indexer/            # Real-time indexing
│
├── atmn-wallet-web/        # Web wallet
│   ├── src/                # React components
│   └── contracts/          # Wallet smart contracts
│
├── atmn-wallet-windows/    # Windows desktop wallet
├── atmn-wallet-macos/      # macOS desktop wallet
├── atmn-mining-pool/       # Mining pool (Stratum protocol)
├── atmn-bridge/            # Cross-chain bridges
├── atmn-contracts/         # Smart contract libraries
├── atmn-docs/              # Developer documentation
└── atmn-tests/             # Comprehensive test suite
```

## Domains

| Service | Domain |
|---------|--------|
| **Main Website** | https://antimony.carphatian.ro |
| **Block Explorer** | https://explorer.carphatian.ro |
| **Web Wallet** | https://antimony.carphatian.ro/wallet |
| **Mining Pool** | pool.antimony.carphatian.ro:5555 |
| **RPC Endpoint** | https://rpc.antimony.carphatian.ro |
| **WebSocket** | ws://ws.antimony.carphatian.ro |

## Quick Links

- 📋 [Full Specifications](./SPECIFICATIONS.md)
- 📖 [Developer Documentation](./atmn-docs/)
- 🧪 [Test Suite](./atmn-tests/)
- 💬 [Smart Contracts](./atmn-contracts/)
- 🔗 [Bridge Documentation](./atmn-bridge/)

## Development Roadmap

### Phase 1: Foundation (Jan-Feb 2025) ⏳
- [x] Project initialization
- [ ] Core blockchain implementation
- [ ] EVM integration layer
- [ ] AI module framework
- [ ] Testnet deployment

### Phase 2: Infrastructure (Mar 2025)
- [ ] Block explorer
- [ ] Web wallet
- [ ] Desktop wallets (Windows/macOS)
- [ ] Mining pool

### Phase 3: Testing (Apr 2025)
- [ ] Security audits
- [ ] Community testnet
- [ ] Cross-chain bridge testing
- [ ] Performance optimization

### Phase 4: Launch (May 2025)
- [ ] Mainnet genesis
- [ ] Mining activation
- [ ] Exchange listings
- [ ] Marketing campaign

## Installation & Setup

### Prerequisites
- Linux (Ubuntu 20.04+ or similar)
- Docker & Docker Compose
- Node.js 18+
- Go 1.20+
- Rust 1.70+ (optional, for core development)

### Clone & Initialize
```bash
git clone https://github.com/msrusu87-web/antimony-2.0.git
cd antimony-2.0
make init
```

### Run Testnet Node
```bash
cd atmn-core
./build.sh
./atmn-node --testnet --log-level=info
```

### Deploy Smart Contracts
```bash
cd atmn-contracts
npm install
npm run deploy:testnet
```

## Security

### Audits
- [ ] Pre-launch security audit (Professional firm)
- [ ] Smart contract audit (OpenZeppelin/Trail of Bits)
- [ ] Consensus mechanism review (Academic experts)

### Bug Bounty
- Maximum bounty: 5% of treasury (~1.5M ATMN)
- Tiers: Critical (1M+), High (100K+), Medium (10K+), Low (1K+)
- Submission: security@antimony.carphatian.ro

## Community

- **Twitter**: [@AtmonyOfficial](https://twitter.com/atmonyofficial)
- **Discord**: [Join Server](https://discord.gg/atmony)
- **Telegram**: [@AtmonyChat](https://t.me/atmony)
- **GitHub**: [msrusu87-web/antimony-2.0](https://github.com/msrusu87-web/antimony-2.0)

## Contributing

We welcome contributions from developers, designers, and community members!

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

See [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines.

## License

This project is licensed under the MIT License with additional clauses:
- Commercial use allowed with attribution
- Patents: Non-aggressive patent clause
- See [LICENSE.md](./LICENSE.md) for full details

## Disclaimer

**⚠️ IMPORTANT**: This project is in active development. Do not send real funds to testnet addresses. All testnet coins have zero value.

- Use at your own risk
- Security audits pending
- Blockchain technology is experimental
- Past performance ≠ future results

## Support & Funding

### Development Grants
Contact: funding@antimony.carphatian.ro

### Sponsorship
- Platinum: $50,000+ (Mainnet naming rights)
- Gold: $10,000+ (Docs sponsorship)
- Silver: $5,000+ (Community support)
- Bronze: $1,000+ (Bug bounty boost)

## Team

| Role | Name | GitHub |
|------|------|--------|
| Project Lead | Marian Rusu | [@msrusu87-web](https://github.com/msrusu87-web) |
| TBD | TBD | TBD |

## Acknowledgments

- Bitcoin Core team for blockchain foundations
- Ethereum for EVM innovation
- Cosmos SDK for modular design inspiration
- Community testers and contributors

---

**Built with ❤️ for a decentralized future**

Last Updated: December 4, 2025
