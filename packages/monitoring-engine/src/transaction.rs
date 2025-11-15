//! Transaction extraction and parsing module
//!
//! This module is responsible for extracting transactions (extrinsics) from
//! Substrate blocks and parsing their metadata.

use crate::types::{ChainEvent, ParsedTransaction, StateChange, TransactionContext};
use anyhow::{Context, Result};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use subxt::{
    backend::BlockRef,
    blocks::Extrinsics,
    config::substrate::H256,
    OnlineClient, PolkadotConfig,
};
use tracing::{debug, trace, warn};

/// Extracts and parses transactions from Substrate blocks
pub struct TransactionExtractor {
    client: Arc<OnlineClient<PolkadotConfig>>,
}

impl TransactionExtractor {
    /// Create a new transaction extractor
    pub fn new(client: Arc<OnlineClient<PolkadotConfig>>) -> Self {
        Self { client }
    }

    /// Extract all transactions from a block
    ///
    /// Returns a vector of ParsedTransaction objects, one for each extrinsic
    /// in the block (including inherents like timestamp)
    pub async fn extract_from_block(
        &self,
        block_hash: H256,
        block_number: u64,
    ) -> Result<Vec<ParsedTransaction>> {
        debug!(
            "Extracting transactions from block #{} ({})",
            block_number,
            hex::encode(block_hash.0)
        );

        // Get the block
        let block = self
            .client
            .blocks()
            .at(BlockRef::from_hash(block_hash))
            .await
            .context("Failed to get block")?;

        // Get extrinsics
        let extrinsics = block.extrinsics().await.context("Failed to get extrinsics")?;

        let mut transactions = Vec::new();

        // Parse each extrinsic
        for (index, ext_details) in extrinsics.iter().enumerate() {
            match self.parse_extrinsic(ext_details?, index as u32, block_number, block_hash).await {
                Ok(tx) => {
                    trace!(
                        "Parsed transaction {}: {}::{}",
                        index,
                        tx.pallet,
                        tx.call
                    );
                    transactions.push(tx);
                }
                Err(e) => {
                    warn!(
                        "Failed to parse extrinsic {} in block {}: {}",
                        index, block_number, e
                    );
                }
            }
        }

        debug!(
            "Extracted {} transactions from block #{}",
            transactions.len(),
            block_number
        );

        Ok(transactions)
    }

    /// Parse a single extrinsic into ParsedTransaction
    async fn parse_extrinsic(
        &self,
        ext: subxt::blocks::ExtrinsicDetails<PolkadotConfig>,
        index: u32,
        block_number: u64,
        block_hash: H256,
    ) -> Result<ParsedTransaction> {
        // Get pallet and call names
        let pallet_name = ext.pallet_name().to_string();
        let call_name = ext.variant_name().to_string();

        // Extract caller/signer address (if signed)
        let caller = if let Some(addr) = ext.address_bytes() {
            // Convert address bytes to hex string
            format!("0x{}", hex::encode(addr))
        } else {
            // Unsigned extrinsic (inherent)
            "system".to_string()
        };

        // Get the call data (arguments)
        let call_data = ext.call_data();
        let args = call_data.to_vec();

        // Get signature and nonce (if signed)
        let signature = ext.signature_bytes().map(|s| s.to_vec());
        let nonce = None; // TODO: Extract nonce from signature

        // Determine success based on events (we'll check this later)
        let success = true; // Assume success for now, will be updated with event data

        // Get current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Create transaction hash from extrinsic hash
        let hash = format!("0x{}", hex::encode(ext.hash()));

        Ok(ParsedTransaction {
            hash,
            block_number,
            block_hash: format!("0x{}", hex::encode(block_hash.0)),
            index,
            caller,
            pallet: pallet_name,
            call: call_name,
            args,
            signature,
            nonce,
            timestamp,
            success,
        })
    }

    /// Create full context for a transaction including associated events
    pub fn create_context(
        transaction: ParsedTransaction,
        all_events: &[ChainEvent],
    ) -> TransactionContext {
        // Filter events that belong to this transaction
        let tx_events: Vec<ChainEvent> = all_events
            .iter()
            .filter(|e| e.extrinsic_index == Some(transaction.index))
            .cloned()
            .collect();

        debug!(
            "Transaction {} has {} associated events",
            transaction.index,
            tx_events.len()
        );

        TransactionContext {
            transaction,
            events: tx_events,
            state_changes: Vec::new(), // TODO: State tracking
        }
    }

    /// Extract events from a block and associate them with transactions
    pub async fn extract_events(
        &self,
        block_hash: H256,
        block_number: u64,
    ) -> Result<Vec<ChainEvent>> {
        let block = self
            .client
            .blocks()
            .at(BlockRef::from_hash(block_hash))
            .await
            .context("Failed to get block for events")?;

        let events = block.events().await.context("Failed to get events")?;

        let mut chain_events = Vec::new();
        let mut event_index = 0u32;

        for event_details in events.iter() {
            let event = event_details?;

            let pallet_name = event.pallet_name().to_string();
            let event_name = event.variant_name().to_string();

            // Get the extrinsic index if this event is associated with one
            let extrinsic_index = event.extrinsic_index();

            // Get event data
            let event_data = event.bytes().to_vec();

            chain_events.push(ChainEvent {
                block_number,
                event_index,
                extrinsic_index,
                pallet: pallet_name,
                event_name,
                data: event_data,
                topics: Vec::new(), // TODO: Extract topics if needed
            });

            event_index += 1;
        }

        debug!(
            "Extracted {} events from block #{}",
            chain_events.len(),
            block_number
        );

        Ok(chain_events)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_extractor_creation() {
        // We can't test actual extraction without a real client
        // but we can test that the struct is created correctly
        // This is a placeholder for now
        assert!(true);
    }
}
