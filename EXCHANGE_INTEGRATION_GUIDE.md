# ANTIMONY 2.0 - Exchange Integration Guide

## Overview

This guide provides step-by-step instructions for cryptocurrency exchanges to integrate Antimony Coin (ATMN) using the Rosetta API v1.4.13.

**Blockchain:** Antimony 2.0  
**Symbol:** ATMN  
**Algorithm:** SHA-256d (Bitcoin-compatible)  
**Block Time:** 12 seconds  
**Confirmations:** 6 blocks recommended (72 seconds)  
**API Standard:** Rosetta v1.4.13

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [API Endpoints](#api-endpoints)
3. [Deposit Integration](#deposit-integration)
4. [Withdrawal Integration](#withdrawal-integration)
5. [Balance Management](#balance-management)
6. [Transaction Monitoring](#transaction-monitoring)
7. [Security Best Practices](#security-best-practices)
8. [Testing Guide](#testing-guide)
9. [Production Checklist](#production-checklist)

---

## 1. Architecture Overview

### System Components

```
┌─────────────────┐
│   Exchange      │
│   Backend       │
└────────┬────────┘
         │
         │ HTTP/HTTPS
         │
┌────────▼────────┐
│  Rosetta API    │  Port 8080
│  (atmn-rosetta) │
└────────┬────────┘
         │
┌────────▼────────┐
│  ATMN Core      │  RocksDB
│  Blockchain     │  Storage
└─────────────────┘
```

### Required Infrastructure

- **Full Node:** ATMN blockchain node with complete history
- **Rosetta API Server:** Data and Construction API endpoints
- **Database:** PostgreSQL or MySQL for exchange wallet tracking
- **Hot Wallet:** For automated withdrawals (small balance)
- **Cold Wallet:** For long-term storage (large balance)

---

## 2. API Endpoints

### Base URL

**Testnet:** `https://testnet-api.antimony.network`  
**Mainnet:** `https://api.antimony.network` (Coming soon)

### Data API Endpoints

#### GET /network/list
List supported networks.

**Request:**
```json
{
  "metadata": {}
}
```

**Response:**
```json
{
  "network_identifiers": [
    {
      "blockchain": "antimony",
      "network": "testnet"
    }
  ]
}
```

#### POST /network/status
Get current network status.

**Request:**
```json
{
  "network_identifier": {
    "blockchain": "antimony",
    "network": "testnet"
  }
}
```

**Response:**
```json
{
  "current_block_identifier": {
    "index": 4032,
    "hash": "0x..."
  },
  "current_block_timestamp": 1733502000000,
  "genesis_block_identifier": {
    "index": 0,
    "hash": "0x..."
  },
  "peers": [
    {
      "peer_id": "7f2181efa8389b94"
    }
  ]
}
```

#### POST /block
Get block by height or hash.

**Request:**
```json
{
  "network_identifier": {
    "blockchain": "antimony",
    "network": "testnet"
  },
  "block_identifier": {
    "index": 4032
  }
}
```

**Response:**
```json
{
  "block": {
    "block_identifier": {
      "index": 4032,
      "hash": "0x..."
    },
    "parent_block_identifier": {
      "index": 4031,
      "hash": "0x..."
    },
    "timestamp": 1733502000000,
    "transactions": [
      {
        "transaction_identifier": {
          "hash": "0x..."
        },
        "operations": [
          {
            "operation_identifier": {
              "index": 0
            },
            "type": "COINBASE",
            "status": "SUCCESS",
            "account": {
              "address": "atmn1..."
            },
            "amount": {
              "value": "5000000000",
              "currency": {
                "symbol": "ATMN",
                "decimals": 8
              }
            }
          }
        ]
      }
    ]
  }
}
```

#### POST /account/balance
Get account balance.

**Request:**
```json
{
  "network_identifier": {
    "blockchain": "antimony",
    "network": "testnet"
  },
  "account_identifier": {
    "address": "atmn1qyqszqgpqyqszqgpqyqszqgpqyqszqgpq5n3xd2"
  }
}
```

**Response:**
```json
{
  "block_identifier": {
    "index": 4032,
    "hash": "0x..."
  },
  "balances": [
    {
      "value": "20073846400000",
      "currency": {
        "symbol": "ATMN",
        "decimals": 8
      }
    }
  ]
}
```

### Construction API Endpoints

#### POST /construction/derive
Derive address from public key.

**Request:**
```json
{
  "network_identifier": {
    "blockchain": "antimony",
    "network": "testnet"
  },
  "public_key": {
    "hex_bytes": "0x...",
    "curve_type": "secp256k1"
  }
}
```

#### POST /construction/preprocess
Preprocess transaction.

**Request:**
```json
{
  "network_identifier": {
    "blockchain": "antimony",
    "network": "testnet"
  },
  "operations": [
    {
      "operation_identifier": {
        "index": 0
      },
      "type": "TRANSFER",
      "account": {
        "address": "atmn1sender..."
      },
      "amount": {
        "value": "-1000000000",
        "currency": {
          "symbol": "ATMN",
          "decimals": 8
        }
      }
    },
    {
      "operation_identifier": {
        "index": 1
      },
      "type": "TRANSFER",
      "account": {
        "address": "atmn1recipient..."
      },
      "amount": {
        "value": "1000000000",
        "currency": {
          "symbol": "ATMN",
          "decimals": 8
        }
      }
    }
  ],
  "metadata": {}
}
```

#### POST /construction/payloads
Generate signing payloads.

#### POST /construction/parse
Parse unsigned or signed transaction.

#### POST /construction/combine
Combine unsigned transaction with signatures.

#### POST /construction/hash
Get transaction hash.

#### POST /construction/submit
Submit signed transaction to mempool.

---

## 3. Deposit Integration

### Architecture

```
User Deposit → Unique Address → Monitor Blockchain → 6 Confirmations → Credit Account
```

### Step 1: Generate Deposit Addresses

Generate unique deposit addresses for each user using your key derivation scheme (BIP32/BIP44).

**Example (Python):**
```python
from bitcoinlib.keys import HDKey

# Master seed (store securely!)
master_key = HDKey.from_seed('your-master-seed-hex')

# Derive user deposit address (BIP44 path)
user_id = 12345
path = f"m/44'/0'/0'/0/{user_id}"
user_key = master_key.subkey_for_path(path)

# ATMN address format (Bech32)
deposit_address = user_key.address()
print(f"Deposit address for user {user_id}: {deposit_address}")
```

### Step 2: Monitor Blockchain

Poll the Rosetta API every 12 seconds (1 block time) for new blocks.

**Example (Python):**
```python
import requests
import time

API_URL = "https://testnet-api.antimony.network"
last_block = 0

while True:
    # Get current block height
    response = requests.post(f"{API_URL}/network/status", json={
        "network_identifier": {
            "blockchain": "antimony",
            "network": "testnet"
        }
    })
    
    current_block = response.json()["current_block_identifier"]["index"]
    
    # Process new blocks
    for block_height in range(last_block + 1, current_block + 1):
        process_block(block_height)
    
    last_block = current_block
    time.sleep(12)  # Wait for next block

def process_block(height):
    response = requests.post(f"{API_URL}/block", json={
        "network_identifier": {
            "blockchain": "antimony",
            "network": "testnet"
        },
        "block_identifier": {
            "index": height
        }
    })
    
    block = response.json()["block"]
    
    # Check each transaction
    for tx in block["transactions"]:
        process_transaction(tx, height)

def process_transaction(tx, block_height):
    tx_hash = tx["transaction_identifier"]["hash"]
    
    for op in tx["operations"]:
        if op["type"] == "TRANSFER" and int(op["amount"]["value"]) > 0:
            address = op["account"]["address"]
            amount = int(op["amount"]["value"]) / 100000000  # Convert to ATMN
            
            # Check if this is a user deposit address
            user_id = lookup_user_by_address(address)
            if user_id:
                # Record pending deposit
                record_pending_deposit(user_id, tx_hash, amount, block_height)
```

### Step 3: Confirmation Tracking

Wait for 6 confirmations before crediting user account.

**Example:**
```python
def check_confirmations():
    pending_deposits = get_pending_deposits()
    current_height = get_current_block_height()
    
    for deposit in pending_deposits:
        confirmations = current_height - deposit["block_height"]
        
        if confirmations >= 6:
            # Credit user account
            credit_user_account(
                user_id=deposit["user_id"],
                amount=deposit["amount"],
                tx_hash=deposit["tx_hash"]
            )
            
            # Mark as confirmed
            mark_deposit_confirmed(deposit["id"])
            
            # Send notification
            notify_user_deposit_confirmed(deposit["user_id"], deposit["amount"])
```

---

## 4. Withdrawal Integration

### Architecture

```
User Request → Verify Balance → Create TX → Sign TX → Submit → Monitor Confirmation
```

### Step 1: UTXO Management

ATMN uses a UTXO model. Track available UTXOs for your hot wallet.

**Example:**
```python
def get_available_utxos(address):
    # Query blockchain for unspent outputs
    response = requests.post(f"{API_URL}/account/balance", json={
        "network_identifier": {
            "blockchain": "antimony",
            "network": "testnet"
        },
        "account_identifier": {
            "address": address
        }
    })
    
    # Parse UTXOs from response
    utxos = []
    # ... implementation depends on your UTXO tracking
    
    return utxos
```

### Step 2: Transaction Construction

Use Rosetta Construction API to build transaction.

**Example:**
```python
def create_withdrawal(recipient, amount, fee=10000):
    # 1. Preprocess
    operations = [
        {
            "operation_identifier": {"index": 0},
            "type": "TRANSFER",
            "account": {"address": HOT_WALLET_ADDRESS},
            "amount": {
                "value": str(-(amount + fee)),
                "currency": {"symbol": "ATMN", "decimals": 8}
            }
        },
        {
            "operation_identifier": {"index": 1},
            "type": "TRANSFER",
            "account": {"address": recipient},
            "amount": {
                "value": str(amount),
                "currency": {"symbol": "ATMN", "decimals": 8}
            }
        }
    ]
    
    preprocess_response = requests.post(f"{API_URL}/construction/preprocess", json={
        "network_identifier": NETWORK_ID,
        "operations": operations
    })
    
    # 2. Get metadata
    metadata_response = requests.post(f"{API_URL}/construction/metadata", json={
        "network_identifier": NETWORK_ID,
        "options": preprocess_response.json()["options"]
    })
    
    # 3. Generate payloads
    payloads_response = requests.post(f"{API_URL}/construction/payloads", json={
        "network_identifier": NETWORK_ID,
        "operations": operations,
        "metadata": metadata_response.json()["metadata"]
    })
    
    # 4. Sign transaction
    unsigned_tx = payloads_response.json()["unsigned_transaction"]
    payloads = payloads_response.json()["payloads"]
    
    signatures = sign_payloads(payloads, HOT_WALLET_PRIVATE_KEY)
    
    # 5. Combine signatures
    combine_response = requests.post(f"{API_URL}/construction/combine", json={
        "network_identifier": NETWORK_ID,
        "unsigned_transaction": unsigned_tx,
        "signatures": signatures
    })
    
    # 6. Submit transaction
    signed_tx = combine_response.json()["signed_transaction"]
    
    submit_response = requests.post(f"{API_URL}/construction/submit", json={
        "network_identifier": NETWORK_ID,
        "signed_transaction": signed_tx
    })
    
    return submit_response.json()["transaction_identifier"]["hash"]
```

### Step 3: Withdrawal Confirmation

Monitor withdrawal transactions until confirmed.

**Example:**
```python
def monitor_withdrawal(tx_hash):
    while True:
        response = requests.post(f"{API_URL}/block/transaction", json={
            "network_identifier": NETWORK_ID,
            "transaction_identifier": {"hash": tx_hash}
        })
        
        if response.status_code == 200:
            block_height = response.json()["block_identifier"]["index"]
            current_height = get_current_block_height()
            confirmations = current_height - block_height
            
            if confirmations >= 6:
                # Mark as confirmed
                mark_withdrawal_confirmed(tx_hash)
                return True
        
        time.sleep(12)
```

---

## 5. Balance Management

### Hot Wallet Management

Keep only necessary funds in hot wallet for withdrawals.

**Recommended Balance:**
- Hot Wallet: 5-10% of total holdings
- Cold Wallet: 90-95% of total holdings

**Rebalancing Strategy:**
```python
def rebalance_wallets():
    hot_balance = get_wallet_balance(HOT_WALLET_ADDRESS)
    target_hot_balance = get_total_holdings() * 0.07  # 7%
    
    if hot_balance < target_hot_balance * 0.5:
        # Hot wallet too low, transfer from cold
        amount = target_hot_balance - hot_balance
        transfer_from_cold_to_hot(amount)
    
    elif hot_balance > target_hot_balance * 2:
        # Hot wallet too high, transfer to cold
        amount = hot_balance - target_hot_balance
        transfer_from_hot_to_cold(amount)
```

### Cold Wallet Security

- **Offline Signing:** Sign transactions on air-gapped machine
- **Multi-Signature:** Require 2-of-3 or 3-of-5 signatures
- **Hardware Wallets:** Use Ledger/Trezor for key storage
- **Geographic Distribution:** Store keys in multiple secure locations

---

## 6. Transaction Monitoring

### Real-time Monitoring

Use WebSocket API for real-time updates (optional).

**Example:**
```python
import websocket

def on_message(ws, message):
    data = json.loads(message)
    
    if data["type"] == "new_block":
        process_new_block(data["block"])
    
    elif data["type"] == "new_transaction":
        process_pending_transaction(data["transaction"])

ws = websocket.WebSocketApp(
    "wss://testnet-api.antimony.network/ws",
    on_message=on_message
)

ws.run_forever()
```

### Orphaned Blocks

Handle chain reorganizations properly.

**Example:**
```python
def handle_reorg(old_chain_tip, new_chain_tip):
    # Find common ancestor
    common_ancestor = find_common_ancestor(old_chain_tip, new_chain_tip)
    
    # Reverse transactions from orphaned blocks
    for height in range(common_ancestor + 1, old_chain_tip + 1):
        reverse_block_transactions(height)
    
    # Reprocess new chain
    for height in range(common_ancestor + 1, new_chain_tip + 1):
        process_block(height)
```

---

## 7. Security Best Practices

### Key Management

1. **Never store private keys in plain text**
2. **Use HSM (Hardware Security Module) for hot wallet**
3. **Encrypt keys with strong passphrase (AES-256)**
4. **Implement key rotation policy (quarterly)**
5. **Backup keys in multiple secure locations**

### API Security

1. **Use HTTPS/TLS for all API calls**
2. **Implement rate limiting (100 req/min)**
3. **Validate all inputs (addresses, amounts)**
4. **Use API keys with IP whitelisting**
5. **Monitor for suspicious activity**

### Transaction Security

1. **Verify destination addresses before signing**
2. **Implement withdrawal limits (daily/per-transaction)**
3. **Require multi-factor authentication for withdrawals**
4. **Implement manual review for large withdrawals**
5. **Use time-locks for large transactions**

### Monitoring

1. **Alert on unusual withdrawal patterns**
2. **Monitor hot wallet balance**
3. **Track failed transaction rates**
4. **Log all API calls**
5. **Set up automated alerts for errors**

---

## 8. Testing Guide

### Testnet Testing

1. **Get testnet ATMN from faucet:**
   ```
   https://testnet-faucet.antimony.network
   ```

2. **Test deposit flow:**
   - Generate 10 test addresses
   - Send test deposits
   - Verify confirmation tracking
   - Verify account crediting

3. **Test withdrawal flow:**
   - Create test withdrawals
   - Verify transaction construction
   - Verify signing and submission
   - Verify confirmation monitoring

4. **Test edge cases:**
   - Double-spend attempts
   - Invalid addresses
   - Insufficient balance
   - Network failures
   - Chain reorganizations

### Load Testing

Test with high transaction volume:

```bash
# Generate 1000 test transactions
for i in {1..1000}; do
    ./test-withdrawal.sh user_$i 1.0
done
```

---

## 9. Production Checklist

### Pre-Launch (Testnet)

- [ ] Deposit system tested (100+ transactions)
- [ ] Withdrawal system tested (100+ transactions)
- [ ] Balance reconciliation verified
- [ ] Security audit completed
- [ ] Load testing completed (1000+ TPS)
- [ ] Disaster recovery plan documented
- [ ] Monitoring and alerting configured
- [ ] Customer support trained

### Launch (Mainnet)

- [ ] Mainnet API endpoint configured
- [ ] Hot wallet funded with initial balance
- [ ] Cold wallet addresses generated
- [ ] Multi-signature setup verified
- [ ] Backup systems tested
- [ ] Compliance checks completed
- [ ] Trading pairs configured
- [ ] Public announcement prepared

### Post-Launch

- [ ] Monitor deposits/withdrawals 24/7
- [ ] Daily balance reconciliation
- [ ] Weekly security reviews
- [ ] Monthly cold wallet audits
- [ ] Quarterly key rotation
- [ ] Regular penetration testing

---

## Support

**Technical Support:**
- Email: tech-support@antimony.network
- Discord: https://discord.gg/antimony
- GitHub: https://github.com/msrusu87-web/antimony-2.0

**Integration Assistance:**
- Schedule a call: https://calendly.com/antimony-dev
- Developer Slack: https://antimony-dev.slack.com

**Documentation:**
- API Docs: https://docs.antimony.network
- Exchange Guide: https://docs.antimony.network/exchange
- Rosetta Spec: https://www.rosetta-api.org

---

## Appendix A: Code Examples

Complete integration examples available at:
https://github.com/msrusu87-web/antimony-2.0/tree/main/examples/exchange

- Python: `examples/exchange/python/`
- JavaScript: `examples/exchange/javascript/`
- Go: `examples/exchange/go/`
- Java: `examples/exchange/java/`

---

## Appendix B: Network Parameters

| Parameter | Testnet | Mainnet |
|-----------|---------|---------|
| Network ID | `testnet` | `mainnet` |
| Genesis Hash | `0x...` | `TBD` |
| Block Time | 12 seconds | 12 seconds |
| Difficulty Adjust | 2016 blocks | 2016 blocks |
| Confirmations | 6 blocks | 6 blocks |
| Address Format | Bech32 | Bech32 |
| Min TX Fee | 0.0001 ATMN | 0.0001 ATMN |
| Decimals | 8 | 8 |

---

*Last Updated: December 6, 2025*  
*Version: 1.0*
