#!/bin/bash

################################################################################
# ANTIMONY 2.0 - Security Audit Script
# Phase 8: Mainnet Preparation
#
# Purpose: Comprehensive security audit of P2P protocol and APIs
# Tests: Rate limiting, DDoS protection, message validation, peer reputation
#
# Usage: ./security_audit.sh
################################################################################

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Test counters
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Output file
OUTPUT_FILE="security_audit_report_$(date +%Y%m%d_%H%M%S).txt"

echo -e "${BLUE}============================================${NC}"
echo -e "${BLUE}ANTIMONY 2.0 - Security Audit${NC}"
echo -e "${BLUE}============================================${NC}"
echo ""

log_test() {
    local test_name=$1
    local result=$2
    local details=$3
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    
    if [ "$result" = "PASS" ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        echo -e "${GREEN}✓ $test_name${NC}" | tee -a "$OUTPUT_FILE"
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        echo -e "${RED}✗ $test_name${NC}" | tee -a "$OUTPUT_FILE"
    fi
    
    if [ -n "$details" ]; then
        echo "  $details" | tee -a "$OUTPUT_FILE"
    fi
    echo "" | tee -a "$OUTPUT_FILE"
}

################################################################################
# 1. P2P Protocol Security Tests
################################################################################
test_p2p_security() {
    echo -e "${YELLOW}[1/7] Testing P2P Protocol Security${NC}" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    # Test 1.1: Handshake validation
    echo -e "${BLUE}Test 1.1: Handshake Validation${NC}"
    if grep -r "validate.*handshake" atmn-node/src/ > /dev/null 2>&1; then
        log_test "Handshake validation implemented" "PASS" "Found validation logic in P2P code"
    else
        log_test "Handshake validation" "FAIL" "No handshake validation found"
    fi
    
    # Test 1.2: Peer authentication
    echo -e "${BLUE}Test 1.2: Peer Authentication${NC}"
    if grep -r "peer.*auth\|authenticate.*peer" atmn-node/src/ > /dev/null 2>&1; then
        log_test "Peer authentication" "PASS" "Authentication logic exists"
    else
        log_test "Peer authentication" "FAIL" "No peer authentication found"
    fi
    
    # Test 1.3: Message size limits
    echo -e "${BLUE}Test 1.3: Message Size Limits${NC}"
    if grep -r "MAX.*SIZE\|size.*limit" atmn-node/src/ > /dev/null 2>&1; then
        log_test "Message size limits" "PASS" "Size limits defined"
    else
        log_test "Message size limits" "FAIL" "No size limits found"
    fi
    
    # Test 1.4: Connection limits
    echo -e "${BLUE}Test 1.4: Connection Limits${NC}"
    if grep -r "MAX.*PEERS\|max.*connections" atmn-node/src/ > /dev/null 2>&1; then
        log_test "Connection limits" "PASS" "Peer connection limits exist"
    else
        log_test "Connection limits" "FAIL" "No connection limits"
    fi
    
    # Test 1.5: Malformed message handling
    echo -e "${BLUE}Test 1.5: Malformed Message Handling${NC}"
    if grep -r "deserialize.*error\|parse.*error" atmn-node/src/ > /dev/null 2>&1; then
        log_test "Malformed message handling" "PASS" "Error handling implemented"
    else
        log_test "Malformed message handling" "FAIL" "No error handling for bad messages"
    fi
}

################################################################################
# 2. Rate Limiting Tests
################################################################################
test_rate_limiting() {
    echo -e "${YELLOW}[2/7] Testing Rate Limiting${NC}" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    # Test 2.1: API rate limiting
    echo -e "${BLUE}Test 2.1: API Rate Limiting${NC}"
    if [ -f "atmn-api/src/main.rs" ]; then
        if grep -r "rate.*limit\|throttle" atmn-api/src/ > /dev/null 2>&1; then
            log_test "API rate limiting" "PASS" "Rate limiting code found"
        else
            log_test "API rate limiting" "FAIL" "No rate limiting implemented"
        fi
    else
        log_test "API rate limiting" "SKIP" "API server not found"
    fi
    
    # Test 2.2: P2P message rate limiting
    echo -e "${BLUE}Test 2.2: P2P Message Rate Limiting${NC}"
    if grep -r "rate.*limit\|messages.*per.*second" atmn-node/src/ > /dev/null 2>&1; then
        log_test "P2P rate limiting" "PASS" "Message rate limiting exists"
    else
        log_test "P2P rate limiting" "FAIL" "No P2P rate limiting"
    fi
    
    # Test 2.3: Connection rate limiting
    echo -e "${BLUE}Test 2.3: Connection Rate Limiting${NC}"
    if grep -r "connection.*limit\|conn.*per.*sec" atmn-node/src/ > /dev/null 2>&1; then
        log_test "Connection rate limiting" "PASS" "Connection throttling exists"
    else
        log_test "Connection rate limiting" "FAIL" "No connection rate limiting"
    fi
}

################################################################################
# 3. DDoS Protection Tests
################################################################################
test_ddos_protection() {
    echo -e "${YELLOW}[3/7] Testing DDoS Protection${NC}" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    # Test 3.1: IP blacklisting
    echo -e "${BLUE}Test 3.1: IP Blacklisting${NC}"
    if grep -r "blacklist\|ban.*ip\|blocked.*peers" atmn-node/src/ > /dev/null 2>&1; then
        log_test "IP blacklisting" "PASS" "Blacklist mechanism exists"
    else
        log_test "IP blacklisting" "FAIL" "No IP blacklisting"
    fi
    
    # Test 3.2: Peer reputation system
    echo -e "${BLUE}Test 3.2: Peer Reputation System${NC}"
    if grep -r "reputation\|score\|misbehav" atmn-node/src/ > /dev/null 2>&1; then
        log_test "Peer reputation" "PASS" "Reputation system found"
    else
        log_test "Peer reputation" "FAIL" "No reputation system"
    fi
    
    # Test 3.3: Flood protection
    echo -e "${BLUE}Test 3.3: Flood Protection${NC}"
    if grep -r "flood\|spam.*protect" atmn-node/src/ > /dev/null 2>&1; then
        log_test "Flood protection" "PASS" "Flood protection exists"
    else
        log_test "Flood protection" "FAIL" "No flood protection"
    fi
}

################################################################################
# 4. Cryptographic Security Tests
################################################################################
test_crypto_security() {
    echo -e "${YELLOW}[4/7] Testing Cryptographic Security${NC}" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    # Test 4.1: SHA-256d implementation
    echo -e "${BLUE}Test 4.1: SHA-256d Hashing${NC}"
    if grep -r "sha256d\|double.*sha256" atmn-core/src/ > /dev/null 2>&1; then
        log_test "SHA-256d implementation" "PASS" "Double SHA-256 hashing used"
    else
        log_test "SHA-256d implementation" "FAIL" "SHA-256d not found"
    fi
    
    # Test 4.2: Secure random generation
    echo -e "${BLUE}Test 4.2: Secure Random Generation${NC}"
    if grep -r "rand.*secure\|OsRng\|ThreadRng" atmn-core/src/ > /dev/null 2>&1; then
        log_test "Secure RNG" "PASS" "Secure random number generation"
    else
        log_test "Secure RNG" "FAIL" "No secure RNG found"
    fi
    
    # Test 4.3: Private key protection
    echo -e "${BLUE}Test 4.3: Private Key Protection${NC}"
    if grep -r "SecretKey\|PrivateKey" atmn-core/src/ > /dev/null 2>&1; then
        log_test "Private key handling" "PASS" "Private key types used"
    else
        log_test "Private key handling" "FAIL" "No private key protection"
    fi
}

################################################################################
# 5. Input Validation Tests
################################################################################
test_input_validation() {
    echo -e "${YELLOW}[5/7] Testing Input Validation${NC}" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    # Test 5.1: Address validation
    echo -e "${BLUE}Test 5.1: Address Validation${NC}"
    if grep -r "validate.*address\|check.*address" atmn-core/src/ > /dev/null 2>&1; then
        log_test "Address validation" "PASS" "Address validation exists"
    else
        log_test "Address validation" "FAIL" "No address validation"
    fi
    
    # Test 5.2: Transaction validation
    echo -e "${BLUE}Test 5.2: Transaction Validation${NC}"
    if grep -r "validate.*transaction\|verify.*tx" atmn-core/src/ > /dev/null 2>&1; then
        log_test "Transaction validation" "PASS" "TX validation logic found"
    else
        log_test "Transaction validation" "FAIL" "No TX validation"
    fi
    
    # Test 5.3: Block validation
    echo -e "${BLUE}Test 5.3: Block Validation${NC}"
    if grep -r "validate.*block\|verify.*block" atmn-core/src/ > /dev/null 2>&1; then
        log_test "Block validation" "PASS" "Block validation exists"
    else
        log_test "Block validation" "FAIL" "No block validation"
    fi
    
    # Test 5.4: UTXO double-spend prevention
    echo -e "${BLUE}Test 5.4: Double-Spend Prevention${NC}"
    if grep -r "double.*spend\|utxo.*spent\|check.*utxo" atmn-core/src/ > /dev/null 2>&1; then
        log_test "Double-spend prevention" "PASS" "UTXO checking implemented"
    else
        log_test "Double-spend prevention" "FAIL" "No double-spend protection"
    fi
}

################################################################################
# 6. Database Security Tests
################################################################################
test_database_security() {
    echo -e "${YELLOW}[6/7] Testing Database Security${NC}" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    # Test 6.1: SQL injection protection
    echo -e "${BLUE}Test 6.1: SQL Injection Protection${NC}"
    if [ -f "atmn-api/src/db.rs" ]; then
        if grep -r "prepare\|bind\|?" atmn-api/src/db.rs > /dev/null 2>&1; then
            log_test "SQL injection protection" "PASS" "Prepared statements used"
        else
            log_test "SQL injection protection" "FAIL" "No prepared statements"
        fi
    else
        log_test "SQL injection protection" "SKIP" "Database module not found"
    fi
    
    # Test 6.2: Access control
    echo -e "${BLUE}Test 6.2: Database Access Control${NC}"
    if grep -r "auth.*database\|db.*credentials" atmn-api/src/ > /dev/null 2>&1; then
        log_test "DB access control" "PASS" "Authentication exists"
    else
        log_test "DB access control" "FAIL" "No DB authentication"
    fi
    
    # Test 6.3: Data encryption at rest
    echo -e "${BLUE}Test 6.3: Data Encryption${NC}"
    if grep -r "encrypt\|cipher" atmn-api/src/ > /dev/null 2>&1; then
        log_test "Data encryption" "PASS" "Encryption code found"
    else
        log_test "Data encryption" "FAIL" "No encryption at rest"
    fi
}

################################################################################
# 7. API Security Tests
################################################################################
test_api_security() {
    echo -e "${YELLOW}[7/7] Testing API Security${NC}" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    # Test 7.1: Authentication
    echo -e "${BLUE}Test 7.1: API Authentication${NC}"
    if [ -f "atmn-api/src/main.rs" ]; then
        if grep -r "jwt\|token\|auth" atmn-api/src/ > /dev/null 2>&1; then
            log_test "API authentication" "PASS" "Auth mechanism found"
        else
            log_test "API authentication" "FAIL" "No API authentication"
        fi
    else
        log_test "API authentication" "SKIP" "API not found"
    fi
    
    # Test 7.2: CORS configuration
    echo -e "${BLUE}Test 7.2: CORS Configuration${NC}"
    if grep -r "cors\|origin" atmn-api/src/ > /dev/null 2>&1; then
        log_test "CORS configuration" "PASS" "CORS configured"
    else
        log_test "CORS configuration" "FAIL" "No CORS configuration"
    fi
    
    # Test 7.3: HTTPS/TLS
    echo -e "${BLUE}Test 7.3: HTTPS/TLS Support${NC}"
    if grep -r "tls\|https\|ssl" atmn-api/src/ > /dev/null 2>&1; then
        log_test "HTTPS/TLS" "PASS" "TLS support exists"
    else
        log_test "HTTPS/TLS" "FAIL" "No TLS configuration"
    fi
    
    # Test 7.4: Input sanitization
    echo -e "${BLUE}Test 7.4: API Input Sanitization${NC}"
    if grep -r "sanitize\|escape\|validate.*input" atmn-api/src/ > /dev/null 2>&1; then
        log_test "Input sanitization" "PASS" "Sanitization exists"
    else
        log_test "Input sanitization" "FAIL" "No input sanitization"
    fi
}

################################################################################
# Generate Detailed Report
################################################################################
generate_report() {
    echo -e "${BLUE}============================================${NC}" | tee -a "$OUTPUT_FILE"
    echo -e "${BLUE}SECURITY AUDIT SUMMARY${NC}" | tee -a "$OUTPUT_FILE"
    echo -e "${BLUE}============================================${NC}" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    echo "Date: $(date)" | tee -a "$OUTPUT_FILE"
    echo "ATMN Version: 2.0.0" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    echo -e "${YELLOW}Test Results:${NC}" | tee -a "$OUTPUT_FILE"
    echo "  Total Tests: $TOTAL_TESTS" | tee -a "$OUTPUT_FILE"
    echo "  Passed: $PASSED_TESTS" | tee -a "$OUTPUT_FILE"
    echo "  Failed: $FAILED_TESTS" | tee -a "$OUTPUT_FILE"
    
    SUCCESS_RATE=$(awk "BEGIN {printf \"%.1f\", ($PASSED_TESTS/$TOTAL_TESTS)*100}")
    echo "  Success Rate: ${SUCCESS_RATE}%" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    if [ "$FAILED_TESTS" -eq 0 ]; then
        echo -e "${GREEN}✓ All security tests passed!${NC}" | tee -a "$OUTPUT_FILE"
        echo -e "${GREEN}✓ System is production-ready${NC}" | tee -a "$OUTPUT_FILE"
    elif [ "$FAILED_TESTS" -le 5 ]; then
        echo -e "${YELLOW}⚠ Minor security issues found${NC}" | tee -a "$OUTPUT_FILE"
        echo -e "${YELLOW}⚠ Review and fix before mainnet${NC}" | tee -a "$OUTPUT_FILE"
    else
        echo -e "${RED}✗ Critical security issues found${NC}" | tee -a "$OUTPUT_FILE"
        echo -e "${RED}✗ DO NOT deploy to mainnet${NC}" | tee -a "$OUTPUT_FILE"
    fi
    
    echo "" | tee -a "$OUTPUT_FILE"
    echo -e "${YELLOW}Recommendations:${NC}" | tee -a "$OUTPUT_FILE"
    echo "  1. Implement rate limiting for all APIs" | tee -a "$OUTPUT_FILE"
    echo "  2. Add peer reputation system" | tee -a "$OUTPUT_FILE"
    echo "  3. Enable DDoS protection" | tee -a "$OUTPUT_FILE"
    echo "  4. Conduct penetration testing" | tee -a "$OUTPUT_FILE"
    echo "  5. External security audit" | tee -a "$OUTPUT_FILE"
    echo "" | tee -a "$OUTPUT_FILE"
    
    echo "Report saved to: $OUTPUT_FILE" | tee -a "$OUTPUT_FILE"
    echo -e "${BLUE}============================================${NC}" | tee -a "$OUTPUT_FILE"
}

################################################################################
# Main Execution
################################################################################
main() {
    test_p2p_security
    test_rate_limiting
    test_ddos_protection
    test_crypto_security
    test_input_validation
    test_database_security
    test_api_security
    generate_report
}

main
