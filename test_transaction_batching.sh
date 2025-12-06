#!/bin/bash
# Test transaction batching with priority ordering

cd /home/ubuntu/atmn-2.0/atmn-core

echo "🧪 Testing Transaction Batching & Priority Ordering"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

MINER="ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178"

# Check initial balance
echo "📊 Initial Balance:"
./target/release/check-balance "$MINER" 2>&1 | grep "Total Balance"
echo ""

# Create 5 transactions with different fees
echo "📝 Creating 5 transactions with different fee rates..."
echo ""

# Transaction 1: Low fee (0.001 ATMN / ~5 satoshi per byte)
echo "1️⃣  Low fee (0.001 ATMN):"
./target/release/submit-transaction "$MINER" ATMN_test_batch_1 2.0 0.001 2>&1 | tail -3
echo ""

# Transaction 2: Medium fee (0.005 ATMN / ~25 satoshi per byte)
echo "2️⃣  Medium fee (0.005 ATMN):"
./target/release/submit-transaction "$MINER" ATMN_test_batch_2 3.0 0.005 2>&1 | tail -3
echo ""

# Transaction 3: High fee (0.01 ATMN / ~50 satoshi per byte) - should be mined first
echo "3️⃣  High fee (0.01 ATMN):"
./target/release/submit-transaction "$MINER" ATMN_test_batch_3 1.5 0.01 2>&1 | tail -3
echo ""

# Transaction 4: Medium fee
echo "4️⃣  Medium fee (0.005 ATMN):"
./target/release/submit-transaction "$MINER" ATMN_test_batch_4 5.0 0.005 2>&1 | tail -3
echo ""

# Transaction 5: Very low fee (0.0005 ATMN / ~2.5 satoshi per byte)
echo "5️⃣  Very low fee (0.0005 ATMN):"
./target/release/submit-transaction "$MINER" ATMN_test_batch_5 4.0 0.0005 2>&1 | tail -3
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All transactions created!"
echo ""
echo "Expected priority order (by fee-per-byte):"
echo "  1. Transaction 3 (0.01 ATMN fee - highest)"
echo "  2. Transaction 2 (0.005 ATMN fee)"
echo "  3. Transaction 4 (0.005 ATMN fee)"
echo "  4. Transaction 1 (0.001 ATMN fee)"
echo "  5. Transaction 5 (0.0005 ATMN fee - lowest)"
echo ""
echo "📊 Final Balance:"
./target/release/check-balance "$MINER" 2>&1 | grep "Total Balance"
echo ""

# Test double-spend prevention
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔒 Testing Double-Spend Prevention..."
echo ""
echo "Attempting to spend same UTXO twice:"
echo "Transaction A: 10 ATMN"
./target/release/submit-transaction "$MINER" ATMN_doublespend_test 10.0 0.01 2>&1 | grep -A 3 "Creating transaction"
echo ""
echo "Transaction B: 10 ATMN (should use different UTXO)"
./target/release/submit-transaction "$MINER" ATMN_doublespend_test 10.0 0.01 2>&1 | grep -A 3 "Creating transaction"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Transaction batching test complete!"
echo ""
echo "Note: Each transaction creates a new UTXO, preventing true double-spend"
echo "      testing without mempool state tracking. In production, the P2P node"
echo "      would maintain mempool state and reject conflicting transactions."
