#!/bin/bash
# Launch 10-node P2P network for Phase 7 testing
# Geographic distribution simulation with multiple bootstrap nodes

set -e

cd /home/ubuntu/atmn-2.0

# Configuration
NUM_NODES=10
BASE_PORT=20000
BASE_DB_DIR="/tmp/atmn-nodes"
LOG_DIR="/tmp/atmn-logs"

echo "🌐 Phase 7: Multi-Node P2P Network Expansion"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Deploying $NUM_NODES nodes across simulated geographic regions..."
echo ""

# Clean up old data
echo "🧹 Cleaning up old node data..."
rm -rf $BASE_DB_DIR $LOG_DIR 2>/dev/null || true
mkdir -p $BASE_DB_DIR $LOG_DIR

# Kill any existing nodes
pkill -9 -f "atmn-node" 2>/dev/null || true
sleep 2

# Build node if needed
if [ ! -f atmn-node/target/release/atmn-node ]; then
    echo "🔨 Building P2P node..."
    cd atmn-node
    cargo build --release
    cd ..
fi

NODE_PIDS=()

# Start Bootstrap Node (Region: US-East)
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🚀 Starting Bootstrap Node (Node 0) - Region: US-EAST"
echo "   Port: $BASE_PORT | Database: $BASE_DB_DIR/node0.db"
touch $BASE_DB_DIR/node0.db
cd atmn-node
RUST_LOG=info nohup ./target/release/atmn-node \
    --port $BASE_PORT \
    --database $BASE_DB_DIR/node0.db \
    > $LOG_DIR/node0.log 2>&1 &
NODE_PIDS+=($!)
echo "   PID: ${NODE_PIDS[0]} ✅"
cd ..
sleep 2

# Define regions for simulation
REGIONS=("US-WEST" "EU-WEST" "EU-CENTRAL" "ASIA-EAST" "US-CENTRAL" "EU-NORTH" "ASIA-SOUTH" "SA-EAST" "AFRICA")

# Start additional nodes with various bootstrap configurations
for i in $(seq 1 $((NUM_NODES - 1))); do
    PORT=$((BASE_PORT + i))
    DB_PATH="$BASE_DB_DIR/node$i.db"
    LOG_PATH="$LOG_DIR/node$i.log"
    REGION=${REGIONS[$((i - 1))]}
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🚀 Starting Node $i - Region: $REGION"
    echo "   Port: $PORT | Database: $DB_PATH"
    
    touch $DB_PATH
    
    # Configure bootstrap nodes
    # Nodes 1-3: Bootstrap from Node 0
    # Nodes 4-6: Bootstrap from Node 1
    # Nodes 7-9: Bootstrap from Node 2
    if [ $i -le 3 ]; then
        BOOTSTRAP="127.0.0.1:$BASE_PORT"
        echo "   Bootstrap: Node 0 ($BOOTSTRAP)"
    elif [ $i -le 6 ]; then
        BOOTSTRAP="127.0.0.1:$((BASE_PORT + 1))"
        echo "   Bootstrap: Node 1 ($BOOTSTRAP)"
    else
        BOOTSTRAP="127.0.0.1:$((BASE_PORT + 2))"
        echo "   Bootstrap: Node 2 ($BOOTSTRAP)"
    fi
    
    cd atmn-node
    RUST_LOG=info nohup ./target/release/atmn-node \
        --port $PORT \
        --database $DB_PATH \
        --bootstrap $BOOTSTRAP \
        > $LOG_PATH 2>&1 &
    NODE_PIDS+=($!)
    echo "   PID: ${NODE_PIDS[$i]} ✅"
    cd ..
    
    # Stagger node startup
    sleep 1
done

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏳ Waiting for network to stabilize (10 seconds)..."
sleep 10

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Network Deployment Complete!"
echo ""
echo "📊 Network Topology:"
echo "   Bootstrap Node: Node 0 (127.0.0.1:$BASE_PORT)"
echo "   Total Nodes: $NUM_NODES"
echo "   Port Range: $BASE_PORT - $((BASE_PORT + NUM_NODES - 1))"
echo ""
echo "🌍 Geographic Distribution (Simulated):"
echo "   Node 0: US-EAST (Bootstrap)"
for i in $(seq 1 $((NUM_NODES - 1))); do
    echo "   Node $i: ${REGIONS[$((i - 1))]}"
done
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 Process Status:"
ps aux | grep "[a]tmn-node" | awk '{print "   PID: " $2 " | Port: " $(NF-3) " | CPU: " $3"% | MEM: " $4"%"}'
echo ""
echo "   Total Running Nodes: $(pgrep -f atmn-node | wc -l)"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📝 Node Connection Status (checking logs)..."
echo ""

# Check for successful connections in logs
for i in $(seq 0 $((NUM_NODES - 1))); do
    HANDSHAKES=$(grep -c "Handshake completed" $LOG_DIR/node$i.log 2>/dev/null || echo "0")
    CONNECTIONS=$(grep -c "Connected to peer" $LOG_DIR/node$i.log 2>/dev/null || echo "0")
    echo "   Node $i: $HANDSHAKES handshakes, $CONNECTIONS peer connections"
done
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Network Statistics:"
echo ""
echo "   Database Directory: $BASE_DB_DIR"
echo "   Log Directory: $LOG_DIR"
echo "   PIDs: ${NODE_PIDS[@]}"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🛠️  Management Commands:"
echo ""
echo "   Monitor all logs:"
echo "     tail -f $LOG_DIR/node*.log"
echo ""
echo "   Monitor specific node:"
echo "     tail -f $LOG_DIR/node0.log"
echo ""
echo "   Check node connections:"
echo "     grep 'Handshake\\|Connected' $LOG_DIR/node0.log"
echo ""
echo "   Stop all nodes:"
echo "     pkill -9 -f atmn-node"
echo ""
echo "   Stop specific node:"
echo "     kill ${NODE_PIDS[0]}  # Stop Node 0"
echo ""
echo "   Network monitoring script:"
echo "     ./monitor_network.sh"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🎯 Phase 7 Testing Tasks:"
echo ""
echo "   ✅ Task 1: Network deployment complete"
echo "   🔄 Task 2: Verify all nodes connected"
echo "   ⏳ Task 3: Test block propagation"
echo "   ⏳ Task 4: Test fork resolution"
echo "   ⏳ Task 5: Load test with concurrent miners"
echo "   ⏳ Task 6: Network stability monitoring"
echo ""
echo "Ready for Phase 7 testing! 🚀"
