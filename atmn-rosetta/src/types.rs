// Rosetta API data types v1.4.13
use serde::{Deserialize, Serialize};

/// Network identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkIdentifier {
    pub blockchain: String,
    pub network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_network_identifier: Option<SubNetworkIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubNetworkIdentifier {
    pub network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Block identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockIdentifier {
    pub index: i64,
    pub hash: String,
}

impl BlockIdentifier {
    pub fn new(index: u64, hash: String) -> Self {
        Self {
            index: index as i64,
            hash,
        }
    }
}

/// Partial block identifier (for requests)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialBlockIdentifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
}

/// Transaction identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionIdentifier {
    pub hash: String,
}

/// Account identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountIdentifier {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<SubAccountIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubAccountIdentifier {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Currency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Currency {
    pub symbol: String,
    pub decimals: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Currency {
    pub fn atmn() -> Self {
        Self {
            symbol: "ATMN".to_string(),
            decimals: 8, // 1 ATMN = 100,000,000 satoshis
            metadata: None,
        }
    }
}

/// Amount
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Amount {
    pub value: String, // String to handle large numbers
    pub currency: Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Amount {
    pub fn new(value: i64, currency: Currency) -> Self {
        Self {
            value: value.to_string(),
            currency,
            metadata: None,
        }
    }
}

/// Operation (part of a transaction)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub operation_identifier: OperationIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_operations: Option<Vec<OperationIdentifier>>,
    pub r#type: String, // TRANSFER, MINT, BURN, FEE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>, // SUCCESS, FAILED, PENDING
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin_change: Option<CoinChange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OperationIdentifier {
    pub index: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_index: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinChange {
    pub coin_identifier: CoinIdentifier,
    pub coin_action: String, // coin_created, coin_spent
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoinIdentifier {
    pub identifier: String, // format: txhash:vout
}

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub transaction_identifier: TransactionIdentifier,
    pub operations: Vec<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_transactions: Option<Vec<RelatedTransaction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedTransaction {
    pub network_identifier: NetworkIdentifier,
    pub transaction_identifier: TransactionIdentifier,
    pub direction: String, // forward, backward
}

/// Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub block_identifier: BlockIdentifier,
    pub parent_block_identifier: BlockIdentifier,
    pub timestamp: i64, // milliseconds since Unix epoch
    pub transactions: Vec<Transaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub retriable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub peer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Version
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub rosetta_version: String,
    pub node_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middleware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Allow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allow {
    pub operation_statuses: Vec<OperationStatus>,
    pub operation_types: Vec<String>,
    pub errors: Vec<Error>,
    pub historical_balance_lookup: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_start_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_methods: Option<Vec<String>>,
    pub balance_exemptions: Vec<BalanceExemption>,
    pub mempool_coins: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationStatus {
    pub status: String,
    pub successful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceExemption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exemption_type: Option<String>,
}

/// SyncStatus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced: Option<bool>,
}

/// Coin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coin {
    pub coin_identifier: CoinIdentifier,
    pub amount: Amount,
}

// ============= Request/Response Types =============

/// /network/list
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}

/// /network/options
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkOptionsRequest {
    pub network_identifier: NetworkIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkOptionsResponse {
    pub version: Version,
    pub allow: Allow,
}

/// /network/status
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatusRequest {
    pub network_identifier: NetworkIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkStatusResponse {
    pub current_block_identifier: BlockIdentifier,
    pub current_block_timestamp: i64,
    pub genesis_block_identifier: BlockIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_block_identifier: Option<BlockIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_status: Option<SyncStatus>,
    pub peers: Vec<Peer>,
}

/// /block
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockRequest {
    pub network_identifier: NetworkIdentifier,
    pub block_identifier: PartialBlockIdentifier,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub block: Option<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_transactions: Option<Vec<TransactionIdentifier>>,
}

/// /block/transaction
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockTransactionRequest {
    pub network_identifier: NetworkIdentifier,
    pub block_identifier: BlockIdentifier,
    pub transaction_identifier: TransactionIdentifier,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockTransactionResponse {
    pub transaction: Transaction,
}

/// /account/balance
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalanceRequest {
    pub network_identifier: NetworkIdentifier,
    pub account_identifier: AccountIdentifier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_identifier: Option<PartialBlockIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<Currency>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalanceResponse {
    pub block_identifier: BlockIdentifier,
    pub balances: Vec<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coins: Option<Vec<Coin>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// /account/coins
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCoinsRequest {
    pub network_identifier: NetworkIdentifier,
    pub account_identifier: AccountIdentifier,
    pub include_mempool: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<Currency>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCoinsResponse {
    pub block_identifier: BlockIdentifier,
    pub coins: Vec<Coin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
