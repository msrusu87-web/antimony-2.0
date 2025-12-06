#!/bin/bash
# Network connectivity and stability test

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🔍 Network Connectivity Test - Phase 7"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check node count
RUNNING_NODES=$(pgrep -f atmn-node | wc -l)
echo "📊 Network Status:"
echo "   Running Nodes: $RUNNING_NODES/10"
echo ""

if [ "$RUNNING_NODES" -lt 10 ]; then
    echo "❌ Not all nodes are running!"
    exit 1
fi

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🤝 Handshake Analysis:"
echo ""

TOTAL_HANDSHAKES=0
SUCCESSFUL_NODES=0

for i in {0..9}; do
    HANDSHAKES=$(grep -c "handshake" /tmp/atmn-logs/node$i.log 2>/dev/null || echo "0")
    SENT=$(grep -c "Sent handshake" /tmp/atmn-logs/node$i.log 2>/dev/null || echo "0")
    RECEIVED=$(grep -c "Received handshake" /tmp/atmn-logs/node$i.log 2>/dev/null || echo "0")
    TOTAL_HANDSHAKES=$((TOTAL_HANDSHAKES + HANDSHAKES))
    
    if [ "$HANDSHAKES" -gt 0 ]; then
        SUCCESSFUL_NODES=$((SUCCESSFUL_NODES + 1))
        echo "   Node $i: ✅ Connected (Sent: $SENT, Received: $RECEIVED)"
    else
        echo "   Node $i: ❌ No connections"
    fi
done

echo ""
echo "   Total Handshakes: $TOTAL_HANDSHAKES"
echo "   Connected Nodes: $SUCCESSFUL_NODES/10"
echo ""

# Calculate success rate
SUCCESS_RATE=$((SUCCESSFUL_NODES * 10))
echo -n "   Connection Success Rate: $SUCCESS_RATE% "
if [ $SUCCESS_RATE -eq 100 ]; then
    echo "✅ Excellent"
elif [ $SUCCESS_RATE -ge 80 ]; then
    echo "✅ Good"
elif [ $SUCCESS_RATE -ge 60 ]; then
    echo "⚠️  Fair"
else
    echo "❌ Poor"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "🌍 Bootstrap Topology:"
echo ""

echo "   Bootstrap Node (Node 0):"
BOOTSTRAP_HANDSHAKES=$(grep -c "Received handshake" /tmp/atmn-logs/node0.log 2>/dev/null || echo "0")
echo "      Accepted $BOOTSTRAP_HANDSHAKES incoming connections"
echo ""

echo "   Secondary Bootstrap (Node 1):"
NODE1_HANDSHAKES=$(grep -c "Received handshake" /tmp/atmn-logs/node1.log 2>/dev/null || echo "0")
echo "      Accepted $NODE1_HANDSHAKES incoming connections"
echo ""

echo "   Tertiary Bootstrap (Node 2):"
NODE2_HANDSHAKES=$(grep -c "Received handshake" /tmp/atmn-logs/node2.log 2>/dev/null || echo "0")
echo "      Accepted $NODE2_HANDSHAKES incoming connections"
echo ""

# Check for errors
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⚠️  Error Analysis:"
echo ""

TOTAL_ERRORS=0
for i in {0..9}; do
    ERRORS=$(grep -c "ERROR\|Error\|Failed" /tmp/atmn-logs/node$i.log 2>/dev/null || echo "0")
    TOTAL_ERRORS=$((TOTAL_ERRORS + ERRORS))
    if [ "$ERRORS" -gt 0 ]; then
        echo "   Node $i: $ERRORS errors detected"
    fi
done

if [ $TOTAL_ERRORS -eq 0 ]; then
    echo "   ✅ No errors detected across all nodes"
else
    echo "   ⚠️  Total Errors: $TOTAL_ERRORS"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "⏱️  Uptime Check:"
echo ""

START_TIME=$(ls -l /tmp/atmn-logs/node0.log | awk '{print $6, $7, $8}')
CURRENT_TIME=$(date '+%b %d %H:%M')
echo "   Network Started: $START_TIME"
echo "   Current Time: $CURRENT_TIME"
echo ""

# Check if all nodes are still responsive
RESPONSIVE=0
for i in {0..9}; do
    # Check if log was modified in last 60 seconds
    if [ -f "/tmp/atmn-logs/node$i.log" ]; then
        AGE=$(find /tmp/atmn-logs/node$i.log -mmin -1 | wc -l)
        if [ "$AGE" -gt 0 ]; then
            RESPONSIVE=$((RESPONSIVE + 1))
        fi
    fi
done

echo "   Responsive Nodes: $RESPONSIVE/10"
if [ $RESPONSIVE -lt 10 ]; then
    echo "   ⚠️  Some nodes may be idle or stalled"
else
    echo "   ✅ All nodes actively logging"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "📊 Summary:"
echo ""

OVERALL_HEALTH=0
[ $RUNNING_NODES -eq 10 ] && OVERALL_HEALTH=$((OVERALL_HEALTH + 25))
[ $SUCCESSFUL_NODES -ge 8 ] && OVERALL_HEALTH=$((OVERALL_HEALTH + 25))
[ $TOTAL_ERRORS -eq 0 ] && OVERALL_HEALTH=$((OVERALL_HEALTH + 25))
[ $RESPONSIVE -ge 8 ] && OVERALL_HEALTH=$((OVERALL_HEALTH + 25))

echo "   Network Health Score: $OVERALL_HEALTH/100"
echo ""

if [ $OVERALL_HEALTH -ge 90 ]; then
    echo "   Status: ✅ EXCELLENT - Network is stable and healthy"
elif [ $OVERALL_HEALTH -ge 75 ]; then
    echo "   Status: ✅ GOOD - Network is functioning well"
elif [ $OVERALL_HEALTH -ge 50 ]; then
    echo "   Status: ⚠️  FAIR - Some issues detected"
else
    echo "   Status: ❌ POOR - Network has significant issues"
fi
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "✅ Connectivity Test Complete!"
echo ""

# Recommendations
if [ $OVERALL_HEALTH -ge 75 ]; then
    echo "🎯 Ready for:"
    echo "   - Load testing with concurrent transactions"
    echo "   - Fork resolution testing"
    echo "   - Geographic distribution simulation"
fi
