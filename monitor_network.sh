#!/bin/bash
# Network monitoring script for Phase 7 testing

LOG_DIR="/tmp/atmn-logs"
DB_DIR="/tmp/atmn-nodes"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

clear
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🌐 ANTIMONY P2P Network Monitor - Phase 7"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if nodes are running
RUNNING_NODES=$(pgrep -f atmn-node | wc -l)

if [ "$RUNNING_NODES" -eq 0 ]; then
    echo -e "${RED}❌ No nodes running!${NC}"
    echo ""
    echo "Start the network with: ./launch_network_expanded.sh"
    exit 1
fi

echo -e "${GREEN}✅ Network Status: $RUNNING_NODES nodes running${NC}"
echo ""

# Function to get node statistics
get_node_stats() {
    NODE_NUM=$1
    LOG_FILE="$LOG_DIR/node$NODE_NUM.log"
    
    if [ ! -f "$LOG_FILE" ]; then
        echo -e "${RED}   Log file not found${NC}"
        return
    fi
    
    # Count events
    HANDSHAKES=$(grep -c "Handshake completed" "$LOG_FILE" 2>/dev/null || echo "0")
    CONNECTIONS=$(grep -c "Connected to peer" "$LOG_FILE" 2>/dev/null || echo "0")
    BLOCKS_RECV=$(grep -c "Received block" "$LOG_FILE" 2>/dev/null || echo "0")
    BLOCKS_SENT=$(grep -c "Broadcasting block" "$LOG_FILE" 2>/dev/null || echo "0")
    TX_RECV=$(grep -c "Received transaction" "$LOG_FILE" 2>/dev/null || echo "0")
    TX_SENT=$(grep -c "Broadcasting transaction" "$LOG_FILE" 2>/dev/null || echo "0")
    ERRORS=$(grep -c "ERROR\|Error\|error" "$LOG_FILE" 2>/dev/null || echo "0")
    
    # Get latest block height if available
    HEIGHT=$(grep "Block height" "$LOG_FILE" 2>/dev/null | tail -1 | grep -o '[0-9]*' | tail -1)
    [ -z "$HEIGHT" ] && HEIGHT="0"
    
    # Check if process is alive
    PID=$(pgrep -f "atmn-node.*$(($BASE_PORT + NODE_NUM))" 2>/dev/null || echo "")
    if [ -n "$PID" ]; then
        STATUS="${GREEN}●${NC}"
        CPU=$(ps -p $PID -o %cpu= 2>/dev/null | xargs)
        MEM=$(ps -p $PID -o %mem= 2>/dev/null | xargs)
    else
        STATUS="${RED}●${NC}"
        CPU="N/A"
        MEM="N/A"
    fi
    
    echo -e "${STATUS} Node $NODE_NUM | Port: $((BASE_PORT + NODE_NUM)) | PID: ${PID:-N/A}"
    echo "   Height: $HEIGHT | Handshakes: $HANDSHAKES | Connections: $CONNECTIONS"
    echo "   Blocks: ↓$BLOCKS_RECV ↑$BLOCKS_SENT | Txs: ↓$TX_RECV ↑$TX_SENT"
    echo "   CPU: $CPU% | MEM: $MEM% | Errors: $ERRORS"
    echo ""
}

# Network configuration
BASE_PORT=20000
NUM_NODES=10

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Node Statistics:"
echo ""

# Display stats for all nodes
for i in $(seq 0 $((NUM_NODES - 1))); do
    get_node_stats $i
done

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔗 Network Topology Analysis:"
echo ""

# Count total connections
TOTAL_HANDSHAKES=0
TOTAL_CONNECTIONS=0
for i in $(seq 0 $((NUM_NODES - 1))); do
    HS=$(grep -c "Handshake completed" "$LOG_DIR/node$i.log" 2>/dev/null || echo "0")
    CONN=$(grep -c "Connected to peer" "$LOG_DIR/node$i.log" 2>/dev/null || echo "0")
    TOTAL_HANDSHAKES=$((TOTAL_HANDSHAKES + HS))
    TOTAL_CONNECTIONS=$((TOTAL_CONNECTIONS + CONN))
done

echo "   Total Handshakes: $TOTAL_HANDSHAKES"
echo "   Total Connections: $TOTAL_CONNECTIONS"
echo "   Average Connections per Node: $((TOTAL_CONNECTIONS / NUM_NODES))"
echo ""

# Calculate network health score
HEALTH_SCORE=$((RUNNING_NODES * 10))
if [ "$TOTAL_CONNECTIONS" -gt "$((NUM_NODES * 2))" ]; then
    HEALTH_SCORE=$((HEALTH_SCORE + 20))
fi

if [ $HEALTH_SCORE -ge 80 ]; then
    HEALTH_COLOR=$GREEN
    HEALTH_STATUS="Excellent"
elif [ $HEALTH_SCORE -ge 60 ]; then
    HEALTH_COLOR=$YELLOW
    HEALTH_STATUS="Good"
else
    HEALTH_COLOR=$RED
    HEALTH_STATUS="Poor"
fi

echo -e "   Network Health: ${HEALTH_COLOR}$HEALTH_SCORE/100 ($HEALTH_STATUS)${NC}"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📈 Network Activity (Last 5 Events):"
echo ""

# Get recent events from all logs
echo "Recent Events:"
tail -n 5 $LOG_DIR/node*.log 2>/dev/null | grep -E "Handshake|Connected|block|transaction" | tail -10 || echo "   No recent activity"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🛠️  Quick Actions:"
echo ""
echo "   [1] View detailed logs: tail -f $LOG_DIR/node0.log"
echo "   [2] Check specific node: grep 'ERROR' $LOG_DIR/node5.log"
echo "   [3] Test block propagation: ./test_block_propagation.sh"
echo "   [4] Stop all nodes: pkill -9 -f atmn-node"
echo "   [5] Restart monitoring: watch -n 5 ./monitor_network.sh"
echo ""
echo "Last updated: $(date)"
