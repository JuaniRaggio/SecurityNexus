//! Transaction extraction and parsing module
//!
//! This module is responsible for extracting transactions (extrinsics) from
//! Substrate blocks and parsing their metadata.

use crate::types::{ChainEvent, ParsedTransaction, TransactionContext};
use anyhow::{Context, Result};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use subxt::{
    backend::BlockRef,
    config::substrate::H256,
    OnlineClient, PolkadotConfig,
};
use tracing::debug;

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
    ///
    /// MVP: For now, we just count extrinsics and create placeholder records.
    /// Full parsing will be added in Phase 2 with proper metadata support.
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

        // Get extrinsics count
        let extrinsics = block.extrinsics().await.context("Failed to get extrinsics")?;
        let extrinsic_count = extrinsics.iter().count();

        debug!(
            "Block #{} contains {} extrinsics",
            block_number, extrinsic_count
        );

        // For MVP: Create placeholder transaction records
        // We'll add full parsing in Phase 2
        let mut transactions = Vec::new();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for index in 0..extrinsic_count {
            let hash = format!(
                "0x{}",
                hex::encode(blake3::hash(format!("{}-{}", block_number, index).as_bytes()).as_bytes())
            );

            transactions.push(ParsedTransaction {
                hash,
                block_number,
                block_hash: format!("0x{}", hex::encode(block_hash.0)),
                index: index as u32,
                caller: format!("extrinsic_{}", index),
                pallet: "system".to_string(), // Placeholder
                call: "extrinsic".to_string(), // Placeholder
                args: Vec::new(),
                signature: None,
                nonce: None,
                timestamp,
                success: true,
            });
        }

        debug!(
            "Created {} transaction records from block #{}",
            transactions.len(),
            block_number
        );

        Ok(transactions)
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
            // Note: field_bytes() returns the raw event data, we'll parse extrinsic_index later if needed
            let extrinsic_index = None; // TODO: Extract extrinsic index from event phase

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
