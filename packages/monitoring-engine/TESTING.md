# Testing Guide - Monitoring Engine

Este documento describe cómo probar el Monitoring Engine con una blockchain local.

## Prerrequisitos

1. **Substrate Node Local**
   - Necesitas tener un nodo Substrate corriendo localmente
   - Puerto por defecto: `ws://127.0.0.1:9944`

### Opción 1: Usar substrate-contracts-node

```bash
# Instalar substrate-contracts-node
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git

# Ejecutar en modo desarrollo
substrate-contracts-node --dev
```

### Opción 2: Usar Polkadot

```bash
# Clonar polkadot-sdk
git clone https://github.com/paritytech/polkadot-sdk.git
cd polkadot-sdk

# Build y ejecutar
cargo build --release
./target/release/polkadot --dev
```

## Ejecutar Tests

### 1. Unit Tests (no requieren blockchain)

```bash
cd packages/monitoring-engine
cargo test --lib
```

**Resultado esperado:** 17 tests passing

### 2. Integration Tests (sin blockchain)

```bash
cargo test --test connection_tests
```

**Resultado esperado:** 4 tests passing, 2 ignored

### 3. Integration Tests (CON blockchain local)

Primero, asegúrate de tener un nodo corriendo en `ws://127.0.0.1:9944`, luego:

```bash
cargo test --test connection_tests -- --ignored --test-threads=1
```

**Resultado esperado:**
- `test_connection_to_local_chain` - ✓ PASS
- `test_block_subscription` - ✓ PASS (verifica que blocks_processed > 0)

### 4. Tests con output detallado

Para ver los logs de tracing durante los tests:

```bash
RUST_LOG=monitoring_engine=debug cargo test --test connection_tests -- --ignored --nocapture --test-threads=1
```

Deberías ver logs como:
```
INFO  monitoring_engine: Connecting to Substrate node at ws://127.0.0.1:9944
INFO  monitoring_engine: Successfully connected to Substrate node
INFO  monitoring_engine: Starting block monitoring
INFO  monitoring_engine: Subscribing to finalized blocks on development
DEBUG monitoring_engine: Received block #123 (hash: 0x...) on development
INFO  monitoring_engine: Starting event monitoring
DEBUG monitoring_engine: Block #123 on development contains 5 events
```

## Verificación Manual

Puedes crear un pequeño programa para verificar manualmente:

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
- Verifica que el nodo esté corriendo: `curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://localhost:9944`
- Verifica el puerto correcto (9944 por defecto)

### Error: "Failed to subscribe to blocks"
- Verifica que el nodo esté en modo dev: `--dev`
- Revisa los logs del nodo para ver errores

### No se procesan bloques
- En modo dev, los bloques se producen cuando hay transacciones
- Puedes forzar la producción de bloques con `--sealing instant`

## Benchmarks

```bash
# Compilar benchmarks
cargo bench --no-run

# Ejecutar benchmarks
cargo bench

# Ver resultados
open target/criterion/report/index.html
```

## Code Quality

```bash
# Linter
cargo clippy -- -D warnings

# Formateo
cargo fmt

# Documentación
cargo doc --no-deps --open
```

## Próximos Pasos

Una vez que los tests con blockchain local pasen:
- [ ] Implementar automatic reconnection logic
- [ ] Implementar transaction extraction from blocks
- [ ] Conectar detectors con el event processing pipeline
- [ ] Story 3.2: Mempool Monitoring
- [ ] Story 3.3: Flash Loan Detector
