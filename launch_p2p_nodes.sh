#!/bin/bash
# Launch 3 P2P nodes for multi-node testing

cd /home/ubuntu/atmn-2.0

echo "🌐 Multi-Node P2P Testing"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Clean up old databases and logs
rm -f /tmp/node1.db* /tmp/node2.db* /tmp/node3.db* 2>/dev/null
rm -f /tmp/node*.log 2>/dev/null

# Kill any existing nodes
pkill -9 -f "atmn-node" 2>/dev/null
sleep 3

# Create empty database files
touch /tmp/node1.db /tmp/node2.db /tmp/node3.db

echo "🚀 Starting Node 1 (Bootstrap node on port 19000)..."
cd atmn-node
RUST_LOG=info nohup ./target/release/atmn-node --port 19000 --database /tmp/node1.db > /tmp/node1.log 2>&1 &
NODE1_PID=$!
echo "   PID: $NODE1_PID"
sleep 3

echo "🚀 Starting Node 2 (port 19001, bootstraps from Node 1)..."
RUST_LOG=info nohup ./target/release/atmn-node --port 19001 --database /tmp/node2.db --bootstrap 127.0.0.1:19000 > /tmp/node2.log 2>&1 &
NODE2_PID=$!
echo "   PID: $NODE2_PID"
sleep 3

echo "🚀 Starting Node 3 (port 19002, bootstraps from Node 1)..."
RUST_LOG=info nohup ./target/release/atmn-node --port 19002 --database /tmp/node3.db --bootstrap 127.0.0.1:19000 > /tmp/node3.log 2>&1 &
NODE3_PID=$!
echo "   PID: $NODE3_PID"
sleep 3

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ All nodes started!"
echo ""
echo "Node 1: localhost:19000 (PID: $NODE1_PID)"
echo "Node 2: localhost:19001 (PID: $NODE2_PID)"
echo "Node 3: localhost:19002 (PID: $NODE3_PID)"
echo ""

# Wait for nodes to connect
echo "⏳ Waiting for nodes to establish connections (5 seconds)..."
sleep 5

echo ""
echo "📊 Node Status:"
echo ""

echo "Node 1 Log (last 10 lines):"
tail -10 /tmp/node1.log
echo ""

echo "Node 2 Log (last 10 lines):"
tail -10 /tmp/node2.log
echo ""

echo "Node 3 Log (last 10 lines):"
tail -10 /tmp/node3.log
echo ""

# Check if nodes are running
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 Process Status:"
ps aux | grep "[a]tmn-node" | awk '{print "   PID " $2 " - Port " $(NF-3)}'
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📝 Next Steps:"
echo "   1. Monitor logs: tail -f /tmp/node*.log"
echo "   2. Test block propagation by mining a block"
echo "   3. Stop nodes: kill $NODE1_PID $NODE2_PID $NODE3_PID"
echo ""
echo "Log files:"
echo "   /tmp/node1.log"
echo "   /tmp/node2.log"
echo "   /tmp/node3.log"
