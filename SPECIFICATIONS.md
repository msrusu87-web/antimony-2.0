# ANTIMONY COIN 2.0 - Complete Specifications

## Project Overview
**ATMN: The Enterprise-Grade AI-Powered Payment & DeFi Chain**

A next-generation cryptocurrency platform combining:
- **EVM Compatibility** for instant DeFi adoption
- **AI Integration** for unique market positioning
- **Cross-chain Bridges** for maximum reach
- **Institutional RWA Support** for enterprise adoption
- **Fast Payments & Low Fees** for everyday transactions

**Domain**: antimony.carphatian.ro  
**Explorer**: explorer.carphatian.ro  
**Status**: Pre-Launch Phase (Mainnet Q2 2025)

---

## 1. TOKENOMICS & SUPPLY

```
Total Supply:                 500,000,000 ATMN
├─ Genesis Block:             50,000,000 ATMN (10% pre-mine)
├─ Mining Rewards Pool:       400,000,000 ATMN (80%)
├─ Reserve/Treasury:          30,000,000 ATMN (6%)
├─ Team/Dev Fund:             15,000,000 ATMN (3%)
└─ Staking/Growth:             5,000,000 ATMN (1%)
```

### Block Reward Schedule (Mining Phase)

**Phase 1: Year 1 (Blocks 1-2,628,000)**
- Blocks 1-525,600 (Year 1):     50 ATMN per block = 26,280,000 ATMN
- Daily emission: 6,300 ATMN (48h blocks at 12s interval)

**Phase 2: Year 2-3 (Blocks 525,601-2,628,000)**
- Blocks 525,601-1,051,200:       25 ATMN per block = 13,140,000 ATMN
- Blocks 1,051,201-2,628,000:     12.5 ATMN per block = 19,710,000 ATMN

**Phase 3: Year 4+ (Blocks 2,628,001+)**
- 6.25 ATMN per block (PoW phase)
- Masternode rewards only after block 5,256,000

**Phase 4: Transition to Pure PoW (Year 5+)**
- After block 5,256,000: No mining rewards
- Shift to governance/staking model only
- Network security via validator set

---

## 2. NETWORK PARAMETERS

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| **Block Time** | 12 seconds | Fast finality, Ethereum parity |
| **Target Timespan** | 20 minutes | Difficulty adjustment period |
| **Block Size** | 8 MB | Support 1000+ tx/block |
| **Max Transaction Size** | 1 MB | Standard for enterprise |
| **Mempool Size** | 300 MB | Buffer for peak load |
| **Transaction Fee** | 0.0001 ATMN | ~$0.001 USD at launch |
| **Smart Contract Gas Limit** | 10,000,000 | Similar to Ethereum |
| **Base Gas Price** | 1 Gwei | Competitive with Polygon |

### Network Ports
```
Mainnet:
├─ P2P Network: 7676
├─ RPC Endpoint: 7674
├─ WebSocket: 7673
├─ Metrics: 7675

Testnet:
├─ P2P Network: 17676
├─ RPC Endpoint: 17674
├─ WebSocket: 17673
└─ Metrics: 17675
```

---

## 3. ADDRESS FORMAT & STANDARDS

| Component | Value | Example |
|-----------|-------|---------|
| **Address Prefix** | 'A' | A1x2CdEfGhIjKlMnOpQrStUvWxYz |
| **Address Length** | 34 chars (bech32) | — |
| **Script Address** | 'S' prefix | S2aBcDeFgHiJkLmNoPqRsTuVwXyZ |
| **Testnet Prefix** | 'TA' | TA1xXxXxXxXxXxXxXxXxXxXxXx |
| **Checksum** | SHA-256 double hash | Bip 173 (Bech32) |

---

## 4. CONSENSUS & MINING

### Phase 1: Hybrid PoW + Masternode (Year 1-4)
```
Block Distribution:
├─ 70% PoW Miners
├─ 25% Masternode Holders
└─ 5% Treasury/Governance
```

### Masternode Requirements
- **Collateral**: 10,000 ATMN (after year 1)
- **Uptime**: 95% required
- **Hardware**: 2GB RAM, 10GB SSD minimum
- **Annual ROI**: 15-20% (varies by network load)

### Mining Algorithm
- **PoW Algorithm**: PHI1612 (ASIC-resistant)
- **Difficulty**: Retargets every 20 minutes
- **Min Difficulty**: 1 ATMN value guarantee

### Mining Difficulty Adjustment
```
Target: nTargetSpacing = 12 seconds
Adjustment: max(prev_diff * 0.25, min(prev_diff * 4, new_target))
Min Diff: 0.00000001
Max Diff: nProofOfWorkLimit >> 20
```

---

## 5. EVM COMPATIBILITY LAYER

### Smart Contract Support
- **Language**: Solidity, Vyper
- **Standards**: ERC-20, ERC-721, ERC-1155, ERC-4626
- **Precompiles**: 
  - SHA-256 hashing
  - RIPEMD-160 hashing
  - Elliptic curve signature recovery
  - Modular exponentiation
  - BLAKEB2 compression

### Gas Model (EIP-1559 compatible)
```
Base Fee: Dynamic (starts at 1 Gwei)
Priority Fee: User-configurable
Gas Limit per Block: 10,000,000
Min Gas Price: 0.1 Gwei
```

### Smart Contract Deployment Cost
- **Deployment**: ~0.01 ATMN for simple contracts
- **Storage**: 0.0000001 ATMN per byte per year
- **Execution**: 0.00001 ATMN per 1000 gas

---

## 6. AI INTEGRATION LAYER

### On-Chain AI Module Features
```
AI Model Registry:
├─ Model Hash: SHA-256 of model weights
├─ Model Metadata: Name, version, creator
├─ Input Schema: Expected data format
├─ Output Schema: Predicted outputs
└─ Fee: % of execution cost to creator

Inference Execution:
├─ Input Data: User provides normalized inputs
├─ Verification: zkProof validates computation
├─ Cost: Paid in ATMN (20% to treasury)
└─ Output: On-chain result with proof

Supported Models:
├─ Price Prediction (ML models)
├─ Risk Assessment (LLM-based)
├─ Data Classification (CNN/RNN)
└─ Custom WASM modules
```

### AI Revenue Model
- **Execution Fee**: 0.001 ATMN per inference
- **Creator Royalty**: 50% of fees
- **Treasury**: 30% of fees
- **Network Validators**: 20% of fees

---

## 7. GOVERNANCE & DAO

### Governance Token
- **1 ATMN = 1 Vote** (staked)
- **Voting Power**: Locked for 7 days minimum
- **Proposal Threshold**: 10,000 ATMN collateral

### Proposal Types
1. **Parameter Changes**: Block rewards, fees, network params
2. **Upgrade Governance**: Code changes, hardforks
3. **Treasury Allocation**: Fund requests, grants
4. **Community Initiatives**: Special projects

### Voting Mechanics
- **Voting Period**: 7 days
- **Execution Delay**: 2 days after approval
- **Quorum**: 40% of staked ATMN
- **Approval**: 50% + 1 votes

### Treasury Management
- **Annual Budget**: 5% of block rewards (~26.5M ATMN in Year 1)
- **Allocation Categories**:
  - Development: 40%
  - Marketing: 25%
  - Infrastructure: 20%
  - Community Grants: 15%

---

## 8. CROSS-CHAIN BRIDGE

### Supported Chains
1. **Ethereum** - Primary bridge
2. **Polygon** - High-speed bridge
3. **Solana** - Ecosystem expansion
4. **Binance Smart Chain** - Liquidity bridge

### Bridge Mechanics
```
Wrapped ATMN (wATMN):
├─ ERC-20 on Ethereum
├─ BEP-20 on BSC
├─ SPL on Solana
└─ PRC-20 on Polygon

Bridge Fee: 0.1% per direction
Bridge Guard: 5 validators minimum
Confirmation Time: 2 blocks
```

### Liquidity Bootstrap
- **Uniswap**: $100K ATMN/ETH pool
- **PancakeSwap**: $100K ATMN/BNB pool
- **Raydium**: $100K ATMN/SOL pool

---

## 9. REAL-WORLD ASSET (RWA) LAYER

### Supported Asset Classes
1. **Real Estate**: Fractional property ownership
2. **Commodities**: Gold, oil, agricultural futures
3. **Bonds**: Corporate and government bonds
4. **Stocks**: Equity tokens
5. **Insurance**: Claims tokenization

### RWA Token Standard (ERC-3643 extended)
```
Features:
├─ Compliance Verification
├─ KYC/AML Integration
├─ Issuer Whitelisting
├─ Transfer Restrictions
├─ Dividend Distribution
└─ Regulatory Audit Trail
```

### RWA Revenue Model
- **Issuance Fee**: 0.5% of token value
- **Annual Management**: 0.1% of assets
- **Transaction Fee**: 0.05% per transfer

---

## 10. INFRASTRUCTURE & DEPLOYMENT

### Node Requirements
```
Full Node:
├─ CPU: 4 cores
├─ RAM: 8 GB
├─ Storage: 500 GB SSD
├─ Bandwidth: 100 Mbps
└─ Network: Always-on

Archive Node:
├─ CPU: 8 cores
├─ RAM: 16 GB
├─ Storage: 2 TB SSD
├─ Bandwidth: 1 Gbps
└─ Network: Dedicated

Mining Node:
├─ CPU: 16+ cores (GPU optional for AI)
├─ RAM: 32 GB
├─ Storage: 1 TB NVMe
├─ Bandwidth: 1 Gbps
└─ Network: Dedicated
```

### Server Deployment
```
Primary Nodes (antimony.carphatian.ro):
├─ Seed Node 1: 142.4.214.15 (New Jersey)
├─ Seed Node 2: 34.125.100.51 (Frankfurt)
├─ Public RPC: rpc.antimony.carphatian.ro
└─ WebSocket: ws.antimony.carphatian.ro

Explorer (explorer.carphatian.ro):
├─ Indexer: Real-time blockchain indexing
├─ API: GraphQL + REST endpoints
├─ Frontend: React SPA
└─ Database: PostgreSQL
```

---

## 11. WALLET SPECIFICATIONS

### Web Wallet (antimony.carphatian.ro/wallet)
- **Framework**: React + TypeScript
- **Security**: Hardware wallet support, 2FA
- **Features**:
  - Multi-signature support
  - Staking interface
  - Token swaps
  - NFT gallery
  - AI model marketplace
  - Cross-chain bridge UI
- **Browser Support**: Chrome, Firefox, Safari, Edge (latest)

### Windows Wallet
- **Framework**: Electron + React
- **Features**:
  - Solo mining
  - Masternode management
  - Cold storage
  - HD wallet support
  - Batch transactions
  - Custom RPC endpoints
- **System**: Windows 10/11, 4GB RAM minimum

### macOS Wallet
- **Framework**: Swift Native UI + Electron web view
- **Features**: Feature parity with Windows
- **System**: macOS 10.13+, 4GB RAM minimum
- **Code Signing**: Notarization ready

---

## 12. BLOCK EXPLORER

### Features
```
Core:
├─ Block Browser: Hash, height, miner, timestamp
├─ Transaction Details: Inputs, outputs, fees, gas
├─ Address Explorer: Balance, transaction history
├─ Smart Contracts: Source code, execution logs
├─ Tokens: ERC-20/721/1155 browsing
└─ AI Models: Inference history, model registry

Analytics:
├─ Network Stats: Hashrate, difficulty, TPS
├─ Address Distribution: Wealth distribution chart
├─ Transaction Flow: Visual network analysis
├─ Token Supply: Circulating vs locked
├─ Mining Pool Stats: Pool dominance, reward distribution
└─ AI Inference Stats: Model popularity, fee trends

Search:
├─ Full-text search (blocks, tx, addresses)
├─ Advanced filters
├─ Tag system (exchange, miner, etc.)
└─ Export to CSV/JSON
```

---

## 13. MINING POOL SYSTEM

### Pool Architecture
```
Mining Pool (pool.antimony.carphatian.ro):
├─ Stratum Protocol: TCP 5555, 5556, 5557
├─ Difficulty: Auto-adjust per miner (0.1 to 1000)
├─ Payment: Proportional reward system
├─ Payout: Daily minimum 0.1 ATMN
├─ Fee: 1% pool fee
└─ Share Difficulty: Dynamic adjustment

Pool Statistics:
├─ Hash Power: Total network hashrate
├─ Blocks Found: Last 24h
├─ Miners Online: Current connected
├─ Avg Difficulty: Network average
└─ Est. Block Time: Next block ETA
```

### Pool Payment Model
- **Payment Method**: Proportional (PROP)
- **Minimum Payout**: 0.1 ATMN
- **Payment Frequency**: Daily at UTC 00:00
- **Commission**: 1% of block rewards

---

## 14. SECURITY & AUDITS

### Security Measures
```
Consensus:
├─ 51% attack protection: Long-range reorg limit
├─ Sybil attack: Proof-of-Work barrier
├─ Eclipse attack: Random peer selection
└─ Selfish mining: Incentive structure

Smart Contracts:
├─ Formal verification: All critical contracts
├─ Audit: Professional security audit pre-launch
├─ Insurance: Protocol insurance pool
└─ Bug bounty: Up to 5% of fund

Network:
├─ TLS 1.3: All RPC connections
├─ Rate limiting: 1000 requests/minute per IP
├─ DDoS protection: Cloudflare integration
└─ Validator key rotation: Monthly
```

---

## 15. LAUNCH TIMELINE

```
Phase 1: Foundation (Jan-Feb 2025)
├─ Week 1-2: Core chain development
├─ Week 3-4: EVM integration
├─ Week 5-6: AI module integration
└─ Week 7-8: Testnet deployment

Phase 2: Infrastructure (Mar 2025)
├─ Week 1-2: Explorer development
├─ Week 2-3: Web wallet launch
├─ Week 3-4: Mining pool setup
└─ Week 4: Desktop wallets (Windows/macOS)

Phase 3: Testing (Apr 2025)
├─ Week 1-2: Security audit
├─ Week 2-3: Community testnet
├─ Week 3-4: Bug fixes & optimization
└─ Week 4: Bridge testing

Phase 4: Launch (May 2025)
├─ Week 1: Mainnet genesis
├─ Week 2: Mining activation
├─ Week 3: Exchange listings
└─ Week 4: Marketing campaign
```

---

## 16. KEY METRICS & TARGETS

| Metric | Year 1 | Year 2 | Year 3 |
|--------|--------|--------|--------|
| **Active Users** | 10K | 100K | 1M |
| **Daily Transactions** | 1K | 100K | 1M |
| **Network Hashrate** | 1 TH/s | 100 TH/s | 1 PH/s |
| **Masternodes** | 100 | 1,000 | 10,000 |
| **TVL in DeFi** | $1M | $10M | $100M |
| **Developer Ecosystem** | 10 dApps | 100 dApps | 1,000 dApps |
| **Token Price Target** | $0.10 | $0.50 | $2.00 |

---

## 17. COMPLIANCE & REGULATIONS

```
Jurisdictions:
├─ EU: MiCA compliance planned
├─ US: FinCEN MSB registration
├─ Asia: Jurisdiction-by-jurisdiction
└─ Global: ISO 27001 security standard

KYC/AML:
├─ Tier 1: Free (no limits)
├─ Tier 2: $100K daily (KYC required)
├─ Tier 3: Institutional (Full AML)
└─ Compliance Provider: Chainalysis integration
```

---

## Document Status
- **Version**: 1.0
- **Last Updated**: December 4, 2025
- **Next Review**: December 15, 2025
- **Author**: ATMN Development Team
