#!/bin/bash

################################################################################
# ANTIMONY 2.0 - Cloud Node Deployment Script
# Phase 8: Mainnet Preparation
#
# Purpose: Deploy production-grade ATMN nodes to cloud providers
# Supports: AWS, GCP, Azure, DigitalOcean, Linode
#
# Usage: ./deploy_cloud_node.sh [provider] [region] [node-type]
# Example: ./deploy_cloud_node.sh aws us-east-1 full-node
################################################################################

set -e

# Configuration
ATMN_VERSION="2.0.0"
ATMN_REPO="https://github.com/msrusu87-web/antimony-2.0.git"
ATMN_BRANCH="main"

# Node types
NODE_TYPE_FULL="full-node"      # Full blockchain with mining
NODE_TYPE_VALIDATOR="validator"  # Full node without mining
NODE_TYPE_BOOTSTRAP="bootstrap"  # Bootstrap seed node

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
PROVIDER="${1:-aws}"
REGION="${2:-us-east-1}"
NODE_TYPE="${3:-$NODE_TYPE_FULL}"

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}ANTIMONY 2.0 - Cloud Node Deployment${NC}"
echo -e "${BLUE}============================================${NC}"
echo -e "Provider: ${GREEN}$PROVIDER${NC}"
echo -e "Region: ${GREEN}$REGION${NC}"
echo -e "Node Type: ${GREEN}$NODE_TYPE${NC}"
echo ""

################################################################################
# System Requirements Check
################################################################################
check_system_requirements() {
    echo -e "${YELLOW}[1/10]${NC} Checking system requirements..."
    
    # Check CPU cores
    CPU_CORES=$(nproc)
    if [ "$CPU_CORES" -lt 2 ]; then
        echo -e "${RED}ERROR: Minimum 2 CPU cores required (found: $CPU_CORES)${NC}"
        exit 1
    fi
    
    # Check RAM
    TOTAL_RAM=$(free -g | awk '/^Mem:/{print $2}')
    if [ "$TOTAL_RAM" -lt 4 ]; then
        echo -e "${RED}ERROR: Minimum 4GB RAM required (found: ${TOTAL_RAM}GB)${NC}"
        exit 1
    fi
    
    # Check disk space
    DISK_SPACE=$(df -BG / | awk 'NR==2 {print $4}' | sed 's/G//')
    if [ "$DISK_SPACE" -lt 50 ]; then
        echo -e "${RED}ERROR: Minimum 50GB free disk space required (found: ${DISK_SPACE}GB)${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✓ System requirements met${NC}"
    echo "  CPU Cores: $CPU_CORES"
    echo "  RAM: ${TOTAL_RAM}GB"
    echo "  Disk Space: ${DISK_SPACE}GB"
    echo ""
}

################################################################################
# Install Dependencies
################################################################################
install_dependencies() {
    echo -e "${YELLOW}[2/10]${NC} Installing dependencies..."
    
    # Update package lists
    sudo apt-get update -qq
    
    # Install Rust dependencies
    sudo apt-get install -y -qq \
        build-essential \
        pkg-config \
        libssl-dev \
        librocksdb-dev \
        libsqlite3-dev \
        clang \
        cmake \
        git \
        curl \
        wget \
        htop \
        net-tools \
        ufw
    
    # Install Rust if not present
    if ! command -v rustc &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
    
    echo -e "${GREEN}✓ Dependencies installed${NC}"
    echo ""
}

################################################################################
# Configure Firewall
################################################################################
configure_firewall() {
    echo -e "${YELLOW}[3/10]${NC} Configuring firewall..."
    
    # Enable UFW
    sudo ufw --force enable
    
    # Allow SSH
    sudo ufw allow 22/tcp
    
    # Allow P2P ports (19000-19010)
    sudo ufw allow 19000:19010/tcp
    
    # Allow API ports if full node
    if [ "$NODE_TYPE" = "$NODE_TYPE_FULL" ]; then
        sudo ufw allow 8000/tcp  # Mining API
        sudo ufw allow 8080/tcp  # Rosetta API
    fi
    
    # Reload firewall
    sudo ufw reload
    
    echo -e "${GREEN}✓ Firewall configured${NC}"
    echo ""
}

################################################################################
# Clone ATMN Repository
################################################################################
clone_repository() {
    echo -e "${YELLOW}[4/10]${NC} Cloning ATMN repository..."
    
    # Remove existing directory if present
    if [ -d "$HOME/atmn-2.0" ]; then
        echo "Removing existing installation..."
        rm -rf "$HOME/atmn-2.0"
    fi
    
    # Clone repository
    git clone --branch "$ATMN_BRANCH" "$ATMN_REPO" "$HOME/atmn-2.0"
    cd "$HOME/atmn-2.0"
    
    echo -e "${GREEN}✓ Repository cloned${NC}"
    echo ""
}

################################################################################
# Build ATMN Binaries
################################################################################
build_binaries() {
    echo -e "${YELLOW}[5/10]${NC} Building ATMN binaries..."
    
    cd "$HOME/atmn-2.0/atmn-core"
    
    # Build core binaries
    echo "Building atmn-core..."
    cargo build --release --bin atmn-miner 2>&1 | tail -5
    cargo build --release --bin check-balance 2>&1 | tail -5
    cargo build --release --bin create-transaction 2>&1 | tail -5
    
    # Build P2P node
    cd "$HOME/atmn-2.0/atmn-node"
    echo "Building atmn-node..."
    cargo build --release 2>&1 | tail -5
    
    # Build APIs if full node
    if [ "$NODE_TYPE" = "$NODE_TYPE_FULL" ]; then
        cd "$HOME/atmn-2.0/atmn-rosetta"
        echo "Building atmn-rosetta..."
        cargo build --release 2>&1 | tail -5
        
        cd "$HOME/atmn-2.0/atmn-api"
        echo "Building atmn-api..."
        cargo build --release 2>&1 | tail -5
    fi
    
    echo -e "${GREEN}✓ Binaries built successfully${NC}"
    echo ""
}

################################################################################
# Initialize Database
################################################################################
initialize_database() {
    echo -e "${YELLOW}[6/10]${NC} Initializing blockchain database..."
    
    # Create data directory
    mkdir -p "$HOME/atmn-data"
    
    # Initialize genesis block
    cd "$HOME/atmn-2.0/atmn-core"
    cargo run --release --bin atmn-miner -- \
        --data-dir "$HOME/atmn-data" \
        --verify-genesis
    
    echo -e "${GREEN}✓ Database initialized${NC}"
    echo ""
}

################################################################################
# Configure Systemd Services
################################################################################
configure_services() {
    echo -e "${YELLOW}[7/10]${NC} Configuring systemd services..."
    
    # P2P Node Service
    sudo tee /etc/systemd/system/atmn-node.service > /dev/null <<EOF
[Unit]
Description=ATMN P2P Node
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME/atmn-2.0/atmn-node
ExecStart=$HOME/atmn-2.0/atmn-node/target/release/atmn-node --port 19000 --db $HOME/atmn-data/node.db
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

    # Mining Service (if full node)
    if [ "$NODE_TYPE" = "$NODE_TYPE_FULL" ]; then
        sudo tee /etc/systemd/system/atmn-miner.service > /dev/null <<EOF
[Unit]
Description=ATMN Miner
After=network.target atmn-node.service

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME/atmn-2.0/atmn-core
ExecStart=$HOME/atmn-2.0/atmn-core/target/release/atmn-miner --data-dir $HOME/atmn-data --threads 4
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

        # API Services
        sudo tee /etc/systemd/system/atmn-rosetta.service > /dev/null <<EOF
[Unit]
Description=ATMN Rosetta API
After=network.target atmn-node.service

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME/atmn-2.0/atmn-rosetta
ExecStart=$HOME/atmn-2.0/atmn-rosetta/target/release/atmn-rosetta
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF
    fi
    
    # Reload systemd
    sudo systemctl daemon-reload
    
    echo -e "${GREEN}✓ Systemd services configured${NC}"
    echo ""
}

################################################################################
# Start Services
################################################################################
start_services() {
    echo -e "${YELLOW}[8/10]${NC} Starting services..."
    
    # Start P2P node
    sudo systemctl enable atmn-node
    sudo systemctl start atmn-node
    sleep 3
    
    # Start miner and APIs if full node
    if [ "$NODE_TYPE" = "$NODE_TYPE_FULL" ]; then
        sudo systemctl enable atmn-miner
        sudo systemctl start atmn-miner
        sleep 2
        
        sudo systemctl enable atmn-rosetta
        sudo systemctl start atmn-rosetta
        sleep 2
    fi
    
    echo -e "${GREEN}✓ Services started${NC}"
    echo ""
}

################################################################################
# Verify Deployment
################################################################################
verify_deployment() {
    echo -e "${YELLOW}[9/10]${NC} Verifying deployment..."
    
    # Check P2P node
    if sudo systemctl is-active --quiet atmn-node; then
        echo -e "${GREEN}✓ P2P Node: Running${NC}"
    else
        echo -e "${RED}✗ P2P Node: Failed${NC}"
    fi
    
    # Check miner if full node
    if [ "$NODE_TYPE" = "$NODE_TYPE_FULL" ]; then
        if sudo systemctl is-active --quiet atmn-miner; then
            echo -e "${GREEN}✓ Miner: Running${NC}"
        else
            echo -e "${RED}✗ Miner: Failed${NC}"
        fi
        
        if sudo systemctl is-active --quiet atmn-rosetta; then
            echo -e "${GREEN}✓ Rosetta API: Running${NC}"
        else
            echo -e "${RED}✗ Rosetta API: Failed${NC}"
        fi
    fi
    
    echo ""
}

################################################################################
# Display Summary
################################################################################
display_summary() {
    echo -e "${YELLOW}[10/10]${NC} Deployment complete!"
    echo ""
    echo -e "${BLUE}============================================${NC}"
    echo -e "${BLUE}DEPLOYMENT SUMMARY${NC}"
    echo -e "${BLUE}============================================${NC}"
    echo "Provider: $PROVIDER"
    echo "Region: $REGION"
    echo "Node Type: $NODE_TYPE"
    echo "Data Directory: $HOME/atmn-data"
    echo "Installation Directory: $HOME/atmn-2.0"
    echo ""
    echo -e "${YELLOW}Service Management:${NC}"
    echo "  Status:  sudo systemctl status atmn-node"
    echo "  Logs:    sudo journalctl -u atmn-node -f"
    echo "  Restart: sudo systemctl restart atmn-node"
    echo ""
    if [ "$NODE_TYPE" = "$NODE_TYPE_FULL" ]; then
        echo -e "${YELLOW}API Endpoints:${NC}"
        echo "  Rosetta: http://$(hostname -I | awk '{print $1}'):8080"
        echo "  Mining:  http://$(hostname -I | awk '{print $1}'):8000"
        echo ""
    fi
    echo -e "${YELLOW}P2P Connection:${NC}"
    echo "  Bootstrap: $(hostname -I | awk '{print $1}'):19000"
    echo ""
    echo -e "${GREEN}Node deployed successfully!${NC}"
    echo -e "${BLUE}============================================${NC}"
}

################################################################################
# Main Execution
################################################################################
main() {
    check_system_requirements
    install_dependencies
    configure_firewall
    clone_repository
    build_binaries
    initialize_database
    configure_services
    start_services
    verify_deployment
    display_summary
}

main
