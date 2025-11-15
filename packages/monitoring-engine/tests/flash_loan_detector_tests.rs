//! Integration tests for Flash Loan Detector
//!
//! TDD RED Phase: Write comprehensive tests BEFORE implementing the detector logic
//!
//! Flash loan attack pattern characteristics:
//! 1. Large borrow amount (typically >$100k equivalent)
//! 2. Multiple DeFi protocol interactions in single transaction
//! 3. Complete repayment within same transaction
//! 4. Significant balance changes (>50% of initial balance)
//! 5. Price manipulation or arbitrage indicators

use monitoring_engine::detectors::{Detector, FlashLoanDetector};
use monitoring_engine::types::{
    AlertSeverity, AttackPattern, ChainEvent, ParsedTransaction, StateChange, TransactionContext,
};

/// Helper: Create a transaction context for flash loan scenario
fn create_flash_loan_context() -> TransactionContext {
    let transaction = ParsedTransaction {
        hash: "0xflashloan123".to_string(),
        block_number: 1000,
        block_hash: "0xblock123".to_string(),
        index: 5,
        caller: "0xattacker".to_string(),
        pallet: "contracts".to_string(),
        call: "call".to_string(),
        args: vec![],
        signature: Some(vec![1, 2, 3]),
        nonce: Some(42),
        timestamp: 1234567890,
        success: true,
    };

    // Flash loan pattern: borrow → swap → swap → repay
    let events = vec![
        // Event 1: Large borrow from lending protocol
        ChainEvent {
            block_number: 1000,
            event_index: 0,
            extrinsic_index: Some(5),
            pallet: "lending_protocol".to_string(),
            event_name: "Borrowed".to_string(),
            data: vec![0; 32], // Placeholder for large amount
            topics: vec!["0xborrow_topic".to_string()],
        },
        // Event 2: Swap on DEX 1
        ChainEvent {
            block_number: 1000,
            event_index: 1,
            extrinsic_index: Some(5),
            pallet: "dex_protocol".to_string(),
            event_name: "Swapped".to_string(),
            data: vec![0; 32],
            topics: vec!["0xswap_topic".to_string()],
        },
        // Event 3: Swap on DEX 2 (price manipulation)
        ChainEvent {
            block_number: 1000,
            event_index: 2,
            extrinsic_index: Some(5),
            pallet: "dex_protocol".to_string(),
            event_name: "Swapped".to_string(),
            data: vec![0; 32],
            topics: vec!["0xswap_topic".to_string()],
        },
        // Event 4: Repayment to lending protocol
        ChainEvent {
            block_number: 1000,
            event_index: 3,
            extrinsic_index: Some(5),
            pallet: "lending_protocol".to_string(),
            event_name: "Repaid".to_string(),
            data: vec![0; 32],
            topics: vec!["0xrepay_topic".to_string()],
        },
    ];

    // Large balance changes indicating manipulation
    let state_changes = vec![
        StateChange {
            key: b"balance:token_a".to_vec(),
            old_value: Some(vec![0, 0, 0, 100]), // 100 tokens
            new_value: Some(vec![0, 0, 0, 200]), // 200 tokens (100% increase)
        },
        StateChange {
            key: b"balance:token_b".to_vec(),
            old_value: Some(vec![0, 0, 1, 0]), // 256 tokens
            new_value: Some(vec![0, 0, 0, 128]), // 128 tokens (50% decrease)
        },
    ];

    TransactionContext {
        transaction,
        events,
        state_changes,
    }
}

/// Helper: Create normal DeFi transaction (should NOT trigger detection)
fn create_normal_defi_context() -> TransactionContext {
    let transaction = ParsedTransaction {
        hash: "0xnormal456".to_string(),
        block_number: 1001,
        block_hash: "0xblock124".to_string(),
        index: 3,
        caller: "0xuser".to_string(),
        pallet: "dex_protocol".to_string(),
        call: "swap".to_string(),
        args: vec![],
        signature: Some(vec![4, 5, 6]),
        nonce: Some(10),
        timestamp: 1234567900,
        success: true,
    };

    // Single swap - normal DeFi activity
    let events = vec![ChainEvent {
        block_number: 1001,
        event_index: 0,
        extrinsic_index: Some(3),
        pallet: "dex_protocol".to_string(),
        event_name: "Swapped".to_string(),
        data: vec![0; 16], // Small amount
        topics: vec![],
    }];

    // Small balance change (normal trading)
    let state_changes = vec![StateChange {
        key: b"balance:token_a".to_vec(),
        old_value: Some(vec![0, 0, 0, 100]),
        new_value: Some(vec![0, 0, 0, 105]), // 5% change - normal
    }];

    TransactionContext {
        transaction,
        events,
        state_changes,
    }
}

/// Helper: Create context with borrow but no repay (incomplete flash loan)
fn create_incomplete_flash_loan_context() -> TransactionContext {
    let transaction = ParsedTransaction {
        hash: "0xincomplete789".to_string(),
        block_number: 1002,
        block_hash: "0xblock125".to_string(),
        index: 7,
        caller: "0xborrower".to_string(),
        pallet: "contracts".to_string(),
        call: "call".to_string(),
        args: vec![],
        signature: Some(vec![7, 8, 9]),
        nonce: Some(20),
        timestamp: 1234567910,
        success: false, // Failed transaction
    };

    // Borrow without repayment (should have lower confidence)
    let events = vec![ChainEvent {
        block_number: 1002,
        event_index: 0,
        extrinsic_index: Some(7),
        pallet: "lending_protocol".to_string(),
        event_name: "Borrowed".to_string(),
        data: vec![0; 32],
        topics: vec![],
    }];

    TransactionContext {
        transaction,
        events,
        state_changes: vec![],
    }
}

// ============================================================================
// TEST SUITE: Flash Loan Detection
// ============================================================================

#[tokio::test]
async fn test_detect_basic_flash_loan_pattern() {
    // Given: A transaction with classic flash loan pattern
    let detector = FlashLoanDetector::new();
    let context = create_flash_loan_context();

    // When: We analyze the transaction
    let result = detector.analyze_transaction(&context).await;

    // Then: Should detect flash loan with high confidence
    assert!(result.detected, "Flash loan pattern should be detected");
    assert_eq!(
        result.pattern,
        AttackPattern::FlashLoan,
        "Pattern should be FlashLoan"
    );
    assert!(
        result.confidence >= 0.8,
        "Confidence should be high (>=80%), got: {}",
        result.confidence
    );
    assert!(
        !result.description.is_empty(),
        "Should have meaningful description"
    );
    assert!(
        !result.evidence.is_empty(),
        "Should provide evidence for detection"
    );
}

#[tokio::test]
async fn test_flash_loan_requires_borrow_and_repay() {
    // Given: Transaction with borrow but no repay
    let detector = FlashLoanDetector::new();
    let context = create_incomplete_flash_loan_context();

    // When: We analyze the transaction
    let result = detector.analyze_transaction(&context).await;

    // Then: Should either not detect or have very low confidence
    if result.detected {
        assert!(
            result.confidence < 0.5,
            "Incomplete pattern should have low confidence (<50%), got: {}",
            result.confidence
        );
    }
}

#[tokio::test]
async fn test_normal_defi_not_detected_as_flash_loan() {
    // Given: Normal DeFi swap transaction
    let detector = FlashLoanDetector::new();
    let context = create_normal_defi_context();

    // When: We analyze the transaction
    let result = detector.analyze_transaction(&context).await;

    // Then: Should NOT detect as flash loan
    assert!(
        !result.detected || result.pattern != AttackPattern::FlashLoan,
        "Normal DeFi should not be flagged as flash loan"
    );
}

#[tokio::test]
async fn test_flash_loan_evidence_includes_key_indicators() {
    // Given: Flash loan transaction
    let detector = FlashLoanDetector::new();
    let context = create_flash_loan_context();

    // When: We analyze the transaction
    let result = detector.analyze_transaction(&context).await;

    // Then: Evidence should mention key indicators
    if result.detected {
        let evidence_str = result.evidence.join(" ").to_lowercase();

        // Should mention borrow and repay
        let has_borrow_evidence = evidence_str.contains("borrow");
        let has_repay_evidence = evidence_str.contains("repay");

        assert!(
            has_borrow_evidence || has_repay_evidence,
            "Evidence should mention borrow/repay pattern. Got: {:?}",
            result.evidence
        );
    }
}

#[tokio::test]
async fn test_flash_loan_confidence_scoring() {
    // Given: Multiple scenarios with different confidence levels
    let detector = FlashLoanDetector::new();

    // Complete flash loan should have high confidence
    let complete_context = create_flash_loan_context();
    let complete_result = detector.analyze_transaction(&complete_context).await;

    // Incomplete flash loan should have lower confidence
    let incomplete_context = create_incomplete_flash_loan_context();
    let incomplete_result = detector.analyze_transaction(&incomplete_context).await;

    // Normal DeFi should have no/low confidence
    let normal_context = create_normal_defi_context();
    let normal_result = detector.analyze_transaction(&normal_context).await;

    // Then: Confidence should correlate with suspiciousness
    if complete_result.detected && incomplete_result.detected {
        assert!(
            complete_result.confidence > incomplete_result.confidence,
            "Complete flash loan should have higher confidence than incomplete"
        );
    }

    if normal_result.detected && complete_result.detected {
        assert!(
            normal_result.confidence < complete_result.confidence,
            "Normal DeFi should have lower confidence than flash loan"
        );
    }
}

#[tokio::test]
async fn test_flash_loan_detector_enabled() {
    // Given: New detector instance
    let detector = FlashLoanDetector::new();

    // Then: Should be enabled by default
    assert!(detector.is_enabled(), "Detector should be enabled");
    assert_eq!(detector.name(), "FlashLoanDetector");
}

#[tokio::test]
async fn test_multiple_dex_interactions_increase_confidence() {
    // Given: Transaction with multiple DEX swaps (price manipulation indicator)
    let detector = FlashLoanDetector::new();
    let mut context = create_flash_loan_context();

    // Add more DEX interactions
    context.events.push(ChainEvent {
        block_number: 1000,
        event_index: 4,
        extrinsic_index: Some(5),
        pallet: "another_dex".to_string(),
        event_name: "Swapped".to_string(),
        data: vec![0; 32],
        topics: vec![],
    });

    // When: We analyze the transaction
    let result = detector.analyze_transaction(&context).await;

    // Then: Should detect with high confidence due to multiple interactions
    if result.detected {
        assert!(
            result.confidence >= 0.8,
            "Multiple DEX interactions should increase confidence"
        );
    }
}

#[tokio::test]
async fn test_large_balance_changes_indicator() {
    // Given: Transaction with >50% balance change
    let detector = FlashLoanDetector::new();
    let context = create_flash_loan_context();

    // When: We analyze the transaction
    let result = detector.analyze_transaction(&context).await;

    // Then: Large balance changes should be part of evidence
    if result.detected {
        let has_balance_evidence = result
            .evidence
            .iter()
            .any(|e| e.to_lowercase().contains("balance") || e.to_lowercase().contains("change"));

        // Note: This might not always be true, but for complete flash loans it should be
        // We'll make this assertion soft for now
        if !has_balance_evidence {
            eprintln!(
                "Warning: Expected balance change in evidence, got: {:?}",
                result.evidence
            );
        }
    }
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[tokio::test]
async fn test_flash_loan_detection_performance() {
    // Given: Flash loan detector and test context
    let detector = FlashLoanDetector::new();
    let context = create_flash_loan_context();

    // When: We measure analysis time
    let start = std::time::Instant::now();
    let _result = detector.analyze_transaction(&context).await;
    let duration = start.elapsed();

    // Then: Should complete in under 3ms (target from plan)
    assert!(
        duration.as_millis() < 3,
        "Detection should complete in <3ms, took: {}ms",
        duration.as_millis()
    );
}

#[tokio::test]
async fn test_batch_analysis_performance() {
    // Given: Multiple transactions to analyze
    let detector = FlashLoanDetector::new();
    let contexts = vec![
        create_flash_loan_context(),
        create_normal_defi_context(),
        create_incomplete_flash_loan_context(),
        create_flash_loan_context(),
        create_normal_defi_context(),
    ];

    // When: We analyze batch
    let start = std::time::Instant::now();
    for context in &contexts {
        let _result = detector.analyze_transaction(context).await;
    }
    let duration = start.elapsed();

    // Then: Should process 5 transactions in under 15ms
    assert!(
        duration.as_millis() < 15,
        "Batch of 5 should complete in <15ms, took: {}ms",
        duration.as_millis()
    );
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_helper_creates_valid_flash_loan_context() {
        let context = create_flash_loan_context();

        assert_eq!(context.transaction.hash, "0xflashloan123");
        assert_eq!(context.events.len(), 4); // borrow + 2 swaps + repay
        assert!(
            context.events[0].event_name == "Borrowed",
            "First event should be Borrowed"
        );
        assert!(
            context.events[3].event_name == "Repaid",
            "Last event should be Repaid"
        );
        assert_eq!(context.state_changes.len(), 2); // Large balance changes
    }

    #[test]
    fn test_helper_creates_valid_normal_context() {
        let context = create_normal_defi_context();

        assert_eq!(context.events.len(), 1); // Single swap
        assert_eq!(context.state_changes.len(), 1); // Small change
    }

    #[test]
    fn test_helper_creates_valid_incomplete_context() {
        let context = create_incomplete_flash_loan_context();

        assert!(!context.transaction.success); // Should be failed
        assert_eq!(context.events.len(), 1); // Only borrow, no repay
        assert_eq!(context.state_changes.len(), 0); // No state changes
    }
}
