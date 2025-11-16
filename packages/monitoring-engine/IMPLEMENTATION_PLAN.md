# Monitoring Engine Implementation Plan
## Professional Development with TDD

**Status:** Phase 2 - Core Feature Enhancement
**Approach:** Test-Driven Development (TDD)
**Timeline:** 2 days (hackathon development)
**Competition Duration:** 3 days total

---

## 📋 Current State Analysis

### ✅ What Exists
- Basic module structure (lib.rs, types.rs)
- Core types: Alert, Transaction, ChainEvent, DetectionResult
- MonitoringEngine skeleton with start/stop
- AlertManager stub
- Detector trait and stubs for FlashLoan, MEV, VolumeAnomaly
- Basic unit tests (3 tests)
- Dependencies: tokio, serde, tracing, anyhow, thiserror

### ❌ What's Missing
- Real Substrate connection (subxt integration)
- Actual detector implementations
- Integration tests
- REST API server
- Webhook delivery system
- Database persistence
- Real mempool/block/event monitoring

---

## 🎯 Implementation Phases

### Phase 1: Foundation & Testing Infrastructure (Day 1 - 6 hours)
**Story 3.1 Enhanced + Testing Setup**

#### Tasks:
1. ✅ Add subxt dependency for Substrate connection
2. ✅ Create comprehensive test infrastructure
3. ✅ Implement mock chain for testing
4. ✅ Add integration test structure
5. ✅ Implement real node connection with subxt
6. ✅ Add connection health checks
7. ✅ Implement reconnection logic

#### Acceptance Criteria (Story 3.1):
- [x] WebSocket connection to Substrate node
- [x] New blocks subscription
- [ ] Pending transactions subscription
- [ ] Automatic reconnection if connection lost
- [ ] Connection event logging
- [ ] Support for multiple chains simultaneously
- [ ] 90%+ test coverage

#### Test Coverage Goals:
- Unit tests for connection logic
- Integration tests with local node
- Mock tests for error scenarios
- Performance benchmarks

---

### Phase 2: Mempool Monitoring (Day 1-2 - 4 hours)
**Story 3.2: Mempool Monitoring**

#### Tasks:
1. Implement transaction pool subscription
2. Parse transaction data (call, params, sender)
3. Store transactions in-memory buffer
4. Add transaction indexing
5. Performance optimization (100+ tx/sec)
6. Comprehensive testing

#### Acceptance Criteria:
- [ ] Pending transactions monitoring
- [ ] Transaction data parsing
- [ ] Call and parameter extraction
- [ ] In-memory indexing
- [ ] Performance: processing 100+ tx/second
- [ ] Detailed logging

#### Tests:
- Unit: Transaction parsing
- Integration: Real mempool subscription
- Performance: Throughput benchmarks
- Mock: Error handling

---

### Phase 3: Flash Loan Detector (Day 2 - 3 hours)
**Story 3.3: Flash Loan Attack Detector**

#### Tasks:
1. Design flash loan detection algorithm
2. Implement pattern matching (borrow → manipulate → repay)
3. Add balance change analysis
4. Implement confidence scoring
5. Alert generation
6. TDD: Write tests first!

#### Acceptance Criteria:
- [ ] Detection of pattern: borrow + manipulation + repay in same block
- [ ] Abnormal balance change analysis (>50% in one tx)
- [ ] Critical severity alert
- [ ] Detection latency < 3 seconds
- [ ] False positive rate < 5%
- [ ] Historical data of known attacks for testing

#### TDD Approach:
```rust
// Test cases to write FIRST:
1. test_flash_loan_detection_basic()
2. test_flash_loan_detection_complex()
3. test_no_false_positive_on_normal_swap()
4. test_confidence_scoring()
5. test_alert_generation()
6. benchmark_detection_latency()
```

---

### Phase 4: Alert System with Webhooks (Day 2 - 2 hours)
**Story 3.6: Alert System with Webhooks**

#### Tasks:
1. Implement AlertManager with delivery queue
2. Add webhook HTTP client
3. Implement retry logic with exponential backoff
4. Add alert de-duplication
5. Delivery logging and tracking
6. Error handling and dead letter queue

#### Acceptance Criteria:
- [ ] Webhook configuration via API
- [ ] POST request to webhook URL when alert occurs
- [ ] Complete JSON payload
- [ ] Retry logic (3 attempts with backoff)
- [ ] Alert de-duplication (no duplicates in 5 min)
- [ ] Delivery logging

#### Tests:
- Mock webhook server for testing
- Retry mechanism tests
- De-duplication tests
- Error scenario tests

---

### Phase 5: Monitoring REST API (Day 1 - 4 hours)
**Story 3.7: Monitoring REST API**

#### Tasks:
1. Add actix-web dependency
2. Implement REST endpoints
3. Add authentication with API keys
4. Implement rate limiting
5. Generate OpenAPI/Swagger docs
6. Integration tests

#### Acceptance Criteria:
- [ ] GET /alerts: Alert list with pagination
- [ ] GET /alerts/:id: Specific alert detail
- [ ] POST /webhooks: Configure webhook
- [ ] GET /stats: Monitoring statistics
- [ ] Authentication with API keys
- [ ] Rate limiting (100 req/min)
- [ ] Swagger/OpenAPI documentation

#### Endpoints to Implement:
```
GET    /health          - Health check
GET    /stats           - Engine statistics
GET    /alerts          - List alerts (paginated)
GET    /alerts/:id      - Get alert details
POST   /alerts/:id/ack  - Acknowledge alert
POST   /webhooks        - Register webhook
GET    /webhooks        - List webhooks
DELETE /webhooks/:id    - Delete webhook
POST   /webhooks/:id/test - Test webhook
```

---

### Phase 6: Additional Detectors (Day 3 - 4 hours)
**Stories 3.4 & 3.5: Oracle & Governance Detectors**

#### Oracle Manipulation Detector (Story 3.4):
- Monitor price feed deviations
- Detect >10% price changes in single block
- Correlate with abnormal volume
- Integration with multiple oracle types

#### Governance Attack Detector (Story 3.5):
- Monitor governance proposals
- Detect abnormal voting patterns
- Last-minute vote surge detection
- Whale voting alerts (>5% of supply)

---

## 🧪 Testing Strategy

### Test Pyramid:
```
        /\
       /  \  E2E (10%)
      /____\
     /      \ Integration (30%)
    /________\
   /          \ Unit Tests (60%)
  /__________\
```

### Test Types:

1. **Unit Tests** (packages/monitoring-engine/src/**/*.rs)
   - Every function has tests
   - Mock external dependencies
   - Fast execution (<1s total)
   - 90%+ code coverage

2. **Integration Tests** (packages/monitoring-engine/tests/*.rs)
   - Test with real Substrate node (local dev chain)
   - Test detector combinations
   - Test end-to-end flows
   - Can be slower (30s-1min)

3. **Performance Benchmarks** (benches/*.rs)
   - Transaction processing throughput
   - Detection latency
   - Memory usage
   - Regression tracking

4. **E2E Tests** (tests/e2e/*.rs)
   - Full system tests
   - Real chain + real detectors + real alerts
   - Smoke tests for deployment

### Test Infrastructure:

```rust
// tests/common/mod.rs - Shared test utilities
pub mod mock_chain;
pub mod test_transactions;
pub mod assertions;

// tests/integration/
- connection_tests.rs
- mempool_tests.rs
- detector_tests.rs
- alert_tests.rs

// benches/
- detection_benchmarks.rs
- throughput_benchmarks.rs
```

---

## 📦 Dependencies to Add

```toml
[dependencies]
# Substrate connection
subxt = { version = "0.35", features = ["substrate-compat"] }
sp-core = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-v1.16.0" }
sp-runtime = { git = "https://github.com/paritytech/polkadot-sdk", tag = "polkadot-v1.16.0" }

# Web server
actix-web = "4"
actix-cors = "0.7"

# Additional utilities
dashmap = "5"  # Concurrent HashMap
governor = "0.6"  # Rate limiting

[dev-dependencies]
# Testing
tokio-test = "0.4"
wiremock = "0.6"  # Mock HTTP server
criterion = "0.5"  # Benchmarking
proptest = "1.0"  # Property-based testing

[[bench]]
name = "detection_benchmarks"
harness = false
```

---

## 🎯 Definition of Done

For each story to be marked as COMPLETED:

1. ✅ **Code Complete**
   - All acceptance criteria met
   - No TODOs in production code
   - Follows Rust best practices

2. ✅ **Tests Pass**
   - All unit tests pass
   - All integration tests pass
   - Code coverage >90%
   - Benchmarks within acceptable range

3. ✅ **Documentation**
   - Public APIs documented with rustdoc
   - Examples provided
   - README updated

4. ✅ **Code Review**
   - Peer reviewed
   - No clippy warnings
   - Formatted with rustfmt

5. ✅ **User Story Updated**
   - Marked as completed in USER_STORIES.md
   - Acceptance criteria checked off
   - Location documented

---

## 📊 Progress Tracking

### Current Sprint Points: 27 total
- [x] Story 3.1: Parachain Connection (3 pts) - ✅ Basic structure
- [ ] Story 3.2: Mempool Monitoring (4 pts)
- [ ] Story 3.3: Flash Loan Detector (5 pts)
- [ ] Story 3.4: Oracle Manipulation (5 pts)
- [ ] Story 3.5: Governance Attack (4 pts)
- [ ] Story 3.6: Alert System (3 pts)
- [ ] Story 3.7: REST API (3 pts)

### Development Speed: 27 story points in 2 days (hackathon pace)

---

## 🚀 Getting Started

### Step 1: Run existing tests
```bash
cd packages/monitoring-engine
cargo test
```

### Step 2: Start local dev chain
```bash
# In another terminal
./target/release/security-nexus-node --dev
```

### Step 3: Run integration tests (after implementation)
```bash
cargo test --test integration_tests
```

### Step 4: Run benchmarks
```bash
cargo bench
```

---

**Next Steps:** Start with Phase 1 - Add subxt and implement real node connection with comprehensive tests.

