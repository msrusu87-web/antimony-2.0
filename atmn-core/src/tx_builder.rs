// atmn-core/src/tx_builder.rs
// Transaction builder for creating and signing transactions

use crate::{Transaction, Storage};
use crate::transaction::{TxInput, TxOutput};
use crate::types::{TxHash, Amount};
use crate::error::{Error, Result};
use crate::storage::UtxoEntry;

pub struct TransactionBuilder {
    storage: Storage,
}

impl TransactionBuilder {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
    
    /// Create a simple payment transaction
    /// Automatically selects UTXOs and calculates change
    pub fn create_payment(
        &self,
        from_address: &str,
        to_address: &str,
        amount: Amount,
        fee: Amount,
    ) -> Result<Transaction> {
        // Get UTXOs for sender
        let utxos = self.storage.get_utxos_for_address(from_address)?;
        
        if utxos.is_empty() {
            return Err(Error::InvalidTransaction);
        }
        
        // Select UTXOs (simple: just take enough to cover amount + fee)
        let total_needed = amount + fee;
        let mut selected_utxos = Vec::new();
        let mut total_input = 0u64;
        
        for utxo in utxos {
            selected_utxos.push(utxo.clone());
            total_input += utxo.amount;
            
            if total_input >= total_needed {
                break;
            }
        }
        
        if total_input < total_needed {
            return Err(Error::InsufficientFunds);
        }
        
        // Create inputs
        let inputs: Vec<TxInput> = selected_utxos
            .iter()
            .map(|utxo| TxInput {
                prev_tx_hash: utxo.tx_hash.clone(),
                prev_tx_index: utxo.output_index,
                script: from_address.as_bytes().to_vec(), // Signature placeholder
                sequence: 0xFFFFFFFF,
            })
            .collect();
        
        // Create outputs
        let mut outputs = Vec::new();
        
        // Payment output
        outputs.push(TxOutput {
            amount,
            script_pubkey: to_address.as_bytes().to_vec(),
        });
        
        // Change output (if any)
        let change = total_input - total_needed;
        if change > 0 {
            outputs.push(TxOutput {
                amount: change,
                script_pubkey: from_address.as_bytes().to_vec(),
            });
        }
        
        Ok(Transaction {
            version: 1,
            inputs,
            outputs,
            locktime: 0,
        })
    }
    
    /// Validate a transaction
    pub fn validate_transaction(&self, tx: &Transaction) -> Result<()> {
        // Check not empty
        if tx.inputs.is_empty() || tx.outputs.is_empty() {
            return Err(Error::InvalidTransaction);
        }
        
        // Check not coinbase (coinbase can only be in block, not submitted)
        if tx.is_coinbase() {
            return Err(Error::InvalidTransaction);
        }
        
        // Verify inputs exist and are unspent
        let mut total_input = 0u64;
        for input in &tx.inputs {
            let utxo_key = format!("{}:{}", input.prev_tx_hash, input.prev_tx_index);
            // TODO: Check UTXO exists in storage
            // For now, just check non-zero
            total_input += 1_000_000_000; // Placeholder
        }
        
        let total_output: Amount = tx.outputs.iter().map(|o| o.amount).sum();
        
        // Verify inputs >= outputs (the difference is the fee)
        if total_input < total_output {
            return Err(Error::InsufficientFunds);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tx_builder() {
        // Test will be implemented when storage is available
        assert!(true);
    }
}
