#!/bin/bash
# Test transaction batching in mempool

cd /home/ubuntu/atmn-2.0/atmn-core

echo "🧪 Testing Transaction Batching"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Create a shared mempool state (note: current implementation creates new mempool per invocation)
# For a real test, we'd need to modify the code to persist mempool or use a single process

echo "📝 Creating 5 test transactions..."
echo ""

# Transaction 1: Small fee (0.001 ATMN, 50 satoshis/byte if ~200 bytes)
echo "1️⃣  Transaction 1: 2 ATMN, fee=0.001 ATMN"
./target/release/submit-transaction \
    ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178 \
    ATMN_batch_test_1 \
    2.0 0.001

echo ""

# Transaction 2: Medium fee (0.005 ATMN, 250 satoshis/byte)
echo "2️⃣  Transaction 2: 3 ATMN, fee=0.005 ATMN"
./target/release/submit-transaction \
    ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178 \
    ATMN_batch_test_2 \
    3.0 0.005

echo ""

# Transaction 3: High fee (0.01 ATMN, 500 satoshis/byte) - should be prioritized
echo "3️⃣  Transaction 3: 1 ATMN, fee=0.01 ATMN (HIGH FEE)"
./target/release/submit-transaction \
    ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178 \
    ATMN_batch_test_3 \
    1.0 0.01

echo ""

# Transaction 4: Another medium fee
echo "4️⃣  Transaction 4: 5 ATMN, fee=0.005 ATMN"
./target/release/submit-transaction \
    ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178 \
    ATMN_batch_test_4 \
    5.0 0.005

echo ""

# Transaction 5: Low fee (0.0005 ATMN, 25 satoshis/byte)
echo "5️⃣  Transaction 5: 4 ATMN, fee=0.0005 ATMN (LOW FEE)"
./target/release/submit-transaction \
    ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178 \
    ATMN_batch_test_5 \
    4.0 0.0005

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📦 All transactions created!"
echo ""
echo "Note: Current implementation creates new mempool per invocation."
echo "For real batching test, we need persistent mempool or mining pool integration."
echo ""
echo "Expected priority order (by fee-per-byte):"
echo "  1. Transaction 3 (0.01 ATMN fee)"
echo "  2. Transaction 2 (0.005 ATMN fee)"
echo "  3. Transaction 4 (0.005 ATMN fee)"
echo "  4. Transaction 1 (0.001 ATMN fee)"
echo "  5. Transaction 5 (0.0005 ATMN fee)"
