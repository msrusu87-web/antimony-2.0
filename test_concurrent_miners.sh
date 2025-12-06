#!/bin/bash
# Concurrent miner load test - Phase 7 completion

set -e

cd /home/ubuntu/atmn-2.0

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⛏️  Concurrent Miner Load Test - Phase 7"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Configuration
NUM_MINERS=5
BLOCKS_PER_MINER=10
TOTAL_BLOCKS=$((NUM_MINERS * BLOCKS_PER_MINER))
TEST_START=$(date +%s)

# Miner addresses (5 different addresses)
ADDRESSES=(
    "AW1nRFVkd2J2eFhZb1pwUzJNeWNBS0RVZTJlNkE="
    "QVcybkRWa2Qyd0oydkZoWm9acFN6Tk15Y0FLRFVBPQ=="
    "QVczblJGVmtlMko0WmdGbmJGZHZVbkJUTWw1dFBRPT0="
    "QVc0blVrWldTMlEzYW5oYVoyWm1iR2R2VWpCVFRRPT0="
    "QVc1b1VrWlhTMlEzY2tGbmJHZHZWbkJUTmw1dFBRPT0="
)

echo "📊 Test Configuration:"
echo "   Number of Miners: $NUM_MINERS"
echo "   Blocks per Miner: $BLOCKS_PER_MINER"
echo "   Total Target Blocks: $TOTAL_BLOCKS"
echo ""

# Check P2P network status
RUNNING_NODES=$(pgrep -f atmn-node | wc -l)
if [ "$RUNNING_NODES" -lt 10 ]; then
    echo "⚠️  Warning: Only $RUNNING_NODES P2P nodes running (expected 10)"
    echo "   Continuing anyway..."
fi
echo "   P2P Nodes Running: $RUNNING_NODES"
echo ""

# Get initial blockchain height
cd atmn-core
INITIAL_HEIGHT=$(cargo run --release --bin verify-blocks 2>&1 | grep "Best block height:" | awk '{print $4}')
echo "📦 Initial Blockchain Height: $INITIAL_HEIGHT"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 Starting $NUM_MINERS concurrent miners..."
echo ""

MINER_PIDS=()
MINER_LOGS=()

# Start miners in parallel
for i in $(seq 0 $((NUM_MINERS - 1))); do
    LOG_FILE="/tmp/miner_$i.log"
    MINER_LOGS+=($LOG_FILE)
    > $LOG_FILE
    
    TARGET_HEIGHT=$((INITIAL_HEIGHT + (i + 1) * BLOCKS_PER_MINER))
    MINER_ADDRESS="${ADDRESSES[$i]}"
    
    echo "   Miner $i: Mining to height $TARGET_HEIGHT..."
    echo "      Address: ${MINER_ADDRESS:0:20}..."
    echo "      Log: $LOG_FILE"
    
    # Start miner in background
    cargo run --release --bin mine-to-height -- \
        --target-height $TARGET_HEIGHT \
        --miner-address "$MINER_ADDRESS" \
        > $LOG_FILE 2>&1 &
    
    MINER_PIDS+=($!)
    echo "      PID: ${MINER_PIDS[$i]} ✅"
    echo ""
    
    # Stagger starts slightly
    sleep 0.5
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏳ Mining in progress..."
echo ""

# Monitor mining progress
LAST_HEIGHT=$INITIAL_HEIGHT
START_TIME=$(date +%s)
UPDATE_INTERVAL=5

while true; do
    sleep $UPDATE_INTERVAL
    
    # Check current height
    CURRENT_HEIGHT=$(cargo run --release --bin verify-blocks 2>&1 | grep "Best block height:" | awk '{print $4}' || echo "$LAST_HEIGHT")
    
    if [ "$CURRENT_HEIGHT" != "$LAST_HEIGHT" ]; then
        ELAPSED=$(($(date +%s) - START_TIME))
        BLOCKS_MINED=$((CURRENT_HEIGHT - INITIAL_HEIGHT))
        BLOCKS_PER_SEC=$(echo "scale=2; $BLOCKS_MINED / $ELAPSED" | bc)
        PROGRESS=$((BLOCKS_MINED * 100 / TOTAL_BLOCKS))
        
        echo "   Height: $CURRENT_HEIGHT (+$BLOCKS_MINED) | Time: ${ELAPSED}s | Rate: ${BLOCKS_PER_SEC} blocks/s | Progress: ${PROGRESS}%"
        LAST_HEIGHT=$CURRENT_HEIGHT
    fi
    
    # Check if target reached
    if [ "$CURRENT_HEIGHT" -ge "$((INITIAL_HEIGHT + TOTAL_BLOCKS))" ]; then
        echo ""
        echo "✅ Target height reached!"
        break
    fi
    
    # Check if miners are still running
    ACTIVE_MINERS=0
    for pid in "${MINER_PIDS[@]}"; do
        if kill -0 $pid 2>/dev/null; then
            ACTIVE_MINERS=$((ACTIVE_MINERS + 1))
        fi
    done
    
    if [ $ACTIVE_MINERS -eq 0 ]; then
        echo ""
        echo "⚠️  All miners have stopped"
        break
    fi
    
    # Timeout after 5 minutes
    if [ $ELAPSED -gt 300 ]; then
        echo ""
        echo "⚠️  Timeout reached (5 minutes)"
        break
    fi
done

# Stop all miners
echo ""
echo "🛑 Stopping miners..."
for pid in "${MINER_PIDS[@]}"; do
    kill $pid 2>/dev/null || true
done
sleep 2

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Mining Results:"
echo ""

# Final height check
FINAL_HEIGHT=$(cargo run --release --bin verify-blocks 2>&1 | grep "Best block height:" | awk '{print $4}')
TOTAL_MINED=$((FINAL_HEIGHT - INITIAL_HEIGHT))
TEST_END=$(date +%s)
TOTAL_TIME=$((TEST_END - TEST_START))

echo "   Initial Height: $INITIAL_HEIGHT"
echo "   Final Height: $FINAL_HEIGHT"
echo "   Blocks Mined: $TOTAL_MINED"
echo "   Test Duration: ${TOTAL_TIME}s"
echo "   Average Rate: $(echo "scale=3; $TOTAL_MINED / $TOTAL_TIME" | bc) blocks/s"
echo ""

# Analyze miner performance
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⛏️  Individual Miner Performance:"
echo ""

for i in $(seq 0 $((NUM_MINERS - 1))); do
    LOG_FILE="${MINER_LOGS[$i]}"
    
    if [ -f "$LOG_FILE" ]; then
        BLOCKS_FOUND=$(grep -c "Block mined" "$LOG_FILE" 2>/dev/null || echo "0")
        HASH_RATE=$(grep "Hash rate" "$LOG_FILE" 2>/dev/null | tail -1 | awk '{print $3, $4}' || echo "N/A")
        
        echo "   Miner $i:"
        echo "      Blocks Found: $BLOCKS_FOUND"
        echo "      Hash Rate: $HASH_RATE"
        echo "      Address: ${ADDRESSES[$i]:0:30}..."
    fi
    echo ""
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Concurrent Mining Test Complete!"
echo ""

# Calculate success metrics
SUCCESS_RATE=$((TOTAL_MINED * 100 / TOTAL_BLOCKS))
echo "🎯 Success Metrics:"
echo "   Target Blocks: $TOTAL_BLOCKS"
echo "   Blocks Mined: $TOTAL_MINED"
echo "   Success Rate: ${SUCCESS_RATE}%"
echo ""

if [ $SUCCESS_RATE -ge 80 ]; then
    echo "   Status: ✅ EXCELLENT - Test passed"
elif [ $SUCCESS_RATE -ge 50 ]; then
    echo "   Status: ✅ GOOD - Acceptable performance"
else
    echo "   Status: ⚠️  FAIR - Needs improvement"
fi

echo ""
echo "Next: ./test_transaction_throughput.sh"
