#!/bin/bash

################################################################################
# ANTIMONY 2.0 - Public Testnet Deployment
# Deploys public testnet with RPC endpoints and faucet
################################################################################

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m'

# Configuration
TESTNET_NODES=5
BASE_PORT=30000
RPC_PORT=8545
TESTNET_DIR="/home/ubuntu/atmn-testnet"
GENESIS_SUPPLY=1000000  # 1M ATMN for testnet
BLOCK_REWARD=50         # 50 ATMN per block
DIFFICULTY=8            # Lower difficulty for testnet

echo -e "${PURPLE}╔════════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║   ANTIMONY 2.0 - TESTNET DEPLOYMENT       ║${NC}"
echo -e "${PURPLE}╚════════════════════════════════════════════╝${NC}"
echo ""

# ============================================================================
# Step 1: Prepare Testnet Environment
# ============================================================================

echo -e "${BLUE}[1/7] Preparing testnet environment...${NC}"

# Create testnet directory
mkdir -p "$TESTNET_DIR"/{nodes,logs,data,config}
cd "$TESTNET_DIR"

# Copy binaries
if [ -f "/home/ubuntu/atmn-2.0/target/release/atmn-node" ]; then
    cp /home/ubuntu/atmn-2.0/target/release/atmn-node ./
    echo -e "${GREEN}✓ Node binary copied${NC}"
else
    echo -e "${RED}✗ Node binary not found - building...${NC}"
    cd /home/ubuntu/atmn-2.0
    cargo build --release
    cp target/release/atmn-node "$TESTNET_DIR/"
    cd "$TESTNET_DIR"
fi

# ============================================================================
# Step 2: Generate Testnet Genesis Block
# ============================================================================

echo -e "${BLUE}[2/7] Generating testnet genesis block...${NC}"

cat > config/genesis.json <<EOF
{
  "network": "testnet",
  "timestamp": $(date +%s),
  "difficulty": $DIFFICULTY,
  "block_reward": $BLOCK_REWARD,
  "halving_interval": 210000,
  "genesis_supply": $GENESIS_SUPPLY,
  "genesis_address": "atmn1qtestnetgenesisaddress000000000000qwerty",
  "magic_bytes": "0xATMNTEST",
  "network_version": 1,
  "min_tx_fee": 1000,
  "max_block_size": 4194304,
  "target_block_time": 600,
  "difficulty_adjustment_interval": 2016
}
EOF

echo -e "${GREEN}✓ Genesis configuration created${NC}"

# ============================================================================
# Step 3: Initialize Testnet Nodes
# ============================================================================

echo -e "${BLUE}[3/7] Initializing $TESTNET_NODES testnet nodes...${NC}"

for i in $(seq 0 $((TESTNET_NODES-1))); do
    NODE_DIR="nodes/node$i"
    mkdir -p "$NODE_DIR"/{data,keys}
    
    # Generate node configuration
    cat > "$NODE_DIR/config.toml" <<EOF
[network]
listen_address = "0.0.0.0:$((BASE_PORT + i))"
network_id = "testnet"
max_peers = 50
enable_upnp = true

[mining]
enabled = true
threads = 2
address = "atmn1qtestnode${i}miningaddress00000000000000"

[rpc]
enabled = true
listen_address = "0.0.0.0:$((RPC_PORT + i))"
allowed_origins = ["*"]
max_connections = 100

[database]
path = "$NODE_DIR/data"
cache_size = 1024

[logging]
level = "info"
file = "$TESTNET_DIR/logs/node$i.log"
EOF
    
    echo -e "${GREEN}  ✓ Node $i configured${NC}"
done

# ============================================================================
# Step 4: Start Testnet Nodes
# ============================================================================

echo -e "${BLUE}[4/7] Starting testnet nodes...${NC}"

for i in $(seq 0 $((TESTNET_NODES-1))); do
    NODE_DIR="nodes/node$i"
    
    # Start node in background
    nohup ./atmn-node \
        --config "$NODE_DIR/config.toml" \
        --genesis config/genesis.json \
        --testnet \
        > "logs/node$i.log" 2>&1 &
    
    NODE_PID=$!
    echo $NODE_PID > "$NODE_DIR/node.pid"
    
    echo -e "${GREEN}  ✓ Node $i started (PID: $NODE_PID, Port: $((BASE_PORT + i)))${NC}"
    
    sleep 2
done

echo -e "${GREEN}✓ All nodes started${NC}"
sleep 5

# ============================================================================
# Step 5: Configure Node Peering
# ============================================================================

echo -e "${BLUE}[5/7] Connecting nodes as peers...${NC}"

# Connect each node to the next one (ring topology)
for i in $(seq 0 $((TESTNET_NODES-2))); do
    NEXT=$((i + 1))
    
    # Add peer via RPC
    curl -s -X POST http://localhost:$((RPC_PORT + i))/rpc \
        -H "Content-Type: application/json" \
        -d "{
            \"method\": \"addpeer\",
            \"params\": [\"127.0.0.1:$((BASE_PORT + NEXT))\"]
        }" > /dev/null 2>&1 || true
    
    echo -e "${GREEN}  ✓ Node $i → Node $NEXT${NC}"
done

# Connect last to first (complete the ring)
curl -s -X POST http://localhost:$((RPC_PORT + TESTNET_NODES - 1))/rpc \
    -H "Content-Type: application/json" \
    -d "{
        \"method\": \"addpeer\",
        \"params\": [\"127.0.0.1:$BASE_PORT\"]
    }" > /dev/null 2>&1 || true

echo -e "${GREEN}✓ Peer topology established${NC}"

# ============================================================================
# Step 6: Deploy Faucet Service
# ============================================================================

echo -e "${BLUE}[6/7] Deploying faucet service...${NC}"

# Copy faucet script
cp /home/ubuntu/atmn-2.0/faucet_service.sh ./
chmod +x faucet_service.sh

# Initialize faucet database
./faucet_service.sh init

# Create faucet web interface
mkdir -p www

cat > www/faucet.html <<'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>ATMN Testnet Faucet</title>
    <style>
        body {
            font-family: 'Segoe UI', Arial, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            margin: 0;
            padding: 20px;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .container {
            background: white;
            border-radius: 15px;
            padding: 40px;
            max-width: 600px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        }
        h1 {
            color: #667eea;
            margin-top: 0;
            font-size: 32px;
        }
        .subtitle {
            color: #666;
            margin-bottom: 30px;
        }
        input {
            width: 100%;
            padding: 15px;
            border: 2px solid #e0e0e0;
            border-radius: 8px;
            font-size: 16px;
            box-sizing: border-box;
            margin-bottom: 15px;
        }
        button {
            width: 100%;
            padding: 15px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            border-radius: 8px;
            font-size: 18px;
            font-weight: bold;
            cursor: pointer;
            transition: transform 0.2s;
        }
        button:hover {
            transform: translateY(-2px);
        }
        button:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }
        .info {
            background: #f5f5f5;
            padding: 20px;
            border-radius: 8px;
            margin-top: 30px;
        }
        .info h3 {
            margin-top: 0;
            color: #333;
        }
        .info ul {
            margin: 10px 0;
            padding-left: 20px;
        }
        .status {
            margin-top: 20px;
            padding: 15px;
            border-radius: 8px;
            display: none;
        }
        .status.success {
            background: #d4edda;
            color: #155724;
            border: 1px solid #c3e6cb;
        }
        .status.error {
            background: #f8d7da;
            color: #721c24;
            border: 1px solid #f5c6cb;
        }
        .tx-hash {
            word-break: break-all;
            font-family: monospace;
            background: white;
            padding: 10px;
            border-radius: 5px;
            margin-top: 10px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>💧 ATMN Testnet Faucet</h1>
        <p class="subtitle">Get free test ATMN for development</p>
        
        <form id="faucetForm">
            <input type="text" 
                   id="address" 
                   placeholder="Enter your ATMN testnet address (atmn1...)" 
                   pattern="^atmn1[a-z0-9]{38,58}$"
                   required>
            <button type="submit" id="submitBtn">Request 10 ATMN</button>
        </form>
        
        <div id="status" class="status"></div>
        
        <div class="info">
            <h3>ℹ️ Faucet Information</h3>
            <ul>
                <li><strong>Amount:</strong> 10 ATMN per request</li>
                <li><strong>Cooldown:</strong> 1 hour per address</li>
                <li><strong>Daily Limit:</strong> 100 ATMN per IP</li>
                <li><strong>Network:</strong> Testnet</li>
            </ul>
            
            <h3>📚 Resources</h3>
            <ul>
                <li><a href="https://github.com/msrusu87-web/antimony-2.0">GitHub Repository</a></li>
                <li><a href="https://explorer.carphatian.ro">Block Explorer</a></li>
                <li><a href="/docs">Documentation</a></li>
            </ul>
        </div>
    </div>
    
    <script>
        document.getElementById('faucetForm').addEventListener('submit', async (e) => {
            e.preventDefault();
            
            const address = document.getElementById('address').value;
            const submitBtn = document.getElementById('submitBtn');
            const status = document.getElementById('status');
            
            // Disable button
            submitBtn.disabled = true;
            submitBtn.textContent = 'Processing...';
            status.style.display = 'none';
            
            try {
                const response = await fetch('/api/faucet', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify({ address })
                });
                
                const data = await response.json();
                
                if (response.ok) {
                    status.className = 'status success';
                    status.innerHTML = `
                        <strong>✓ Success!</strong><br>
                        10 ATMN sent to your address.<br>
                        <div class="tx-hash">TX: ${data.tx_hash}</div>
                    `;
                } else {
                    status.className = 'status error';
                    status.innerHTML = `<strong>✗ Error:</strong> ${data.error}`;
                }
                
                status.style.display = 'block';
            } catch (error) {
                status.className = 'status error';
                status.innerHTML = `<strong>✗ Error:</strong> ${error.message}`;
                status.style.display = 'block';
            } finally {
                submitBtn.disabled = false;
                submitBtn.textContent = 'Request 10 ATMN';
            }
        });
    </script>
</body>
</html>
EOF

echo -e "${GREEN}✓ Faucet deployed${NC}"

# ============================================================================
# Step 7: Display Testnet Information
# ============================================================================

echo ""
echo -e "${PURPLE}╔════════════════════════════════════════════╗${NC}"
echo -e "${PURPLE}║        TESTNET DEPLOYMENT COMPLETE         ║${NC}"
echo -e "${PURPLE}╚════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${GREEN}✓ Testnet Status:${NC}"
echo -e "  Network: testnet"
echo -e "  Nodes: $TESTNET_NODES"
echo -e "  Genesis Supply: $GENESIS_SUPPLY ATMN"
echo -e "  Block Reward: $BLOCK_REWARD ATMN"
echo ""

echo -e "${BLUE}RPC Endpoints:${NC}"
for i in $(seq 0 $((TESTNET_NODES-1))); do
    echo -e "  Node $i: http://localhost:$((RPC_PORT + i))"
done
echo ""

echo -e "${BLUE}P2P Ports:${NC}"
for i in $(seq 0 $((TESTNET_NODES-1))); do
    echo -e "  Node $i: $((BASE_PORT + i))"
done
echo ""

echo -e "${YELLOW}Faucet:${NC}"
echo -e "  URL: http://localhost:8080/faucet.html"
echo -e "  Script: $TESTNET_DIR/faucet_service.sh"
echo ""

echo -e "${YELLOW}Management Commands:${NC}"
echo -e "  Stop all nodes:  $0 stop"
echo -e "  Check status:    $0 status"
echo -e "  View logs:       tail -f logs/node0.log"
echo -e "  Faucet stats:    ./faucet_service.sh stats"
echo ""

echo -e "${GREEN}Testnet is running!${NC}"

# Save deployment info
cat > testnet_info.txt <<EOF
ANTIMONY TESTNET DEPLOYMENT
===========================
Deployment Time: $(date)
Network: testnet
Nodes: $TESTNET_NODES
Base Port: $BASE_PORT
RPC Port: $RPC_PORT

RPC Endpoints:
$(for i in $(seq 0 $((TESTNET_NODES-1))); do echo "  http://localhost:$((RPC_PORT + i))"; done)

Faucet: http://localhost:8080/faucet.html

Genesis Block:
  Supply: $GENESIS_SUPPLY ATMN
  Reward: $BLOCK_REWARD ATMN
  Difficulty: $DIFFICULTY
EOF

echo -e "${GREEN}✓ Deployment info saved to testnet_info.txt${NC}"
