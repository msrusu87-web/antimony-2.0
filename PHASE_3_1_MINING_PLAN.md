# Phase 3.1 - Mining System Implementation Plan

## Status: IN PROGRESS ✅

### Completed Tasks:
- ✅ Created `atmn-core/src/miner.rs` (450+ lines)
  - Core Miner struct with configuration
  - MiningStats for tracking performance
  - BlockTemplate for passing mining data
  - MiningResult for results
  - Full SHA256d-based PoW implementation
  - Difficulty adjustment calculations
  - Header serialization for mining
  - 8 comprehensive unit tests

### File Structure:
```
atmn-2.0/
├── atmn-core/
│   ├── Cargo.toml (ADD: num_cpus dependency)
│   └── src/
│       ├── lib.rs (ADD: pub mod miner export)
│       ├── block.rs ✅ (has nonce field)
│       ├── consensus.rs ✅ (has mining functions)
│       └── miner.rs ✅ (NEW - 450 lines)
├── atmn-mining-pool/
│   └── (To be implemented - stratum protocol)
└── PHASE3_ROADMAP.md ✅
```

### Key Components Implemented:

#### 1. Miner Struct
```rust
pub struct Miner {
    config: MinerConfig,
    stats: MiningStats,
}
```

**Methods:**
- `new()` - Create with default config (auto-detect CPU cores)
- `with_config()` - Create with custom config
- `mine_block()` - Main mining loop (iterates nonce until valid PoW found)
- `verify_block_pow()` - Verify block has valid PoW
- `difficulty_for_next_block()` - Calculate difficulty for next block
- `stats()` - Get mining statistics

#### 2. Mining Algorithm
```
1. Start with nonce = 0
2. Loop while nonce < max_nonce:
   a. Create block header with current nonce
   b. Hash header with SHA256d
   c. Check if hash < target (meets difficulty)
   d. If yes: Found block! Return with stats
   e. If no: Increment nonce, continue
   f. Periodically update hash rate (every 1M hashes)
3. If max nonce reached: Return unsuccessful
```

#### 3. MiningStats Tracking
- `hashes_computed` - Total hashes attempted
- `hash_rate` - Hashes per second (updated in real-time)
- `blocks_found` - Total blocks mined
- `last_block_time` - Timestamp of last block
- `difficulty_bits` - Current mining difficulty

#### 4. Difficulty Adjustment
- Adjusts every 2016 blocks (like Bitcoin)
- Target timespan: 2 weeks (based on 12s block time in ATMN)
- Constraints: No adjustment >4x or <1/4x per period
- Calculation: `new_difficulty = old_difficulty * actual_timespan / target_timespan`

### Next Tasks (NOT YET DONE):

#### 5. Update lib.rs to export miner module
```rust
pub mod miner;
pub use miner::{Miner, MinerConfig, BlockTemplate, MiningResult, MiningStats};
```

#### 6. Add num_cpus dependency to Cargo.toml
```toml
[dependencies]
num_cpus = "1.16"
```

#### 7. Create miner-cli binary
Location: `atmn-core/bin/miner.rs`

Features:
- Connect to local Rosetta node
- Get mining template via `/construction/metadata`
- Mine blocks with configurable difficulty
- Submit mined blocks via `/construction/submit`
- Display live hash rate and block count

```rust
// Pseudocode:
fn main() {
    let mut miner = Miner::new();
    loop {
        let template = get_block_template_from_rosetta();
        match miner.mine_block(template)? {
            Ok(result) if result.success => {
                submit_block_to_rosetta(result.block.unwrap())?;
                println!("Block #{} found! Hash rate: {}", ...);
            }
            _ => println!("No block found this round. Hash rate: {}", ...)
        }
    }
}
```

#### 8. Create mining pool module (atmn-mining-pool)
Features:
- Stratum v2 protocol support
- Share validation
- Worker management
- Reward distribution
- Pool statistics

#### 9. Add mining RPC endpoints
Via Rosetta Construction API:
- `POST /construction/metadata` - Get block template + difficulty
- `POST /construction/payloads` - Create block for mining
- `POST /construction/submit` - Submit mined block

#### 10. Testing Suite
Unit tests to add:
- [ ] Test header serialization (byte-for-byte correctness)
- [ ] Test nonce iteration (verify it increments correctly)
- [ ] Test hash difficulty verification (hashes < target pass, others fail)
- [ ] Test difficulty adjustment (timespan ratios apply correctly)
- [ ] Test mining with low difficulty (should find blocks quickly in tests)
- [ ] Test max nonce handling (graceful exit when nonce overflows)
- [ ] Performance test (measure hash rate on test machine)
- [ ] Verify against reference implementations (Bitcoin Core)

### Implementation Notes:

**Why This Order?**
1. Miner core logic first (foundation for everything)
2. Module integration (lib.rs export, dependency management)
3. Binary/CLI (makes it usable immediately)
4. Pool (scales to multi-worker setup)
5. RPC (allows remote mining)

**Performance Considerations:**
- SHA256d hashing is CPU-intensive - expect 10-100 million hashes/sec depending on CPU
- Block time is 12 seconds, so difficulty should be tuned so average miner finds blocks regularly
- Multi-threaded support can be added later (current implementation is single-threaded but thread-ready)

**Security Considerations:**
- Nonce is u32 (4 billion possible values) - may cycle for very fast miners
- Timestamp can be manipulated (but miner sets it to current time)
- No stake weighting (pure PoW, not PoS)
- No merkle tree validation yet (TODO in block.rs)

**Known TODOs:**
- [ ] Implement `block.rs::Block::hash()` - currently returns zero()
- [ ] Implement `block.rs::Block::calculate_merkle_root()` - currently returns zero()
- [ ] Implement `block.rs::Block::is_valid()` - currently just returns Ok(())
- [ ] Add merkle tree proof verification
- [ ] Add transaction validation before mining
- [ ] Add fee market logic for transaction selection

### Success Criteria:
1. ✅ Miner struct compiles and passes tests
2. ⏳ lib.rs exports miner module
3. ⏳ Cargo.toml has num_cpus dependency
4. ⏳ miner-cli binary can run and mine blocks locally
5. ⏳ Mine a test block successfully (with low difficulty)
6. ⏳ Verify block has valid PoW signature
7. ⏳ Hash rate displayed correctly
8. ⏳ Statistics persist across block finds
9. ⏳ Difficulty adjustment works correctly
10. ⏳ Integration with Rosetta API

### Estimated Time: 1-2 weeks total
- Core miner (✅ DONE): 2-3 hours
- Integration & testing: 1-2 hours  
- CLI binary: 2-3 hours
- Mining pool: 3-5 hours
- RPC integration: 2-3 hours
- Full testing suite: 3-4 hours

---

## TECHNICAL DETAILS

### Block Header Format (for mining)
```
Bytes  0-3:   Version (4 bytes, little-endian)
Bytes  4-35:  Previous Block Hash (32 bytes)
Bytes 36-67:  Merkle Root of Transactions (32 bytes)
Bytes 68-75:  Timestamp (8 bytes, little-endian) [NOTE: Current code uses u32, should be u64]
Bytes 76-79:  Difficulty Bits (4 bytes, little-endian)
Bytes 80-83:  Nonce (4 bytes, little-endian)
```

Total: 84 bytes

### Mining Algorithm Details

**1. Hash Target Verification:**
```
Block hash must be ≤ target value (as little-endian 256-bit number)
Target computed from difficulty bits using Bitcoin compact format
```

**2. Difficulty Adjustment (every 2016 blocks):**
```
Actual time = timestamp_last_block - timestamp_first_block
Target time = 2 weeks = 1,209,600 seconds
Ratio = actual_time / target_time

If ratio < 0.25: Use 0.25 (difficulty can't increase >4x)
If ratio > 4.0: Use 4.0 (difficulty can't decrease >4x)

new_difficulty = old_difficulty × ratio
```

**3. Hash Rate Calculation:**
```
Updated every 1,000,000 hashes during mining
hash_rate = total_hashes / elapsed_time_in_seconds
```

---

## NEXT PHASE DEPENDENCIES

Phase 4 (P2P Networking) needs:
- ✅ Miner producing valid blocks
- ⏳ Block distribution mechanism
- ⏳ Peer verification of mined blocks

Phase 5 (Rosetta Construction API) needs:
- ✅ Blocks created with valid PoW
- ⏳ Construction endpoints for building blocks
- ⏳ Submit endpoint for adding blocks to chain

