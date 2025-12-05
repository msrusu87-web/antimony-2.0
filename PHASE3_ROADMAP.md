# ANTIMONY COIN - PHASE 3+ DEVELOPMENT ROADMAP

## Current Status
- ✅ Phase 2a-2d: Core blockchain (consensus, difficulty adjustment, genesis)
- ✅ Phase 2e: Rosetta API endpoints (network/status, block, account/balance)
- ✅ Explorer: Web UI with search, block details, address/tx framework

---

## Phase 3 - MINING & POW SYSTEM (3-4 weeks)

### 3.1 Mining System Implementation
**Goal**: Implement full Proof-of-Work mining with nonce iteration

**Tasks**:
1. **Mining Algorithm**
   - Implement block mining with nonce iteration
   - Hash difficulty verification
   - Target adjustment
   - Merkle tree for transactions

2. **Miner Component**
   ```rust
   // atmn-core/src/miner.rs - NEW
   pub struct Miner {
       pub mempool: Vec<Transaction>,
       pub difficulty_target: [u8; 32],
       pub max_nonce: u32,
   }
   
   impl Miner {
       pub fn mine_block(prev_block: &Block) -> Result<Block>;
       pub fn verify_work(block: &Block) -> Result<()>;
   }
   ```

3. **Transaction Mempool**
   - Add transaction validation
   - Priority queue for fees
   - Remove spent transactions on block

4. **Block Production**
   - Coinbase transaction generation
   - Block reward calculation
   - Timestamp validation

**Timeline**: 1 week
**Tests**: 15+ unit tests for mining validation

---

### 3.2 Mining Pool Support
**Goal**: Enable mining pool operations

**Features**:
1. Pool protocol (Stratum v2)
2. Share validation
3. Difficulty adjustment per worker
4. Reward distribution

**Timeline**: 1 week

---

## Phase 4 - P2P NETWORKING (3-4 weeks)

### 4.1 Full P2P Implementation
**Goal**: Decentralized peer-to-peer network

**Components**:
1. **Network Protocol**
   - Message types (inv, getblocks, tx, block)
   - Peer discovery
   - Peer management

2. **Node Implementation**
   ```rust
   // atmn-core/src/node.rs - EXPAND
   pub struct Node {
       pub peers: Vec<PeerConnection>,
       pub blockchain: Blockchain,
       pub mempool: Mempool,
   }
   
   impl Node {
       pub async fn broadcast_block(block: Block);
       pub async fn broadcast_transaction(tx: Transaction);
       pub async fn sync_chain();
   }
   ```

3. **Connection Management**
   - TCP connections
   - Connection pooling
   - Peer reputation system

4. **Message Handling**
   - Block propagation
   - Transaction relay
   - Chain synchronization

**Timeline**: 1.5 weeks

---

### 4.2 Chain Synchronization
**Goal**: Allow nodes to sync state

**Features**:
1. Full sync (from genesis)
2. Fast sync (checkpoint-based)
3. Lite sync (headers only)
4. Fork resolution

**Timeline**: 1 week

---

## Phase 5 - ROSETTA CONSTRUCTION API (2-3 weeks)

### 5.1 Construction API Endpoints
**Goal**: Full Rosetta construction for offline transaction building

**Endpoints**:
```
POST /construction/preprocess      - Validate inputs
POST /construction/metadata        - Get metadata/fees
POST /construction/payloads        - Create transaction
POST /construction/parse           - Parse transaction
POST /construction/combine         - Combine signatures
POST /construction/hash            - Hash transaction
POST /construction/submit          - Submit to network
```

**Implementation**:
1. Transaction builder
2. Fee estimation
3. UTXO management
4. Multi-sig support

**Timeline**: 1.5 weeks
**Tests**: 20+ endpoint tests

---

### 5.2 Advanced Features
**Goal**: Support complex transaction scenarios

**Features**:
1. Multi-input/output transactions
2. Fee market logic
3. Transaction prioritization
4. Batch operations

**Timeline**: 1 week

---

## Phase 6 - EVM COMPATIBILITY (4-6 weeks)

### 6.1 EVM Layer Integration
**Goal**: Enable smart contracts on ATMN

**Components**:
1. **EVM Execution**
   - Integrate Ethererum VM
   - Gas metering
   - State management
   - Contract storage

2. **Smart Contracts**
   - Solidity compilation
   - Contract deployment
   - Contract interactions

3. **Storage**
   - Account state trees
   - Contract code storage
   - Storage proofs

**Timeline**: 3 weeks

---

### 6.2 Cross-Chain Bridges (Phase 5+)
**Goal**: Enable asset transfers between chains

**Features**:
1. Bridge contracts
2. Validator consensus
3. Wrapped tokens
4. Liquidity pools

**Timeline**: 2-3 weeks

---

## Phase 7 - AI INFERENCE MODULE (Phase 5+)

### 7.1 AI Integration
**Goal**: Enable on-chain AI inference

**Components**:
1. Model execution
2. ZK proof for results
3. Oracle integration
4. Computation fees

---

## PRIORITY RANKING

**HIGH PRIORITY** (Do Soon):
1. Mining System (Phase 3.1)
2. P2P Networking (Phase 4.1)
3. Construction API (Phase 5.1)

**MEDIUM PRIORITY**:
1. Mining Pool (Phase 3.2)
2. Chain Sync (Phase 4.2)
3. Advanced Construction (Phase 5.2)

**LOWER PRIORITY**:
1. EVM (Phase 6) - Complex, 4-6 weeks
2. Bridges (Phase 6.2) - Depends on EVM
3. AI Module (Phase 7) - Research phase

---

## TECHNICAL DEPENDENCIES

```
Mining System (Phase 3.1)
    ↓
P2P Networking (Phase 4.1)
    ↓
Chain Sync (Phase 4.2)
    ↓ (parallel)
┌─────────────────┬──────────────────┐
│ Construction    │  Mining Pool     │
│ API (Phase 5.1) │ (Phase 3.2)      │
└─────────────────┴──────────────────┘
    ↓
EVM (Phase 6.1)
    ↓
Bridges (Phase 6.2)
    ↓
AI Module (Phase 7)
```

---

## TESTING STRATEGY

Each phase requires:
- ✅ Unit tests (90%+ coverage)
- ✅ Integration tests
- ✅ Network tests (testnet)
- ✅ Performance benchmarks
- ✅ Security audits (for financial operations)

---

## ESTIMATED TIMELINE

- Phase 3 (Mining): 2-3 weeks
- Phase 4 (P2P): 2-3 weeks
- Phase 5 (Construction API): 1-2 weeks
- Phase 6 (EVM): 4-6 weeks
- Phase 7 (AI): 3+ weeks

**Total**: ~14-20 weeks for full stack

---

## WHICH PHASE FIRST?

**RECOMMENDATION**: Start with Phase 3.1 (Mining)
- Foundation for everything else
- Validates block production
- Critical for mainnet launch
- Already has consensus/difficulty logic
- Estimated effort: 1-2 weeks

