#!/bin/bash
# API Testing Suite - Comprehensive blockchain API tests

echo "🧪 ANTIMONY BLOCKCHAIN API TEST SUITE"
echo "======================================"
echo ""

API_URL="http://localhost:8000"

# Test 1: Health Check
echo "1️⃣ Health Check:"
curl -s "$API_URL/health" | jq '.'
echo ""

# Test 2: Blockchain Stats
echo "2️⃣ Blockchain Statistics:"
curl -s "$API_URL/api/blockchain/stats" | jq '.'
echo ""

# Test 3: Latest Blocks
echo "3️⃣ Latest Blocks (top 3):"
curl -s "$API_URL/api/blocks/latest" | jq '.blocks[0:3] | .[] | {height, hash: .hash[0:16], nonce}'
echo ""

# Test 4: Specific Block by Height
echo "4️⃣ Get Block #5:"
curl -s "$API_URL/api/blocks/5" | jq '.block | {height, hash: .hash[0:16], nonce, timestamp}'
echo ""

# Test 5: Block Range
echo "5️⃣ Block Range (1-5):"
curl -s "$API_URL/api/blocks/range?start=1&end=5" | jq '.blocks | length'
echo " blocks returned"
echo ""

# Test 6: Miner Address Balance
echo "6️⃣ Miner Address Balance:"
curl -s "$API_URL/api/address/ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178/balance" | jq '.'
echo ""

# Test 7: Address Transactions
echo "7️⃣ Address Transactions:"
curl -s "$API_URL/api/address/ATMN_1e6df34f5f50ff6b581b827c5e9dc5b5b787e178/transactions" | jq '{count: .count, transactions: .transactions[0:3]}'
echo ""

# Test 8: Mining Template (requires authentication)
echo "8️⃣ Get Mining Template:"
curl -s "$API_URL/api/mining/template" | jq '. | if .error then {error} else {success, height: .template.height, difficulty: .template.difficulty_bits} end'
echo ""

echo "✅ API Testing Complete!"
echo ""
echo "Summary:"
echo "--------"
curl -s "$API_URL/api/blockchain/stats" | jq -r '"Blocks: \(.total_blocks) | Height: \(.current_height) | Network: \(.network)"'
