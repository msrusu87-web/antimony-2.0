# ATMN Mining API Documentation

## Overview
Complete REST API for mining control, block submission, and mempool monitoring for the ATMN blockchain.

## Base URL
```
http://127.0.0.1:8000/api
```

## API Endpoints

### 1. Start Mining
Start CPU mining with specified miner address.

**Endpoint:** `POST /mining/start`

**Request Body:**
```json
{
  "miner_address": "ATMN_TEST123",
  "threads": 2
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Mining started",
  "miner_address": "ATMN_TEST123",
  "threads": 2
}
```

**Response (Already Mining):**
```json
{
  "error": "ALREADY_MINING",
  "message": "Mining is already active"
}
```

**Response (Invalid Address):**
```json
{
  "error": "INVALID_ADDRESS",
  "message": "Miner address must start with ATMN_"
}
```

**Example:**
```bash
curl -X POST http://127.0.0.1:8000/api/mining/start \
  -H "Content-Type: application/json" \
  -d '{"miner_address":"ATMN_TEST123","threads":2}'
```

---

### 2. Stop Mining
Stop active mining process.

**Endpoint:** `POST /mining/stop`

**Request Body:** None

**Response (Success):**
```json
{
  "success": true,
  "message": "Mining stopped",
  "blocks_found": 5,
  "miner_address": "ATMN_TEST123"
}
```

**Response (Not Mining):**
```json
{
  "error": "NOT_MINING",
  "message": "Mining is not active"
}
```

**Example:**
```bash
curl -X POST http://127.0.0.1:8000/api/mining/stop
```

---

### 3. Get Mining Status
Get current mining state and statistics.

**Endpoint:** `GET /mining/status`

**Request Body:** None

**Response:**
```json
{
  "is_mining": true,
  "miner_address": "ATMN_TEST123",
  "blocks_found": 3,
  "hash_rate": 125000.5,
  "uptime_seconds": 3600
}
```

**Fields:**
- `is_mining` (bool): Whether mining is currently active
- `miner_address` (string|null): Address receiving mining rewards
- `blocks_found` (int): Number of blocks found this session
- `hash_rate` (float): Current hash rate in H/s
- `uptime_seconds` (int|null): Time since mining started

**Example:**
```bash
curl http://127.0.0.1:8000/api/mining/status
```

---

### 4. Get Block Template
Get template for mining next block.

**Endpoint:** `GET /mining/template`

**Request Body:** None

**Response:**
```json
{
  "version": 1,
  "prev_block_hash": "000000000000000000000000000000000000000000000000000000000000abcd",
  "merkle_root": "0000000000000000000000000000000000000000000000000000000000000000",
  "timestamp": 1764906289,
  "bits": 486604799,
  "height": 1234,
  "transactions": []
}
```

**Fields:**
- `version` (int): Block version number
- `prev_block_hash` (string): Previous block hash (64 hex chars)
- `merkle_root` (string): Merkle tree root of transactions (64 hex chars)
- `timestamp` (int): Current Unix timestamp
- `bits` (int): Difficulty target (compact format)
- `height` (int): Next block height
- `transactions` (array): Transaction IDs to include

**Example:**
```bash
curl http://127.0.0.1:8000/api/mining/template
```

---

### 5. Submit Block
Submit mined block for validation.

**Endpoint:** `POST /mining/submit`

**Request Body:**
```json
{
  "block_data": "base64_encoded_block_data",
  "nonce": 12345678,
  "hash": "0000000000000000000000000000000000000000000000000000000000001234"
}
```

**Response (Success):**
```json
{
  "success": true,
  "message": "Block accepted",
  "hash": "0000000000000000000000000000000000000000000000000000000000001234",
  "height": 1234,
  "reward": 50.0
}
```

**Response (Invalid Hash):**
```json
{
  "error": "INVALID_HASH",
  "message": "Block hash must be 64 hex characters"
}
```

**Example:**
```bash
curl -X POST http://127.0.0.1:8000/api/mining/submit \
  -H "Content-Type: application/json" \
  -d '{"block_data":"test","nonce":12345,"hash":"0000000000000000000000000000000000000000000000000000000000001234"}'
```

---

### 6. Get Mempool Statistics
Get transaction mempool statistics.

**Endpoint:** `GET /mempool/stats`

**Request Body:** None

**Response:**
```json
{
  "transaction_count": 42,
  "total_size_bytes": 15000,
  "max_size": 50000,
  "min_fee_per_byte": 1
}
```

**Fields:**
- `transaction_count` (int): Number of pending transactions
- `total_size_bytes` (int): Total size of pending transactions
- `max_size` (int): Maximum mempool capacity
- `min_fee_per_byte` (int): Minimum fee requirement (satoshis)

**Example:**
```bash
curl http://127.0.0.1:8000/api/mempool/stats
```

---

## Legacy Pool Endpoints

These endpoints maintain backward compatibility with existing mining pool implementations.

### 7. Register Worker
Register a mining pool worker.

**Endpoint:** `POST /mining/worker/register`

**Request Body:**
```json
{
  "worker_id": "worker_001",
  "miner_address": "ATMN_MINER_ADDRESS"
}
```

---

### 8. Get Workers
List all registered workers.

**Endpoint:** `GET /mining/workers`

---

### 9. Get Pool Stats
Get mining pool statistics.

**Endpoint:** `GET /mining/stats`

---

### 10. Get Payouts
Get mining payouts for address.

**Endpoint:** `GET /mining/payouts/{address}`

---

## Error Responses

All error responses follow this format:
```json
{
  "error": "ERROR_CODE",
  "message": "Human readable error message"
}
```

### Common Error Codes:
- `ALREADY_MINING` - Mining already active
- `NOT_MINING` - Mining not active
- `INVALID_ADDRESS` - Address format invalid
- `INVALID_HASH` - Block hash format invalid
- `DATABASE_ERROR` - Database operation failed
- `INTERNAL_ERROR` - Internal server error

---

## Mining Workflow

### Starting Mining
1. Call `POST /mining/start` with miner address
2. Server validates address format (must start with "ATMN_")
3. Mining state is set to active
4. TODO: Background mining thread starts

### Monitoring Mining
1. Call `GET /mining/status` to check if mining is active
2. Monitor `hash_rate` for performance
3. Check `blocks_found` for success count
4. Use `uptime_seconds` for session duration

### External Mining
1. Call `GET /mining/template` to get block header
2. Mine block externally (iterate nonce)
3. Call `POST /mining/submit` when solution found
4. Server validates proof-of-work (TODO)
5. Block added to blockchain (TODO)

### Mempool Monitoring
1. Call `GET /mempool/stats` for pending transactions
2. Use in block template generation
3. Monitor `transaction_count` for mempool size
4. Check `total_size_bytes` for memory usage

---

## Implementation Status

### ✅ Completed
- All 6 new API endpoints implemented
- Mining state management (start/stop/status)
- Block template generation
- Mempool statistics endpoint
- Error handling and validation
- REST API documentation

### 🔄 TODO
- Connect to actual miner module (atmn-core)
- Start background mining threads
- Implement proof-of-work validation
- Connect to real mempool
- Block submission to blockchain
- Mining statistics updates
- Multi-threaded mining support

---

## Technical Details

### Dependencies
- `actix-web` 4.4 - HTTP server framework
- `tokio` 1.35 - Async runtime
- `serde/serde_json` - Serialization
- `sqlx` 0.7 - Database access
- `lazy_static` 1.4 - Global state
- `chrono` 0.4 - Timestamps

### State Management
Mining state is currently managed using a global `Arc<Mutex<MinerState>>` via `lazy_static!`. In production, this should be replaced with:
- Dedicated state management crate (e.g., `tokio::sync::RwLock`)
- Database-backed persistent state
- Distributed state for multi-instance deployments

### Performance
- All endpoints respond in <10ms (current implementation)
- State access is thread-safe via Mutex
- Database queries use connection pooling (max 5 connections)
- Block template generation includes latest blockchain state

---

## Testing

### Test All Endpoints
```bash
# Start mining
curl -X POST http://127.0.0.1:8000/api/mining/start \
  -H "Content-Type: application/json" \
  -d '{"miner_address":"ATMN_TEST","threads":2}'

# Check status
curl http://127.0.0.1:8000/api/mining/status

# Get template
curl http://127.0.0.1:8000/api/mining/template

# Submit block
curl -X POST http://127.0.0.1:8000/api/mining/submit \
  -H "Content-Type: application/json" \
  -d '{"block_data":"test","nonce":12345,"hash":"0000000000000000000000000000000000000000000000000000000000001234"}'

# Check mempool
curl http://127.0.0.1:8000/api/mempool/stats

# Stop mining
curl -X POST http://127.0.0.1:8000/api/mining/stop
```

---

## Security Considerations

1. **Authentication**: Currently no authentication required. Add API keys or JWT tokens for production.
2. **Rate Limiting**: No rate limiting implemented. Should limit mining start/stop calls.
3. **Address Validation**: Basic validation (ATMN_ prefix). Should verify full address format.
4. **Block Validation**: Currently accepts any hash. Must implement full PoW verification.
5. **CORS**: Currently allows all origins. Restrict in production.

---

## Next Steps

1. **Phase 3.2**: Connect mining API to atmn-core miner module
2. **Phase 3.3**: Implement actual background mining threads
3. **Phase 3.4**: Add proof-of-work validation
4. **Phase 3.5**: Connect mempool to API
5. **Phase 3.6**: Implement mining statistics tracking
6. **Phase 3.7**: Add WebSocket support for real-time updates

