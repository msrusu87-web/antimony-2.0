# Quick Start Guide - ATMN 2.0 Development

## Environment Setup (First Time Only)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# Install system dependencies
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev librocksdb-dev

# Clone and navigate
git clone https://github.com/msrusu87-web/antimony-2.0.git
cd atmn-2.0
```

## Quick Status Check

```bash
# Build the project
cd atmn-core
cargo build

# Run tests
cargo test

# Run specific module tests
cargo test chain_params::
cargo test consensus::
```

## Current Work Status

**Last Completed:** Foundation modules (types, errors, chain_params, block, tx, consensus, storage, network)
- ✅ All 13 unit tests passing
- ✅ Compiling without errors
- ✅ Ready for Phase 1 detailed implementation

**Next Priority:** Implement consensus engine (`src/consensus.rs`)

## Key Files to Know

```
atmn-core/
├── Cargo.toml              # Dependencies and project config
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── types.rs            # Core data types (BlockHash, TxHash, etc)
│   ├── error.rs            # Error handling (13 error types)
│   ├── chain_params.rs     # ATMN 2.0 chain parameters (500M supply, etc)
│   ├── block.rs            # Block structure (skeleton, 70% done)
│   ├── transaction.rs      # Transaction types (skeleton, 70% done)
│   ├── consensus.rs        # PoW consensus (skeleton, 30% done)
│   ├── storage.rs          # Database abstraction (skeleton, 30% done)
│   └── network.rs          # P2P networking (skeleton, 30% done)
└── target/                 # Build output (ignored)
```

## Key Constants & Parameters

```rust
// From chain_params.rs
total_supply: 500_000_000 ATMN
block_time: 12 seconds
network_id: 7676 (mainnet), 17676 (testnet)

// Block rewards
Year 1: 50 ATMN/block
Year 2: 25 ATMN/block
Year 3+: 12.5 ATMN/block

// Masternode requirements
Year 1-2: 100,000 ATMN
Year 3+: 10,000 ATMN
```

## Important Build Commands

```bash
# Build (development, fast)
cargo build

# Build release (optimized)
cargo build --release

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Clean build artifacts
cargo clean
```

## Test Structure

All tests are in the respective module files:
- `src/types.rs::tests::*`
- `src/error.rs::tests::*`
- `src/chain_params.rs::tests::*`
- etc.

**Run all tests:** `cargo test`
**Run module tests:** `cargo test module_name::`

## Git Workflow

```bash
# Check status
git status

# Add changes
git add .

# Commit with conventional commits format
git commit -m "feat: description" # New feature
git commit -m "fix: description"  # Bug fix
git commit -m "docs: description" # Documentation
git commit -m "refactor: description"  # Code reorganization

# Push to GitHub
git push origin main
```

## Next Implementation Tasks (Priority Order)

1. **Consensus Engine** (Week 2)
   - File: `src/consensus.rs`
   - Implement: `verify_block()`, `calculate_difficulty()`, masternode validation
   - Tests needed: 8-10 new tests

2. **Block Validation** (Week 2)
   - File: `src/block.rs`
   - Implement: `hash()`, `calculate_merkle_root()`, `is_valid()`
   - Tests needed: 8-10 new tests

3. **Transaction Processing** (Week 3)
   - File: `src/transaction.rs`
   - Implement: signature verification, UTXO validation, serialization
   - Tests needed: 10-12 new tests

4. **Storage Layer** (Week 3)
   - Expand: `src/storage.rs` with RocksDB
   - New: `src/utxo_set.rs` for UTXO management
   - Tests needed: 8-10 new tests

5. **Network P2P** (Week 4)
   - Expand: `src/network.rs` with full protocol
   - Add: peer discovery, block/tx propagation
   - Tests needed: 8-10 integration tests

## Useful Resources

- **ATMN Specifications:** `./SPECIFICATIONS.md` (550+ lines)
- **Phase 1 Status:** `./PHASE1_STATUS.md` (comprehensive progress tracking)
- **Project Structure:** `./PROJECT_INITIALIZATION.md` (11 modules overview)
- **Contributing Guide:** `./CONTRIBUTING.md` (coding standards)
- **Rust Docs:** https://doc.rust-lang.org/book/
- **Tokio Async:** https://tokio.rs/

## GitHub Repository

**URL:** https://github.com/msrusu87-web/antimony-2.0
**Branch:** main
**Access:** msrusu87-web GitHub account

## Troubleshooting

**Compiler errors about unused variables:**
- Normal during skeleton implementation
- Use `cargo clippy` to see all warnings
- Fix when consolidating code

**Cargo build takes too long:**
- First build downloads all dependencies (~1-2 minutes)
- Incremental builds are fast (~0.5 seconds)
- Use `cargo check` for quick syntax checking

**Cannot compile RocksDB:**
- rocksdb is currently disabled in Cargo.toml
- Will enable after complete C dev tools setup
- Currently using stub storage implementation

**Git push fails:**
- Check SSH key is set up: `ssh -T git@github.com`
- Or use HTTPS with GitHub token
- Verify remote: `git remote -v`

## Development Notes

- **Type Safety:** Leveraging Rust's type system - types.rs defines all core types
- **Async-First:** Using Tokio for non-blocking operations
- **Test-Driven:** Write tests before implementation
- **Modular:** Each module is independent and testable
- **Error Handling:** Using thiserror for comprehensive error types

## Performance Targets

Once implementation complete:
- Block time: 12 seconds
- Transactions/second: 10-15 TPS (expandable with sharding)
- Block size: 1-2 MB average
- Consensus: PoW + Masternode validation

---

**Created:** January 2025  
**For:** ATMN 2.0 Phase 1 Development  
**Status:** Active Development
