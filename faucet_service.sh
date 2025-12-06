#!/bin/bash

################################################################################
# ANTIMONY 2.0 - Public Testnet Faucet Service
# Dispenses test ATMN for developers and testers
################################################################################

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
FAUCET_ADDRESS="atmn1qfaucetqfaucetqfaucetqfaucetqfaucet5n3xd2"
FAUCET_AMOUNT="10.0"  # 10 ATMN per request
FAUCET_COOLDOWN=3600  # 1 hour cooldown per address
FAUCET_DAILY_LIMIT=100  # Max 100 ATMN per day per IP
DB_FILE="/home/ubuntu/faucet-requests.db"

# Initialize database
init_database() {
    sqlite3 "$DB_FILE" <<EOF
CREATE TABLE IF NOT EXISTS requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address TEXT NOT NULL,
    ip_address TEXT NOT NULL,
    amount REAL NOT NULL,
    timestamp INTEGER NOT NULL,
    tx_hash TEXT
);

CREATE INDEX IF NOT EXISTS idx_address ON requests(address);
CREATE INDEX IF NOT EXISTS idx_ip ON requests(ip_address);
CREATE INDEX IF NOT EXISTS idx_timestamp ON requests(timestamp);
EOF
}

# Check if address can request
can_request() {
    local address=$1
    local ip=$2
    local now=$(date +%s)
    local cooldown_time=$((now - FAUCET_COOLDOWN))
    
    # Check address cooldown
    local last_request=$(sqlite3 "$DB_FILE" \
        "SELECT MAX(timestamp) FROM requests WHERE address='$address'")
    
    if [ -n "$last_request" ] && [ "$last_request" -gt "$cooldown_time" ]; then
        local wait_time=$((last_request + FAUCET_COOLDOWN - now))
        echo "Address must wait $wait_time seconds"
        return 1
    fi
    
    # Check IP daily limit
    local day_ago=$((now - 86400))
    local daily_total=$(sqlite3 "$DB_FILE" \
        "SELECT COALESCE(SUM(amount), 0) FROM requests WHERE ip_address='$ip' AND timestamp > $day_ago")
    
    if (( $(echo "$daily_total >= $FAUCET_DAILY_LIMIT" | bc -l) )); then
        echo "IP daily limit exceeded"
        return 1
    fi
    
    return 0
}

# Process faucet request
process_request() {
    local address=$1
    local ip=$2
    
    echo -e "${BLUE}Processing faucet request${NC}"
    echo "Address: $address"
    echo "IP: $ip"
    echo ""
    
    # Validate address
    if [[ ! $address =~ ^atmn1[a-z0-9]{38,58}$ ]]; then
        echo -e "${RED}Invalid ATMN address format${NC}"
        return 1
    fi
    
    # Check rate limits
    if ! can_request "$address" "$ip"; then
        return 1
    fi
    
    # Create transaction
    echo -e "${YELLOW}Creating transaction...${NC}"
    
    # NOTE: This would call the actual transaction creation utility
    # For now, we simulate it
    local tx_hash="0x$(openssl rand -hex 32)"
    
    # Record request
    local now=$(date +%s)
    sqlite3 "$DB_FILE" \
        "INSERT INTO requests (address, ip_address, amount, timestamp, tx_hash) \
         VALUES ('$address', '$ip', $FAUCET_AMOUNT, $now, '$tx_hash')"
    
    echo -e "${GREEN}✓ Faucet request successful!${NC}"
    echo "Amount: $FAUCET_AMOUNT ATMN"
    echo "TX Hash: $tx_hash"
    echo ""
    
    return 0
}

# Get faucet statistics
get_statistics() {
    echo -e "${BLUE}Faucet Statistics${NC}"
    echo "================================"
    
    local total_requests=$(sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM requests")
    local total_distributed=$(sqlite3 "$DB_FILE" "SELECT COALESCE(SUM(amount), 0) FROM requests")
    local unique_addresses=$(sqlite3 "$DB_FILE" "SELECT COUNT(DISTINCT address) FROM requests")
    local last_24h=$(sqlite3 "$DB_FILE" "SELECT COUNT(*) FROM requests WHERE timestamp > $(date -d '24 hours ago' +%s)")
    
    echo "Total Requests: $total_requests"
    echo "Total Distributed: $total_distributed ATMN"
    echo "Unique Addresses: $unique_addresses"
    echo "Last 24 Hours: $last_24h requests"
    echo ""
}

# Main execution
case "${1:-help}" in
    init)
        init_database
        echo -e "${GREEN}✓ Faucet database initialized${NC}"
        ;;
    
    request)
        if [ -z "$2" ] || [ -z "$3" ]; then
            echo "Usage: $0 request <address> <ip>"
            exit 1
        fi
        init_database
        process_request "$2" "$3"
        ;;
    
    stats)
        init_database
        get_statistics
        ;;
    
    *)
        echo "ATMN Testnet Faucet"
        echo ""
        echo "Usage: $0 <command> [args]"
        echo ""
        echo "Commands:"
        echo "  init                Initialize faucet database"
        echo "  request <addr> <ip> Process faucet request"
        echo "  stats               Show faucet statistics"
        echo ""
        ;;
esac
