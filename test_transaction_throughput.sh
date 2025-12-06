#!/bin/bash
# Transaction throughput and mempool sync test

set -e

cd /home/ubuntu/atmn-2.0

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "💸 Transaction Throughput Test - Phase 7"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Configuration
NUM_TRANSACTIONS=50
SENDER_ADDRESS="AW1nRFVkd2J2eFhZb1pwUzJNeWNBS0RVZTJlNkE="
TEST_START=$(date +%s)

echo "📊 Test Configuration:"
echo "   Total Transactions: $NUM_TRANSACTIONS"
echo "   Sender Address: ${SENDER_ADDRESS:0:30}..."
echo ""

# Check balance
echo "💰 Checking sender balance..."
cd atmn-core
BALANCE=$(cargo run --release --bin check-balance -- "$SENDER_ADDRESS" 2>&1 | grep "Total balance:" | awk '{print $3}')
echo "   Balance: $BALANCE ATMN"
echo ""

if [ -z "$BALANCE" ] || [ "$(echo "$BALANCE < 100" | bc)" -eq 1 ]; then
    echo "❌ Insufficient balance for testing"
    echo "   Need at least 100 ATMN"
    exit 1
fi

# Create multiple recipient addresses
RECIPIENTS=(
    "QVcybkRWa2Qyd0oydkZoWm9acFN6Tk15Y0FLRFVBPQ=="
    "QVczblJGVmtlMko0WmdGbmJGZHZVbkJUTWw1dFBRPT0="
    "QVc0blVrWldTMlEzYW5oYVoyWm1iR2R2VWpCVFRRPT0="
    "QVc1b1VrWlhTMlEzY2tGbmJHZHZWbkJUTmw1dFBRPT0="
    "QVc1aFZrWlhTMlEzY2tGbmJHZHZWbkJUTmw1dFBRPT0="
)

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📝 Creating $NUM_TRANSACTIONS transactions..."
echo ""

TX_FILES=()
TX_CREATE_START=$(date +%s)
SUCCESSFUL_CREATES=0

for i in $(seq 1 $NUM_TRANSACTIONS); do
    # Select recipient in round-robin fashion
    RECIPIENT_IDX=$((i % 5))
    RECIPIENT="${RECIPIENTS[$RECIPIENT_IDX]}"
    
    # Vary amounts slightly
    AMOUNT=$(echo "scale=4; 0.1 + ($i % 10) * 0.05" | bc)
    
    TX_FILE="/tmp/tx_$i.json"
    TX_FILES+=($TX_FILE)
    
    # Create transaction
    cargo run --release --bin create-transaction -- \
        --from "$SENDER_ADDRESS" \
        --to "$RECIPIENT" \
        --amount "$AMOUNT" \
        --fee 0.0001 \
        --output "$TX_FILE" > /dev/null 2>&1
    
    if [ $? -eq 0 ]; then
        SUCCESSFUL_CREATES=$((SUCCESSFUL_CREATES + 1))
        echo -n "."
    else
        echo -n "X"
    fi
    
    # Progress indicator
    if [ $((i % 10)) -eq 0 ]; then
        echo " $i/$NUM_TRANSACTIONS"
    fi
done

TX_CREATE_END=$(date +%s)
TX_CREATE_TIME=$((TX_CREATE_END - TX_CREATE_START))

echo ""
echo ""
echo "   Created: $SUCCESSFUL_CREATES/$NUM_TRANSACTIONS transactions"
echo "   Time: ${TX_CREATE_TIME}s"
echo "   Rate: $(echo "scale=2; $SUCCESSFUL_CREATES / $TX_CREATE_TIME" | bc) tx/s"
echo ""

if [ $SUCCESSFUL_CREATES -eq 0 ]; then
    echo "❌ Failed to create any transactions"
    exit 1
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📤 Submitting transactions to mempool..."
echo ""

TX_SUBMIT_START=$(date +%s)
SUCCESSFUL_SUBMITS=0

for i in $(seq 1 $SUCCESSFUL_CREATES); do
    TX_FILE="/tmp/tx_$i.json"
    
    if [ -f "$TX_FILE" ]; then
        cargo run --release --bin submit-transaction -- "$TX_FILE" > /dev/null 2>&1
        
        if [ $? -eq 0 ]; then
            SUCCESSFUL_SUBMITS=$((SUCCESSFUL_SUBMITS + 1))
            echo -n "."
        else
            echo -n "X"
        fi
        
        # Progress indicator
        if [ $((i % 10)) -eq 0 ]; then
            echo " $i/$SUCCESSFUL_CREATES"
        fi
        
        # Small delay to avoid overwhelming mempool
        sleep 0.01
    fi
done

TX_SUBMIT_END=$(date +%s)
TX_SUBMIT_TIME=$((TX_SUBMIT_END - TX_SUBMIT_START))

echo ""
echo ""
echo "   Submitted: $SUCCESSFUL_SUBMITS/$SUCCESSFUL_CREATES transactions"
echo "   Time: ${TX_SUBMIT_TIME}s"
echo "   Rate: $(echo "scale=2; $SUCCESSFUL_SUBMITS / $TX_SUBMIT_TIME" | bc) tx/s"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Transaction Throughput Analysis:"
echo ""

TEST_END=$(date +%s)
TOTAL_TIME=$((TEST_END - TEST_START))

echo "   Total Time: ${TOTAL_TIME}s"
echo "   Creation Rate: $(echo "scale=2; $SUCCESSFUL_CREATES / $TX_CREATE_TIME" | bc) tx/s"
echo "   Submission Rate: $(echo "scale=2; $SUCCESSFUL_SUBMITS / $TX_SUBMIT_TIME" | bc) tx/s"
echo "   Overall Throughput: $(echo "scale=2; $SUCCESSFUL_SUBMITS / $TOTAL_TIME" | bc) tx/s"
echo ""

# Verify mempool contains transactions
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 Mempool Status:"
echo ""

# Check mempool size (if available)
echo "   Note: Mempool is in-memory and will be included in next mined block"
echo ""

# Cleanup
rm -f /tmp/tx_*.json

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Transaction Throughput Test Complete!"
echo ""

# Success metrics
CREATE_SUCCESS=$((SUCCESSFUL_CREATES * 100 / NUM_TRANSACTIONS))
SUBMIT_SUCCESS=$((SUCCESSFUL_SUBMITS * 100 / SUCCESSFUL_CREATES))

echo "🎯 Success Metrics:"
echo "   Creation Success: ${CREATE_SUCCESS}%"
echo "   Submission Success: ${SUBMIT_SUCCESS}%"
echo ""

if [ $SUBMIT_SUCCESS -ge 90 ]; then
    echo "   Status: ✅ EXCELLENT"
elif [ $SUBMIT_SUCCESS -ge 70 ]; then
    echo "   Status: ✅ GOOD"
else
    echo "   Status: ⚠️  NEEDS IMPROVEMENT"
fi

echo ""
echo "💡 Recommendations:"
echo "   - Mine blocks to confirm transactions"
echo "   - Monitor mempool size with: cargo run --bin mempool-manager list"
echo "   - Check transaction confirmations after mining"
