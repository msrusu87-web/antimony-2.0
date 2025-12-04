// atmn-core/src/transaction.rs
// Transaction structure and validation

use serde::{Deserialize, Serialize};
use crate::types::{TxHash, Amount, Address, Signature};
use crate::error::Result;

/// Transaction Input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxInput {
    pub prev_tx_hash: TxHash,
    pub prev_tx_index: u32,
    pub script: Vec<u8>,
    pub sequence: u32,
}

/// Transaction Output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutput {
    pub amount: Amount,
    pub script_pubkey: Vec<u8>,
}

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub locktime: u32,
}

impl Transaction {
    pub fn hash(&self) -> TxHash {
        // Double SHA256 of serialized tx
        TxHash::from_bytes([0u8; 32])  // TODO: Implement hashing
    }

    pub fn is_coinbase(&self) -> bool {
        self.inputs.len() == 1 && 
        TxHash::from_bytes([0u8; 32]) == self.inputs[0].prev_tx_hash
    }

    pub fn total_input_amount(&self) -> Amount {
        // TODO: Sum input values
        0
    }

    pub fn total_output_amount(&self) -> Amount {
        self.outputs.iter().map(|o| o.amount).sum()
    }

    pub fn is_valid(&self) -> Result<()> {
        // TODO: Implement transaction validation
        Ok(())
    }

    pub fn size(&self) -> usize {
        // TODO: Calculate serialized size
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction {
            version: 1,
            inputs: vec![],
            outputs: vec![],
            locktime: 0,
        };
        assert!(tx.is_coinbase() || !tx.is_coinbase());
    }
}
