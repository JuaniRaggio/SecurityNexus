# SecurityNexus - Implementation Documentation

## Overview

SecurityNexus is a comprehensive blockchain security monitoring system for Polkadot parachains, with specialized detection for Hyperbridge (cross-chain) and Hydration (DeFi) protocols. It combines real-time threat detection, machine learning feature extraction, and advanced analytics.

Authors: Juan Ignacio Raggio and Victoria Helena Park

## Architecture

### Backend (Rust)
- **Monitoring Engine**: Real-time transaction analysis
- **TimescaleDB**: Time-series database for historical data
- **REST API**: Analytics and export endpoints
- **ML Pipeline**: Feature extraction for future ML models

### Frontend (Next.js)
- **Dashboard**: Real-time monitoring interface
- **Analytics**: Visualizations and trends
- **Specialized Pages**: Hyperbridge and Hydration monitoring

---

## Phase 1: Database Integration

### TimescaleDB Schema

**Tables Created**:
1. `transactions` - All blockchain transactions with hypertable partitioning
2. `detections` - Security detections from all detectors
3. `hyperbridge_messages` - Cross-chain message tracking
4. `hydration_pool_state` - DeFi pool state snapshots
5. `hydration_liquidations` - Liquidation events
6. `ml_features` - Machine learning features (JSONB + vector array)

**Continuous Aggregates**:
- `detector_stats_hourly` - Hourly statistics by detector
- `transaction_stats_hourly` - Transaction volume and success rates

**Files**:
- `packages/monitoring-engine/src/database/mod.rs` - Database client with connection pooling
- `packages/monitoring-engine/src/database/models.rs` - Rust models matching SQL schema
- `packages/monitoring-engine/docker/init.sql` - Complete TimescaleDB schema

**Integration**:
- Every transaction automatically stored in database
- Every detection automatically stored with evidence
- Async storage with error handling (non-blocking)

---

## Phase 2: Hyperbridge Integration

### Cross-Chain Attack Detectors

#### 1. CrossChainBridgeDetector
**File**: `packages/monitoring-engine/src/detectors/hyperbridge.rs:16`

**Detects**:
- Duplicate message relay (same request commitment)
- Spray attacks (multiple destination chains)
- Cross-chain drain attacks (high-value transfers)
- Timeout exploitation (requests without responses)

**Indicators Analyzed**:
- POST/GET requests and responses
- Request commitments (duplicate detection)
- Destination chain diversity
- High-value balance transfers
- Rapid succession of requests (>3 in one tx)

**Confidence Calculation**:
- Duplicate commitments: +0.6
- Rapid succession: +0.2
- Multiple destinations: +0.15
- High value transfer + multiple requests: +0.15

#### 2. StateProofVerificationDetector
**File**: `packages/monitoring-engine/src/detectors/hyperbridge.rs:251`

**Detects**:
- Proof manipulation attempts
- Verification failures
- Multiple proofs for same block height
- Invalid proof structures
- Suspicious relayer behavior

**Indicators Analyzed**:
- State proof submissions
- Consensus proof submissions
- Verification success/failure
- Proof heights (duplicate detection)
- Relayer validity

**Confidence Calculation**:
- Verification failed: +0.7
- Multiple proofs same height: +0.5
- Invalid proof structure: +0.4
- Suspicious relayer: +0.3

---

## Phase 3: Hydration Integration

### DeFi Attack Detectors

#### 1. OmnipoolManipulationDetector
**File**: `packages/monitoring-engine/src/detectors/hydration.rs:15`

**Detects**:
- Flash loan + swap combinations (sandwich attacks)
- Large price impact (>5%)
- Rapid swaps (>2 in one transaction)
- Oracle price deviation (>3%)
- Add/remove liquidity in same transaction

**Pattern Detection**:
- Flash loan pattern: Borrow + Repay events
- Sandwich pattern: Swap + large price impact
- Rapid manipulation: Multiple swaps in single tx

**Confidence Calculation**:
- Flash loan + swap: +0.6
- Large price impact (>5%): +0.3
- Rapid swaps: +0.25
- Oracle deviation: +0.2

#### 2. LiquidityDrainDetector
**File**: `packages/monitoring-engine/src/detectors/hydration.rs:127`

**Detects**:
- Large withdrawals (single or multiple)
- Pool depletion risk
- Suspicious timing patterns
- Coordinated drain attacks

**Indicators**:
- Remove liquidity events
- Withdrawal size relative to pool
- Multiple withdrawals in one tx
- Timing analysis

**Confidence Calculation**:
- Large withdrawal: +0.4
- Multiple withdrawals: +0.3
- Pool depletion risk: +0.25
- Suspicious timing: +0.15

#### 3. CollateralManipulationDetector
**File**: `packages/monitoring-engine/src/detectors/hydration.rs:213`

**Detects**:
- Flash loan + liquidation attacks
- Liquidation cascade risks
- Health factor manipulation (>30% drop)
- Multiple liquidations in one tx

**Pattern Detection**:
- Flash loan liquidation: Borrow + Liquidation events
- Cascade risk: Multiple liquidations
- Health factor analysis

**Confidence Calculation**:
- Flash loan + liquidation: +0.7
- Multiple liquidations: +0.4
- Health factor drop >30%: +0.3
- Collateral change + liquidation: +0.2

---

## Phase 4: ML Feature Pipeline

### Feature Extraction System

**File**: `packages/monitoring-engine/src/ml/features.rs`

**TransactionFeatures** - 33 numerical features extracted per transaction:

#### Transaction Metadata (5 features)
- `block_number` - Block height
- `tx_index` - Position in block
- `tx_success` - Success flag (1.0/0.0)
- `has_signature` - Signature presence
- `nonce` - Transaction nonce

#### Temporal Features (3 features)
- `hour_of_day` - 0-23
- `day_of_week` - 0-6 (Monday = 0)
- `timestamp` - Unix timestamp

#### Event Features (7 features)
- `event_count` - Total events
- `unique_event_types` - Distinct event types
- `has_swap_events` - DEX activity flag
- `has_transfer_events` - Transfer activity flag
- `has_borrow_events` - Lending activity flag
- `has_liquidation_events` - Liquidation flag
- `has_bridge_events` - Cross-chain flag

#### State Change Features (3 features)
- `state_change_count` - Number of state changes
- `state_change_magnitude` - Average bytes changed
- `max_state_change` - Largest state change

#### Behavioral Features (6 features)
- `is_dex_interaction` - DEX operation flag
- `is_lending_interaction` - Lending flag
- `is_bridge_interaction` - Cross-chain flag
- `is_governance_interaction` - Governance flag
- `is_batch_call` - Batch operation flag
- `is_utility_call` - Utility pallet flag

#### Pattern Indicators (4 features)
- `rapid_succession_indicator` - Fast execution (nonce <5)
- `flash_loan_pattern` - Borrow + Repay detection
- `sandwich_risk` - DEX + sequential position
- `cross_chain_activity` - ISMP/Hyperbridge events

#### Complexity Metrics (3 features)
- `call_depth` - Estimated call nesting
- `data_size` - Transaction args size
- `event_diversity` - Shannon entropy of events

#### Network Features (2 features)
- `caller_hash` - Numeric hash of address
- `pallet_category` - Encoded pallet type (0-8)

### Feature Storage

**Database Table**: `ml_features`
- Features stored as JSONB for flexibility
- Feature vector stored as DOUBLE PRECISION[] for ML
- Indexed by timestamp and tx_hash
- Automatic extraction on every transaction

**Integration**:
- `FeatureExtractor` maintains caller history
- Features extracted in `process_transaction()`
- Stored in parallel with transaction data
- No blocking on storage failures

---

## Phase 5: Analytics & Export API

### Database Analytics Methods

**File**: `packages/monitoring-engine/src/database/mod.rs`

#### 1. `get_ml_feature_stats(limit)`
Returns recent ML features with full JSON data.

**Query**:
```sql
SELECT timestamp, tx_hash, caller, pallet, call_name, features
FROM ml_features
ORDER BY timestamp DESC
LIMIT $1
```

#### 2. `get_attack_trends(hours)`
Analyzes attack pattern trends over time.

**Query**:
```sql
SELECT
  date_trunc('hour', timestamp) as hour,
  attack_pattern,
  COUNT(*) as count,
  AVG(confidence) as avg_confidence
FROM detections
WHERE timestamp >= NOW() - INTERVAL '1 hour' * $1
GROUP BY hour, attack_pattern
ORDER BY hour DESC, count DESC
```

#### 3. `get_export_data(hours)`
Full detection data with transaction context (JOIN).

**Query**:
```sql
SELECT
  d.timestamp, d.detection_id, d.tx_hash, d.detector_name,
  d.attack_pattern, d.confidence, d.severity, d.description,
  d.evidence, t.caller, t.pallet, t.call_name, t.success, t.chain
FROM detections d
LEFT JOIN transactions t ON d.tx_hash = t.tx_hash
WHERE d.timestamp >= NOW() - INTERVAL '1 hour' * $hours
ORDER BY d.timestamp DESC
```

### REST API Endpoints

**File**: `packages/monitoring-engine/src/api.rs`

#### Analytics Endpoints

**1. GET `/api/analytics/ml-features?limit=100`**
- Returns ML features with configurable limit
- Default: 100 most recent features
- Response: JSON array with complete features

**2. GET `/api/analytics/attack-trends?hours=24`**
- Attack pattern trends by hour
- Default: last 24 hours
- Response: Count and avg confidence per pattern

**3. GET `/api/analytics/detector-stats?hours=24`**
- Detector statistics from continuous aggregates
- Groups by detector, chain, and attack pattern
- Response: Detections, confidence, critical count

#### Export Endpoints

**4. GET `/api/export/json?hours=48`**
- Full export in JSON format
- Optional: filter by hours (default: 1000 records)
- Headers: `Content-Disposition: attachment`

**5. GET `/api/export/csv?hours=48`**
- Full export in CSV format
- Dynamic CSV generation from JSON
- Headers: `Content-Type: text/csv`
- Columns: timestamp, detection_id, tx_hash, detector_name, attack_pattern, confidence, severity, description, caller, pallet, call_name, success, chain

**Error Handling**:
- Returns 503 if database unavailable
- Returns 500 with error message on query failure
- Query parameters optional with sensible defaults

---

## Phase 6: Dashboard Pages

### Created Pages

#### 1. Hyperbridge Monitoring (`/hyperbridge`)
**File**: `packages/web-dashboard/src/app/hyperbridge/page.tsx`

**Features**:
- Real-time cross-chain attack monitoring
- Time range selector (6h, 12h, 24h, 48h, 168h)
- Stats cards: Total Detections, Bridge Attacks, Proof Issues
- Detection timeline table with:
  - Timestamp
  - Attack pattern (color-coded)
  - Count
  - Average confidence
- Export buttons (CSV/JSON)
- Dark mode support
- Responsive design

**API Integration**:
- Fetches from `/api/analytics/attack-trends`
- Filters for CrossChain and StateProof patterns
- Auto-refresh on time range change

**Design**:
- Gradient header: cyan → blue → indigo
- Icon: Bridge (lucide-react)
- Color coding: Orange for CrossChain, Purple for StateProof

---

## System Statistics

### Detectors Implemented: 9

1. **Flash Loan Detector** - Generic flash loan attacks
2. **MEV Detector** - Sandwich, frontrunning, backrunning
3. **Volume Anomaly Detector** - Unusual transaction volumes
4. **FrontRunning Detector** - Transaction ordering attacks
5. **Cross-Chain Bridge Detector** - Message replay, spray attacks
6. **State Proof Verification Detector** - Proof manipulation
7. **Omnipool Manipulation Detector** - DEX manipulation
8. **Liquidity Drain Detector** - Pool drain attacks
9. **Collateral Manipulation Detector** - Liquidation attacks

### ML Features: 33

All features automatically extracted per transaction and stored in database.

### API Endpoints: 13

**Existing**:
- `/api/health` - Health check
- `/api/stats` - Engine statistics
- `/api/detectors` - Detector list
- `/api/alerts` - Recent alerts
- `/api/alerts/unacknowledged` - Unacked alerts
- `/api/alerts/{id}/acknowledge` - Acknowledge alert
- `/api/chains` - Available chains
- `/api/chains/current` - Current chain
- `/api/chains/switch` - Switch chain

**New (Phase 5)**:
- `/api/analytics/ml-features` - ML feature statistics
- `/api/analytics/attack-trends` - Attack trends over time
- `/api/analytics/detector-stats` - Detector statistics
- `/api/export/json` - JSON export
- `/api/export/csv` - CSV export

### Database Tables: 8

**Main Tables**:
1. `transactions` (hypertable)
2. `detections`
3. `hyperbridge_messages`
4. `hydration_pool_state`
5. `hydration_liquidations`
6. `ml_features`

**Continuous Aggregates**:
7. `detector_stats_hourly`
8. `transaction_stats_hourly`

---

## Running the System

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js/pnpm
npm install -g pnpm

# Install Docker for TimescaleDB
docker --version
```

### Start TimescaleDB

```bash
cd packages/monitoring-engine
docker-compose up -d

# Initialize schema
docker exec -i polcacadot-timescaledb psql -U postgres -d polcacadot < docker/init.sql
```

### Start Monitoring Engine

```bash
cd packages/monitoring-engine

# Set database URL (optional, works without DB)
export DATABASE_URL="postgresql://postgres:postgres@localhost:5432/polcacadot"

# Run engine
cargo run --release
```

**API Available at**: http://localhost:8080

### Start Web Dashboard

```bash
cd packages/web-dashboard
pnpm install
pnpm dev
```

**Dashboard Available at**: http://localhost:3000

---

## Configuration

### Monitoring Engine

**File**: `packages/monitoring-engine/config.toml`

```toml
[chain]
name = "Polkadot"
ws_endpoint = "wss://rpc.polkadot.io"

[monitoring]
min_alert_severity = "Medium"  # Low, Medium, High, Critical
alert_webhook = ""  # Optional webhook URL

[api]
bind_address = "127.0.0.1:8080"

[database]
# Optional - system works without database
url = "postgresql://postgres:postgres@localhost:5432/polkacadot"
max_connections = 10
```

### Environment Variables

```bash
# Required
WS_ENDPOINT=wss://rpc.polkadot.io

# Optional
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/polcacadot
DATABASE_MAX_CONNECTIONS=10
API_BIND_ADDRESS=127.0.0.1:8080
MIN_ALERT_SEVERITY=Medium
```

---

## Development

### Adding a New Detector

1. Create detector file in `packages/monitoring-engine/src/detectors/`
2. Implement `Detector` trait:
   ```rust
   #[async_trait]
   impl Detector for MyDetector {
       fn name(&self) -> &str { "My Detector" }
       async fn analyze_transaction(&self, ctx: &TransactionContext) -> DetectionResult {
           // Detection logic
       }
   }
   ```
3. Add to `src/detectors/mod.rs`
4. Add to `MonitoringEngine::initialize_detectors()`
5. Add stats entry in `EngineState::default()`

### Adding New ML Features

1. Add fields to `TransactionFeatures` struct in `src/ml/features.rs`
2. Extract features in `FeatureExtractor::extract_features()`
3. Add to `to_vector()` method for ML model input
4. Add to `feature_names()` for interpretability

### Adding API Endpoints

1. Create handler function in `src/api.rs`:
   ```rust
   async fn my_endpoint(data: web::Data<ApiState>) -> HttpResponse {
       // Handler logic
   }
   ```
2. Add route in `configure_routes()`:
   ```rust
   .route("/my-endpoint", web::get().to(my_endpoint))
   ```

---

## Testing

### Unit Tests

```bash
cd packages/monitoring-engine
cargo test
```

### Integration Tests

```bash
# Start monitoring engine
cargo run

# Test endpoints
curl http://localhost:8080/api/health
curl http://localhost:8080/api/stats
curl http://localhost:8080/api/analytics/attack-trends?hours=24
```

### Database Tests

```bash
# Verify database connection
psql -U postgres -h localhost -d polcacadot

# Check tables
\dt

# Query detections
SELECT * FROM detections LIMIT 10;

# Check ML features
SELECT * FROM ml_features LIMIT 5;
```

---

## Performance

### Optimizations Implemented

1. **Async Processing**: All I/O operations are async
2. **Connection Pooling**: Database connections pooled (default: 10)
3. **Non-Blocking Storage**: Database writes don't block detection
4. **Hypertables**: Automatic time-based partitioning
5. **Continuous Aggregates**: Pre-computed hourly statistics
6. **Batch Analysis**: MEV detector analyzes transaction batches

### Scalability

- **Horizontal**: Multiple engine instances can write to same database
- **Vertical**: Connection pool size adjustable
- **Storage**: TimescaleDB compression policies (not yet configured)
- **API**: Actix-web handles concurrent requests efficiently

---

## Security Considerations

### Attack Surface

1. **API Endpoints**: Rate limiting not yet implemented
2. **Database Injection**: Using prepared statements (safe)
3. **CORS**: Configured for development (adjust for production)
4. **Authentication**: Not implemented (add for production)

### Recommendations for Production

1. Add API authentication (JWT tokens)
2. Implement rate limiting
3. Configure CORS for specific domains
4. Use TLS for database connections
5. Set up database backups
6. Configure TimescaleDB retention policies
7. Add monitoring alerts (Prometheus/Grafana)

---

## Future Enhancements

### Planned Features

1. **ML Model Training**: Use extracted features to train attack prediction models
2. **Real-time Alerts**: WebSocket connections for live alerts
3. **Multi-chain Support**: Monitor multiple parachains simultaneously
4. **Advanced Visualizations**: Charts and graphs in Analytics page
5. **Alert Routing**: Route alerts to different channels (Slack, Discord, PagerDuty)
6. **Historical Analysis**: Deep-dive analysis of past attacks
7. **Attack Playback**: Replay detected attacks for analysis

### Integration Opportunities

1. **Polkadot.js Integration**: Direct wallet connections
2. **Subscan Integration**: Link to block explorer
3. **On-chain Governance**: Submit findings as governance proposals
4. **External Threat Intel**: Integrate with threat intelligence feeds

---

## Troubleshooting

### Common Issues

**1. Database Connection Fails**
```bash
# Check if TimescaleDB is running
docker ps | grep timescaledb

# Check connection string
echo $DATABASE_URL

# Restart database
docker-compose restart
```

**2. No Detections Appearing**
- Check if monitoring engine is connected to chain
- Verify detector configuration
- Check minimum alert severity
- Look for errors in engine logs

**3. API Returns 503**
- Database is unavailable or not configured
- System works without database but analytics endpoints require it

**4. Dashboard Shows No Data**
- Verify API URL in frontend (default: localhost:8080)
- Check CORS configuration
- Verify monitoring engine is running

---

## Credits

**Authors**:
- Juan Ignacio Raggio
- Victoria Helena Park

**Technologies**:
- Rust
- Subxt (Substrate client)
- TimescaleDB
- Next.js
- Tailwind CSS
- Actix-web

**Repository**: https://github.com/JuaniRaggio/SecurityNexus

**Hackathon**: Polkadot Sub0 Hack 2024
- Link: https://luma.com/sub0hack
