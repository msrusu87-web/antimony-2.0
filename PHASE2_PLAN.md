# Phase 2: SHA-256d PoW Consensus Implementation

**Start Date:** December 4, 2025  
**Duration:** Weeks 2-3 (estimated 80-100 hours)  
**Goal:** Implement complete Proof-of-Work consensus engine with difficulty adjustment

---

## Objectives

### Primary Goals
1. ✅ Implement SHA-256d hashing (double SHA-256)
2. ✅ Create complete block validation logic
3. ✅ Implement difficulty adjustment algorithm
4. ✅ Build Proof-of-Work verification
5. ✅ Add 100+ integration tests
6. ✅ Achieve <2 second block validation time

### Success Criteria
- All consensus tests passing (50+ new tests)
- Block validation working for 1000+ blocks
- Difficulty adjustment verified mathematically
- Hash rate calculation accurate
- No compilation errors or warnings
- Performance benchmarks established

---

## Work Breakdown

### Task 1: SHA-256d Hashing Implementation (6-8 hours)

**File:** `atmn-core/src/consensus.rs` (expand from skeleton)

#### Implementation Steps
1. Create hash utility functions
2. Implement SHA-256d (double SHA-256)
3. Implement Merkle tree calculation
4. Add endianness conversion (little-endian)
5. Create hash verification function

#### Key Functions to Implement
```rust
pub fn sha256d(data: &[u8]) -> BlockHash
pub fn calculate_merkle_root(transactions: &[Transaction]) -> BlockHash
pub fn verify_hash_difficulty(hash: &BlockHash, target: u32) -> bool
pub fn bits_to_target(bits: u32) -> BlockHash
pub fn target_to_bits(target: &BlockHash) -> u32
```

#### Tests Needed
- ✅ SHA-256d produces correct hashes (known vectors)
- ✅ Double hashing different from single
- ✅ Merkle root calculation correctness
- ✅ Endianness conversion
- ✅ Hash difficulty verification
- ✅ Edge cases (empty input, max value)

#### Deliverable
- SHA-256d verified against Bitcoin test vectors
- Merkle tree calculation tested
- 10+ passing unit tests

---

### Task 2: Difficulty Adjustment Algorithm (8-10 hours)

**File:** `atmn-core/src/consensus.rs` (new module)

#### Algorithm Details
```
Target Timespan: 2 weeks (2,016 blocks)
Actual Timespan: (last_block_time - first_block_time)

New Difficulty = Old Difficulty * (Actual Timespan / Target Timespan)

Constraints:
- Minimum: 1x (don't decrease by more than 75%)
- Maximum: 4x (don't increase by more than 300%)
- No change: If 1x-4x range, multiply by ratio
```

#### Implementation Steps
1. Create difficulty calculation function
2. Implement timespan calculation
3. Add constraint enforcement (1x-4x limits)
4. Handle edge cases (block 0, testnet, regtest)
5. Create difficulty adjustment schedule

#### Key Functions
```rust
pub fn calculate_difficulty(
    prev_blocks: &[&Block],
    target_timespan: u32,
    block_time: u32,
) -> u32

pub fn get_next_difficulty(
    last_block: &Block,
    first_block_in_period: &Block,
    network: &ChainParams,
) -> u32

pub fn is_difficulty_adjustment_block(height: BlockHeight) -> bool
```

#### Tests Needed
- ✅ Difficulty increases when blocks too fast
- ✅ Difficulty decreases when blocks too slow
- ✅ Constraints enforced (1x-4x limits)
- ✅ Edge cases (genesis, small window)
- ✅ Convergence (approaches target)
- ✅ Stability (doesn't oscillate)
- ✅ Mathematical correctness (known test vectors)

#### Deliverable
- Difficulty adjustment verified against Bitcoin Core
- 15+ integration tests
- Performance benchmarks (<10ms per calculation)

---

### Task 3: Block Validation Logic (10-12 hours)

**File:** `atmn-core/src/block.rs` (expand from skeleton)

#### Validation Checks
```
Block Header Validation:
├─ Magic bytes correct
├─ Timestamp within acceptable range
├─ Difficulty matches chain params
├─ Nonce valid range (0 to u32::MAX)
├─ Previous block hash matches
└─ Merkle root correct

Block Body Validation:
├─ No duplicate transactions
├─ Coinbase transaction valid
├─ All transactions valid
├─ Block size within limits
├─ Transaction total size valid
└─ No UTXO conflicts

Proof-of-Work Validation:
├─ Block hash below target
├─ Difficulty matches adjustment schedule
└─ Hash rate consistent with network
```

#### Key Functions
```rust
pub fn validate_block_header(&self) -> Result<()>
pub fn validate_block_body(&self) -> Result<()>
pub fn validate_proof_of_work(&self, params: &ChainParams) -> Result<()>
pub fn validate_coinbase_transaction(&self) -> Result<()>
pub fn verify_transactions(&self) -> Result<()>
```

#### Tests Needed
- ✅ Valid block passes all checks
- ✅ Invalid hash rejected
- ✅ Timestamp too old/new rejected
- ✅ Merkle root mismatch detected
- ✅ Duplicate transactions rejected
- ✅ Invalid coinbase rejected
- ✅ Transaction validation
- ✅ Block size limits enforced
- ✅ 30+ validation tests total

#### Deliverable
- Complete block validation pipeline
- 30+ passing tests
- Performance: <1ms per block validation

---

### Task 4: Integration & Performance Testing (8-10 hours)

**File:** New `atmn-core/src/consensus_tests.rs`

#### Integration Tests
1. **Chain Validation Tests**
   - Generate 100 blocks sequentially
   - Validate entire chain
   - Check difficulty progression
   - Verify reward schedule

2. **Difficulty Progression Tests**
   - Simulate fast blocks → difficulty increase
   - Simulate slow blocks → difficulty decrease
   - Test multiple adjustment periods
   - Verify convergence

3. **Edge Case Tests**
   - Genesis block validation
   - Testnet parameters
   - Regtest parameters
   - Network forking

4. **Performance Tests**
   - Hash calculation time
   - Difficulty adjustment time
   - Block validation time
   - Chain validation throughput

#### Test Scenarios
```rust
#[test]
fn test_100_block_chain() { }

#[test]
fn test_difficulty_adjustment_accuracy() { }

#[test]
fn test_fast_blocks_increase_difficulty() { }

#[test]
fn test_slow_blocks_decrease_difficulty() { }

#[test]
fn test_mainnet_parameters() { }

#[test]
fn test_testnet_parameters() { }

#[test]
fn test_block_validation_performance() { }

#[test]
fn test_hash_rate_calculation() { }
```

#### Deliverable
- 50+ integration tests
- Performance benchmarks
- Chain validation verified for 1000+ blocks

---

### Task 5: Rosetta API Preparation (4-6 hours)

**File:** New `atmn-core/src/rosetta.rs`

#### Core Structures
```rust
pub struct RosettaBlock {
    pub block_identifier: BlockIdentifier,
    pub parent_block_identifier: BlockIdentifier,
    pub timestamp: Timestamp,
    pub transactions: Vec<RosettaTransaction>,
}

pub struct RosettaTransaction {
    pub transaction_identifier: String,
    pub operations: Vec<Operation>,
    pub metadata: TransactionMetadata,
}

pub struct Operation {
    pub operation_id: u32,
    pub type_: String,  // "TRANSFER", "MINT", "BURN"
    pub account: String,
    pub amount: Amount,
    pub status: String,
}
```

#### API Response Builders
```rust
impl Block {
    pub fn to_rosetta(&self) -> RosettaBlock
}

impl Transaction {
    pub fn to_rosetta(&self) -> RosettaTransaction
}

impl ChainParams {
    pub fn to_rosetta_network(&self) -> RosettaNetwork
}
```

#### Deliverable
- Rosetta conversion functions
- Block/transaction serialization
- 10+ conversion tests
- Ready for HTTP API layer

---

### Task 6: Documentation & Code Quality (6-8 hours)

#### Documentation to Create
1. **Consensus Algorithm Guide** - Explain SHA-256d and difficulty
2. **Block Validation Specification** - All validation rules
3. **Rosetta Integration Guide** - API implementation details
4. **Testing Guide** - How to run and write tests
5. **Performance Benchmarks** - Baseline metrics

#### Code Quality Tasks
1. Add comprehensive code comments
2. Create module-level documentation
3. Verify no clippy warnings
4. Check code coverage (target: 80%)
5. Add examples in tests

#### Deliverable
- 5+ documentation files
- Code coverage report
- Performance baseline established

---

## Architecture Overview

### Module Structure
```
atmn-core/
├── src/
│   ├── lib.rs (main exports)
│   ├── types.rs ✅ (existing)
│   ├── error.rs ✅ (existing)
│   ├── chain_params.rs ✅ (existing)
│   ├── block.rs (expand validation)
│   ├── transaction.rs (expand validation)
│   ├── consensus.rs (expand - hash + difficulty)
│   ├── storage.rs (existing - ready for DB)
│   ├── network.rs (existing - ready for P2P)
│   ├── rosetta.rs (new - API support)
│   └── consensus_tests.rs (new - 50+ tests)
└── Cargo.toml (existing)
```

### Data Flow
```
Input Block
    ↓
[Hash Calculation]
    ↓
[Difficulty Verification]
    ↓
[Header Validation]
    ↓
[Transaction Validation]
    ↓
[Coinbase Validation]
    ↓
Valid Block ✓
```

---

## Testing Strategy

### Unit Tests (Per Module)
- SHA-256d correctness: 5 tests
- Difficulty calculation: 10 tests
- Block validation: 15 tests
- Transaction validation: 10 tests
- Edge cases: 10 tests

**Total Unit Tests: 50+**

### Integration Tests
- Chain of 100 blocks: 5 test scenarios
- Difficulty progression: 5 scenarios
- Network parameters: 3 scenarios
- Performance benchmarks: 4 scenarios

**Total Integration Tests: 20+**

### Test Coverage Target
- Lines: 85%+
- Branches: 75%+
- Functions: 90%+

---

## Timeline (2 weeks)

### Week 2 (Days 1-5)
**Days 1-2:** SHA-256d Implementation
- Implement hash functions
- Verify against test vectors
- 10 tests passing

**Days 3-4:** Difficulty Adjustment
- Implement algorithm
- Add constraints and edge cases
- 15 tests passing

**Day 5:** Block Validation Part 1
- Implement header validation
- Begin body validation
- 10 tests passing

### Week 3 (Days 6-10)
**Days 6-7:** Block Validation Part 2
- Complete body validation
- Add coinbase validation
- 30 tests total passing

**Days 8-9:** Integration Testing
- Chain validation
- Performance testing
- 50+ tests total

**Day 10:** Rosetta + Documentation
- Rosetta API structures
- Documentation
- Code cleanup

---

## Deliverables

### Code
- ✅ SHA-256d implementation with tests
- ✅ Difficulty adjustment algorithm with tests
- ✅ Complete block validation pipeline
- ✅ Rosetta API conversion functions
- ✅ 100+ passing tests

### Documentation
- ✅ Algorithm documentation
- ✅ Validation specification
- ✅ Rosetta integration guide
- ✅ Performance benchmarks
- ✅ Code comments

### Git
- ✅ 5-10 feature commits
- ✅ Commit messages with context
- ✅ All pushed to GitHub

### Verification
- ✅ All tests passing
- ✅ No compilation warnings
- ✅ Code coverage 80%+
- ✅ Performance benchmarks met
- ✅ Ready for Phase 3

---

## Success Metrics

| Metric | Target | Status |
|--------|--------|--------|
| Tests Passing | 100+ | TBD |
| Code Coverage | 85%+ | TBD |
| Hash Speed | <1ms | TBD |
| Block Validation | <2ms | TBD |
| Difficulty Calc | <10ms | TBD |
| Build Time | <2 min | TBD |

---

## Risk Mitigation

### Technical Risks
- **Risk:** SHA-256d mismatch with Bitcoin
  - **Mitigation:** Use Bitcoin test vectors, compare output byte-by-byte

- **Risk:** Difficulty adjustment oscillation
  - **Mitigation:** Implement constraints, test multiple scenarios

- **Risk:** Block validation too slow
  - **Mitigation:** Profile code, optimize hot paths, use parallelization

### Schedule Risks
- **Risk:** Underestimated complexity
  - **Mitigation:** Daily progress reviews, adjust scope if needed

- **Risk:** Integration issues
  - **Mitigation:** Comprehensive integration tests, gradual integration

---

## Next Phase (Phase 3)

Once Phase 2 completes:
1. **Storage Layer** - RocksDB integration (UTXO management)
2. **Network P2P** - Node networking and block propagation
3. **Mining System** - Block creation and nonce search

---

## Resources

### Reference Materials
- Bitcoin Core: https://github.com/bitcoin/bitcoin
- Difficulty Adjustment: BIP66, BIP112
- SHA-256: FIPS 180-4
- Merkle Trees: Bitcoin Wiki

### Tools
- `cargo test` - Run test suite
- `cargo bench` - Performance testing
- `cargo flamegraph` - Profiling
- `cargo clippy` - Linting

### Team Knowledge Base
- Phase 1 Status: PHASE1_STATUS.md
- Pure PoW Details: SIMPLIFICATION_UPDATE.md
- Specifications: SPECIFICATIONS.md (sections 2-4)

---

**Ready to begin Phase 2 SHA-256d implementation?**

Next step: Start with SHA-256d hashing in `consensus.rs`
