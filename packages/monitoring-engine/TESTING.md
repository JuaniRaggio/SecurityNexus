# Testing Guide - Monitoring Engine

This document describes how to test the Monitoring Engine with a local blockchain.

## Prerequisites

1. **Local Substrate Node**
   - You need a Substrate node running locally
   - Default port: `ws://127.0.0.1:9944`

### Option 1: Using substrate-contracts-node

```bash
# Install substrate-contracts-node
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git

# Run in development mode
substrate-contracts-node --dev
```

### Option 2: Using Polkadot

```bash
# Clone polkadot-sdk
git clone https://github.com/paritytech/polkadot-sdk.git
cd polkadot-sdk

# Build and run
cargo build --release
./target/release/polkadot --dev
```

## Running Tests

### 1. Unit Tests (no blockchain required)

```bash
cd packages/monitoring-engine
cargo test --lib
```

**Expected result:** 17 tests passing

### 2. Integration Tests (without blockchain)

```bash
cargo test --test connection_tests
```

**Expected result:** 4 tests passing, 2 ignored

### 3. Integration Tests (WITH local blockchain)

First, ensure you have a node running at `ws://127.0.0.1:9944`, then:

```bash
cargo test --test connection_tests -- --ignored --test-threads=1
```

**Expected result:**
- `test_connection_to_local_chain` - ✓ PASS
- `test_block_subscription` - ✓ PASS (verifies blocks_processed > 0)

### 4. Tests with detailed output

To see tracing logs during tests:

```bash
RUST_LOG=monitoring_engine=debug cargo test --test connection_tests -- --ignored --nocapture --test-threads=1
```

You should see logs like:
```
INFO  monitoring_engine: Connecting to Substrate node at ws://127.0.0.1:9944
INFO  monitoring_engine: Successfully connected to Substrate node
INFO  monitoring_engine: Starting block monitoring
INFO  monitoring_engine: Subscribing to finalized blocks on development
DEBUG monitoring_engine: Received block #123 (hash: 0x...) on development
INFO  monitoring_engine: Starting event monitoring
DEBUG monitoring_engine: Block #123 on development contains 5 events
```

## Manual Verification

### Option 1: Using the binary (RECOMMENDED)

**IMPORTANT:** This is the CORRECT way to run the monitoring engine.

```bash
# 1. Ensure the binary is compiled
cargo build --release --package monitoring-engine

# 2. Export environment variables (REQUIRED)
export WS_ENDPOINT="ws://127.0.0.1:9944"
export CHAIN_NAME="local-dev"
export RUST_LOG=monitoring_engine=info

# 3. Run the monitoring engine
./target/release/monitoring-engine

# Or with cargo:
# cargo run --release --package monitoring-engine
```

You will see output similar to:
```
INFO  monitoring_engine: Starting Polkadot Security Nexus - Monitoring Engine
INFO  monitoring_engine: Configuration:
INFO  monitoring_engine:   WebSocket: ws://127.0.0.1:9944
INFO  monitoring_engine:   Chain: local-dev
INFO  monitoring_engine: Successfully connected to Substrate node
INFO  monitoring_engine: Processing block #123 on local-dev
```

### Option 2: Create a custom program

If you need advanced customization, you can create a program:

```rust
use monitoring_engine::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let config = MonitorConfig {
        ws_endpoint: "ws://127.0.0.1:9944".to_string(),
        chain_name: "local-dev".to_string(),
        enable_mempool: false,
        enable_blocks: true,
        enable_events: true,
        alert_webhook: None,
        min_alert_severity: AlertSeverity::Low,
        buffer_size: 100,
        max_reconnect_attempts: 5,
    };

    let engine = MonitoringEngine::new(config);

    println!("Starting monitoring engine...");
    engine.start().await?;

    // Wait for blocks
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    let stats = engine.get_stats().await;
    println!("Statistics:");
    println!("  - Is running: {}", stats.is_running);
    println!("  - Blocks processed: {}", stats.blocks_processed);
    println!("  - Transactions analyzed: {}", stats.transactions_analyzed);
    println!("  - Alerts triggered: {}", stats.alerts_triggered);

    engine.stop().await?;
    println!("Engine stopped");

    Ok(())
}
```

## Troubleshooting

### Error: "Connection timeout"
- Verify the node is running: `curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://localhost:9944`
- Verify the correct port (9944 by default)

### Error: "Failed to subscribe to blocks"
- Verify the node is in dev mode: `--dev`
- Check the node logs for errors

### No blocks are being processed
- In dev mode, blocks are produced when there are transactions
- You can force block production with `--sealing instant`

## Benchmarks

```bash
# Compile benchmarks
cargo bench --no-run

# Run benchmarks
cargo bench

# View results
open target/criterion/report/index.html
```

## Code Quality

```bash
# Linter
cargo clippy -- -D warnings

# Formatting
cargo fmt

# Documentation
cargo doc --no-deps --open
```

## Next Steps

Once tests with local blockchain pass:
- [ ] Implement automatic reconnection logic
- [ ] Implement transaction extraction from blocks
- [ ] Connect detectors with event processing pipeline
- [ ] Story 3.2: Mempool Monitoring
- [ ] Story 3.3: Flash Loan Detector
