#!/bin/bash
# Test block propagation across 10-node network

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🧪 Block Propagation Test - Phase 7"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if nodes are running
RUNNING_NODES=$(pgrep -f atmn-node | wc -l)
if [ "$RUNNING_NODES" -lt 10 ]; then
    echo "❌ Error: Expected 10 nodes, found $RUNNING_NODES"
    echo "   Start the network with: ./launch_network_expanded.sh"
    exit 1
fi

echo "✅ Network Status: $RUNNING_NODES nodes running"
echo ""

# Get initial block heights
echo "📊 Initial Block Heights:"
for i in {0..9}; do
    HEIGHT=$(grep "Block height" /tmp/atmn-logs/node$i.log 2>/dev/null | tail -1 | grep -o '[0-9]*' | tail -1)
    [ -z "$HEIGHT" ] && HEIGHT="0"
    echo "   Node $i: $HEIGHT"
done
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⛏️  Starting miner on Node 0 to mine 5 blocks..."
echo ""

# Mine 5 blocks and watch propagation
cd /home/ubuntu/atmn-2.0/atmn-core

MINER_ADDRESS="AW1nRFVkd2J2eFhZb1pwUzJNeWNBS0RVZTJlNkE="

for BLOCK_NUM in {1..5}; do
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Mining block $BLOCK_NUM..."
    
    # Clear previous mining logs
    > /tmp/mining.log
    
    # Mine a single block
    timeout 30s cargo run --release --bin mine-to-height -- \
        --target-height $BLOCK_NUM \
        --miner-address "$MINER_ADDRESS" \
        > /tmp/mining.log 2>&1 &
    
    MINER_PID=$!
    
    # Wait for block to be mined
    echo -n "   Mining in progress"
    for i in {1..30}; do
        if grep -q "Block mined" /tmp/mining.log 2>/dev/null; then
            echo " ✅"
            break
        fi
        echo -n "."
        sleep 1
    done
    echo ""
    
    # Kill miner if still running
    kill $MINER_PID 2>/dev/null || true
    
    echo ""
    echo "⏳ Waiting for block propagation (5 seconds)..."
    sleep 5
    
    echo ""
    echo "📊 Block heights after mining block $BLOCK_NUM:"
    for i in {0..9}; do
        HEIGHT=$(grep "Block height" /tmp/atmn-logs/node$i.log 2>/dev/null | tail -1 | grep -o '[0-9]*' | tail -1)
        [ -z "$HEIGHT" ] && HEIGHT="0"
        BLOCKS_RECEIVED=$(grep -c "Received block" /tmp/atmn-logs/node$i.log 2>/dev/null || echo "0")
        echo "   Node $i: Height $HEIGHT (Received $BLOCKS_RECEIVED blocks)"
    done
    echo ""
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📈 Final Statistics:"
echo ""

# Count propagation events
TOTAL_BROADCASTS=0
TOTAL_RECEIVES=0

for i in {0..9}; do
    BROADCASTS=$(grep -c "Broadcasting block" /tmp/atmn-logs/node$i.log 2>/dev/null || echo "0")
    RECEIVES=$(grep -c "Received block" /tmp/atmn-logs/node$i.log 2>/dev/null || echo "0")
    TOTAL_BROADCASTS=$((TOTAL_BROADCASTS + BROADCASTS))
    TOTAL_RECEIVES=$((TOTAL_RECEIVES + RECEIVES))
done

echo "   Total Blocks Broadcast: $TOTAL_BROADCASTS"
echo "   Total Blocks Received: $TOTAL_RECEIVES"
echo "   Average Receives per Node: $((TOTAL_RECEIVES / 10))"
echo ""

# Calculate propagation success rate
echo "🎯 Propagation Analysis:"
for i in {0..9}; do
    HEIGHT=$(grep "Block height" /tmp/atmn-logs/node$i.log 2>/dev/null | tail -1 | grep -o '[0-9]*' | tail -1)
    [ -z "$HEIGHT" ] && HEIGHT="0"
    if [ "$HEIGHT" -eq 5 ]; then
        echo "   Node $i: ✅ Synchronized (Height: $HEIGHT)"
    else
        echo "   Node $i: ⚠️  Out of sync (Height: $HEIGHT)"
    fi
done
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Block Propagation Test Complete!"
echo ""
echo "Next: ./test_fork_resolution.sh"
