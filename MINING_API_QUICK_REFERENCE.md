# ATMN Mining API - Quick Reference

## Base URL
```
http://127.0.0.1:8000/api
```

## Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/mining/start` | Start mining |
| POST | `/mining/stop` | Stop mining |
| GET | `/mining/status` | Get mining status |
| GET | `/mining/template` | Get block template |
| POST | `/mining/submit` | Submit mined block |
| GET | `/mempool/stats` | Get mempool stats |

## Quick Examples

### Start Mining
```bash
curl -X POST http://127.0.0.1:8000/api/mining/start \
  -H "Content-Type: application/json" \
  -d '{"miner_address":"ATMN_YOUR_ADDRESS","threads":2}'
```

### Check Status
```bash
curl http://127.0.0.1:8000/api/mining/status | jq .
```

### Get Template
```bash
curl http://127.0.0.1:8000/api/mining/template | jq .
```

### Submit Block
```bash
curl -X POST http://127.0.0.1:8000/api/mining/submit \
  -H "Content-Type: application/json" \
  -d '{"block_data":"...","nonce":12345,"hash":"0000..."}'
```

### Check Mempool
```bash
curl http://127.0.0.1:8000/api/mempool/stats | jq .
```

### Stop Mining
```bash
curl -X POST http://127.0.0.1:8000/api/mining/stop | jq .
```

## Response Formats

### Status Response
```json
{
  "is_mining": true,
  "miner_address": "ATMN_...",
  "blocks_found": 5,
  "hash_rate": 125000.5,
  "uptime_seconds": 3600
}
```

### Template Response
```json
{
  "version": 1,
  "prev_block_hash": "000000...",
  "merkle_root": "000000...",
  "timestamp": 1764906289,
  "bits": 486604799,
  "height": 1234,
  "transactions": []
}
```

### Error Response
```json
{
  "error": "ERROR_CODE",
  "message": "Error description"
}
```

## Error Codes
- `ALREADY_MINING` - Mining already active
- `NOT_MINING` - Mining not active
- `INVALID_ADDRESS` - Invalid address format
- `INVALID_HASH` - Invalid block hash
- `DATABASE_ERROR` - Database failure

## Testing All Endpoints
```bash
# Complete test flow
curl -X POST http://127.0.0.1:8000/api/mining/start -H "Content-Type: application/json" -d '{"miner_address":"ATMN_TEST","threads":2}' && \
curl http://127.0.0.1:8000/api/mining/status && \
curl http://127.0.0.1:8000/api/mining/template && \
curl http://127.0.0.1:8000/api/mempool/stats && \
curl -X POST http://127.0.0.1:8000/api/mining/stop
```

## Common Tasks

### Monitor Mining
```bash
# Watch mining status every 2 seconds
watch -n 2 'curl -s http://127.0.0.1:8000/api/mining/status | jq .'
```

### Check if Mining Active
```bash
curl -s http://127.0.0.1:8000/api/mining/status | jq -r '.is_mining'
```

### Get Current Hash Rate
```bash
curl -s http://127.0.0.1:8000/api/mining/status | jq -r '.hash_rate'
```

### Get Blocks Found
```bash
curl -s http://127.0.0.1:8000/api/mining/status | jq -r '.blocks_found'
```

### Get Mempool Size
```bash
curl -s http://127.0.0.1:8000/api/mempool/stats | jq -r '.transaction_count'
```

