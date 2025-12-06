#!/bin/bash
# Mining monitor script - checks progress and notifies on completion

echo "🔍 ANTIMONY MINING MONITOR"
echo "=========================="
echo ""

TARGET_BLOCKS=10
CHECK_INTERVAL=5  # seconds

while true; do
    # Get current blockchain height
    HEIGHT=$(curl -s http://localhost:8000/api/blockchain/stats 2>/dev/null | grep -o '"current_height":[0-9]*' | cut -d':' -f2)
    
    if [ -z "$HEIGHT" ]; then
        HEIGHT=0
    fi
    
    TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
    
    echo "[$TIMESTAMP] Current height: $HEIGHT / $TARGET_BLOCKS blocks"
    
    # Check if target reached
    if [ "$HEIGHT" -ge "$TARGET_BLOCKS" ]; then
        echo ""
        echo "🎉 ============================================"
        echo "🎉 TARGET REACHED! $HEIGHT blocks mined!"
        echo "🎉 ============================================"
        echo ""
        
        # Get latest block info
        echo "Latest block info:"
        curl -s http://localhost:8000/api/blocks/latest | jq '.' 2>/dev/null || echo "API not responding"
        
        # Get blockchain stats
        echo ""
        echo "Blockchain stats:"
        curl -s http://localhost:8000/api/blockchain/stats | jq '.' 2>/dev/null || echo "API not responding"
        
        exit 0
    fi
    
    # Check if mining process is running
    if ! pgrep -f "mine-production" > /dev/null; then
        echo "⚠️  WARNING: Mining process not detected!"
    fi
    
    sleep $CHECK_INTERVAL
done
