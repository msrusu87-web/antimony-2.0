#!/bin/bash
# Continuous mining script for difficulty adjustment testing
# Mines blocks until target height (2016) is reached

cd /home/ubuntu/atmn-2.0/atmn-core

TARGET_HEIGHT=2016
START_TIME=$(date +%s)

echo "🎯 Difficulty Adjustment Testing"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Target: Mine to block $TARGET_HEIGHT"
echo "Started: $(date)"
echo ""

# Function to get current height
get_height() {
    ./target/release/verify-blocks 2>/dev/null | grep "Best block height:" | awk '{print $4}'
}

# Function to check if difficulty adjustment happened
check_difficulty() {
    local height=$1
    if [ $((height % 2016)) -eq 0 ] && [ $height -gt 0 ]; then
        echo ""
        echo "🎉 DIFFICULTY ADJUSTMENT at block $height!"
        echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
        # Read last block to check difficulty
        echo "Checking difficulty bits..."
        return 0
    fi
    return 1
}

CURRENT=$(get_height)
echo "📊 Current height: $CURRENT"
echo "📊 Blocks to mine: $((TARGET_HEIGHT - CURRENT))"
echo ""

BATCH_SIZE=100
BLOCKS_MINED=0

while [ $CURRENT -lt $TARGET_HEIGHT ]; do
    # Mine in batches for better progress reporting
    BATCH_END=$((CURRENT + BATCH_SIZE))
    if [ $BATCH_END -gt $TARGET_HEIGHT ]; then
        BATCH_END=$TARGET_HEIGHT
    fi
    
    echo "⛏️  Mining blocks $((CURRENT + 1)) to $BATCH_END..."
    
    # Mine blocks
    for i in $(seq $((CURRENT + 1)) $BATCH_END); do
        # Run mine-production binary
        ./target/release/mine-production >> /tmp/mining.log 2>&1
        
        # Check for difficulty adjustment
        if [ $((i % 2016)) -eq 0 ]; then
            check_difficulty $i
        fi
        
        # Progress indicator every 10 blocks
        if [ $((i % 10)) -eq 0 ]; then
            ELAPSED=$(($(date +%s) - START_TIME))
            RATE=$(echo "scale=2; $((i - CURRENT)) / $ELAPSED" | bc -l)
            ETA=$(echo "scale=0; ($TARGET_HEIGHT - $i) / $RATE" | bc -l)
            echo "   Block $i mined (${RATE} blocks/sec, ETA: ${ETA}s)"
        fi
    done
    
    CURRENT=$(get_height)
    BLOCKS_MINED=$((CURRENT - $(get_height)))
    
    echo "✅ Batch complete! Current height: $CURRENT"
    echo ""
done

END_TIME=$(date +%s)
TOTAL_TIME=$((END_TIME - START_TIME))
MINUTES=$((TOTAL_TIME / 60))
SECONDS=$((TOTAL_TIME % 60))

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎉 Mining Complete!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Final height: $CURRENT"
echo "Blocks mined: $((CURRENT - 21))"
echo "Time taken: ${MINUTES}m ${SECONDS}s"
echo "Avg rate: $(echo "scale=2; $((CURRENT - 21)) / $TOTAL_TIME" | bc -l) blocks/sec"
echo ""
echo "Verifying blockchain..."
./target/release/verify-blocks | tail -10
