-- ============================================
-- Security Nexus - TimescaleDB Schema
-- ============================================
-- Database schema for storing vulnerable transactions,
-- attack patterns, and ML features for prediction

-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;

-- ============================================
-- 1. TRANSACTIONS TABLE (Hypertable)
-- ============================================
-- Stores all blockchain transactions for analysis
CREATE TABLE IF NOT EXISTS transactions (
    -- Temporal
    timestamp TIMESTAMPTZ NOT NULL,

    -- Transaction identifiers
    tx_hash TEXT PRIMARY KEY,
    block_number BIGINT NOT NULL,
    block_hash TEXT NOT NULL,

    -- Chain info
    chain TEXT NOT NULL,

    -- Transaction details
    pallet TEXT NOT NULL,
    call_name TEXT NOT NULL,
    caller TEXT NOT NULL,
    success BOOLEAN NOT NULL,
    nonce BIGINT,

    -- Flexible storage for call arguments
    args JSONB,

    -- Gas/fees
    gas_used BIGINT,
    fee_paid NUMERIC,

    -- Created timestamp
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Convert to hypertable (partitioned by time)
SELECT create_hypertable('transactions', 'timestamp', if_not_exists => TRUE);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_tx_chain_time ON transactions(chain, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_tx_pallet_time ON transactions(pallet, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_tx_caller ON transactions(caller);
CREATE INDEX IF NOT EXISTS idx_tx_block ON transactions(block_number);

-- ============================================
-- 2. EVENTS TABLE (Hypertable)
-- ============================================
-- Stores blockchain events emitted by transactions
CREATE TABLE IF NOT EXISTS events (
    -- Temporal
    timestamp TIMESTAMPTZ NOT NULL,

    -- Event identifiers
    event_id TEXT PRIMARY KEY,
    block_number BIGINT NOT NULL,
    tx_hash TEXT REFERENCES transactions(tx_hash) ON DELETE CASCADE,
    event_index INT NOT NULL,

    -- Chain info
    chain TEXT NOT NULL,

    -- Event details
    pallet TEXT NOT NULL,
    event_name TEXT NOT NULL,

    -- Event data
    data JSONB,
    topics TEXT[],

    -- Created timestamp
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Convert to hypertable
SELECT create_hypertable('events', 'timestamp', if_not_exists => TRUE);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_event_chain_time ON events(chain, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_event_type ON events(pallet, event_name);
CREATE INDEX IF NOT EXISTS idx_event_tx ON events(tx_hash);

-- ============================================
-- 3. DETECTIONS TABLE (Hypertable)
-- ============================================
-- Stores attack pattern detections and alerts
CREATE TABLE IF NOT EXISTS detections (
    -- Temporal
    timestamp TIMESTAMPTZ NOT NULL,

    -- Detection identifiers
    detection_id TEXT PRIMARY KEY,
    tx_hash TEXT REFERENCES transactions(tx_hash) ON DELETE CASCADE,
    block_number BIGINT NOT NULL,

    -- Chain info
    chain TEXT NOT NULL,

    -- Detection details
    detector_name TEXT NOT NULL,
    attack_pattern TEXT NOT NULL,
    confidence DOUBLE PRECISION NOT NULL CHECK (confidence >= 0 AND confidence <= 1),
    severity TEXT NOT NULL CHECK (severity IN ('low', 'medium', 'high', 'critical')),

    -- Alert information
    description TEXT,
    evidence JSONB,
    metadata JSONB,
    recommended_actions TEXT[],

    -- Status
    acknowledged BOOLEAN DEFAULT FALSE,
    acknowledged_at TIMESTAMPTZ,
    acknowledged_by TEXT,

    -- Created timestamp
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Convert to hypertable
SELECT create_hypertable('detections', 'timestamp', if_not_exists => TRUE);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_detection_severity ON detections(severity, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_detection_pattern ON detections(attack_pattern, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_detection_chain ON detections(chain, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_detection_ack ON detections(acknowledged, timestamp DESC);

-- ============================================
-- 4. ML FEATURES TABLE (Hypertable)
-- ============================================
-- Stores extracted features for machine learning
CREATE TABLE IF NOT EXISTS ml_features (
    -- Temporal
    timestamp TIMESTAMPTZ NOT NULL,

    -- Transaction reference
    tx_hash TEXT REFERENCES transactions(tx_hash) ON DELETE CASCADE,
    chain TEXT NOT NULL,

    -- Basic transaction features
    gas_used BIGINT,
    num_events INT,
    num_calls INT,
    complexity_score DOUBLE PRECISION,

    -- DeFi features (Hydration)
    price_impact_percent DOUBLE PRECISION,
    slippage_percent DOUBLE PRECISION,
    liquidity_change_percent DOUBLE PRECISION,
    pool_volume_ratio DOUBLE PRECISION,

    -- Flash loan features
    has_borrow BOOLEAN DEFAULT FALSE,
    has_repay BOOLEAN DEFAULT FALSE,
    borrow_amount NUMERIC,
    borrow_repay_time_diff INT, -- seconds
    dex_interaction_count INT DEFAULT 0,

    -- Cross-chain features (Hyperbridge)
    is_cross_chain BOOLEAN DEFAULT FALSE,
    source_chain TEXT,
    dest_chain TEXT,
    bridge_amount NUMERIC,
    proof_verification_time INT, -- milliseconds

    -- Temporal features
    hour_of_day INT CHECK (hour_of_day >= 0 AND hour_of_day < 24),
    day_of_week INT CHECK (day_of_week >= 0 AND day_of_week < 7),
    time_since_last_similar_tx INT, -- seconds
    tx_frequency_last_hour INT,

    -- Labels (for supervised learning)
    is_attack BOOLEAN,
    attack_type TEXT,
    attack_severity TEXT,

    -- Created timestamp
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Convert to hypertable
SELECT create_hypertable('ml_features', 'timestamp', if_not_exists => TRUE);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_ml_chain_time ON ml_features(chain, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_ml_labeled ON ml_features(is_attack, timestamp DESC) WHERE is_attack IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_ml_attack_type ON ml_features(attack_type) WHERE attack_type IS NOT NULL;

-- ============================================
-- 5. HYPERBRIDGE MESSAGES TABLE (Hypertable)
-- ============================================
-- Stores cross-chain messages via Hyperbridge/ISMP
CREATE TABLE IF NOT EXISTS hyperbridge_messages (
    -- Temporal
    timestamp TIMESTAMPTZ NOT NULL,

    -- Message identifiers
    request_commitment TEXT PRIMARY KEY,
    tx_hash TEXT REFERENCES transactions(tx_hash) ON DELETE CASCADE,

    -- Cross-chain info
    source_chain TEXT NOT NULL,
    dest_chain TEXT NOT NULL,
    nonce BIGINT NOT NULL,

    -- Request details
    request_type TEXT NOT NULL CHECK (request_type IN ('post', 'get')),
    request_data JSONB,
    response_data JSONB,

    -- Proof verification
    proof_verified BOOLEAN DEFAULT FALSE,
    consensus_proof BYTEA,
    state_proof BYTEA,
    verification_height BIGINT,

    -- Relayer information
    relayer_address TEXT,
    relayer_fee NUMERIC,

    -- Timing
    timeout_timestamp TIMESTAMPTZ,
    response_timestamp TIMESTAMPTZ,

    -- Status
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'verified', 'executed', 'timeout', 'failed')),

    -- Asset tracking (if token bridge)
    token_address TEXT,
    amount NUMERIC,
    recipient TEXT,

    -- Created timestamp
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Convert to hypertable
SELECT create_hypertable('hyperbridge_messages', 'timestamp', if_not_exists => TRUE);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_hyperbridge_chains ON hyperbridge_messages(source_chain, dest_chain, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_hyperbridge_status ON hyperbridge_messages(status, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_hyperbridge_relayer ON hyperbridge_messages(relayer_address, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_hyperbridge_token ON hyperbridge_messages(token_address) WHERE token_address IS NOT NULL;

-- ============================================
-- 6. HYDRATION POOL STATE TABLE (Hypertable)
-- ============================================
-- Stores snapshots of Hydration DeFi pool state
CREATE TABLE IF NOT EXISTS hydration_pool_state (
    -- Temporal
    timestamp TIMESTAMPTZ NOT NULL,

    -- Pool identification
    pool_id TEXT NOT NULL,
    pool_type TEXT NOT NULL CHECK (pool_type IN ('omnipool', 'stablepool', 'xyk')),

    -- Trading pair (for non-omnipool)
    token_in TEXT,
    token_out TEXT,

    -- Liquidity metrics
    total_liquidity NUMERIC,
    token_reserves JSONB, -- Map of token -> reserve amount

    -- Price data
    price NUMERIC,
    oracle_price NUMERIC,
    oracle_deviation DOUBLE PRECISION,

    -- Volume metrics
    volume_1h NUMERIC,
    volume_24h NUMERIC,
    trade_count_1h INT,

    -- Price impact tracking
    avg_price_impact_1h DOUBLE PRECISION,
    max_price_impact_1h DOUBLE PRECISION,

    -- LP token info
    lp_token_supply NUMERIC,
    lp_token_price NUMERIC,

    -- Created timestamp
    created_at TIMESTAMPTZ DEFAULT NOW(),

    PRIMARY KEY (pool_id, timestamp)
);

-- Convert to hypertable
SELECT create_hypertable('hydration_pool_state', 'timestamp', if_not_exists => TRUE);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_pool_id_time ON hydration_pool_state(pool_id, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_pool_type ON hydration_pool_state(pool_type, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_pool_deviation ON hydration_pool_state(oracle_deviation) WHERE oracle_deviation > 0.05;

-- ============================================
-- 7. HYDRATION LIQUIDATIONS TABLE
-- ============================================
-- Tracks liquidation events for analysis
CREATE TABLE IF NOT EXISTS hydration_liquidations (
    -- Temporal
    timestamp TIMESTAMPTZ NOT NULL,

    -- Liquidation identifiers
    liquidation_id TEXT PRIMARY KEY,
    tx_hash TEXT REFERENCES transactions(tx_hash) ON DELETE CASCADE,
    block_number BIGINT NOT NULL,

    -- Parties involved
    liquidator TEXT NOT NULL,
    borrower TEXT NOT NULL,

    -- Assets
    collateral_token TEXT NOT NULL,
    debt_token TEXT NOT NULL,

    -- Amounts
    collateral_seized NUMERIC NOT NULL,
    debt_covered NUMERIC NOT NULL,
    liquidator_profit NUMERIC,

    -- Borrower state before liquidation
    collateral_ratio_before DOUBLE PRECISION,
    health_factor_before DOUBLE PRECISION,

    -- Price data
    collateral_price NUMERIC,
    debt_price NUMERIC,

    -- Created timestamp
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Convert to hypertable
SELECT create_hypertable('hydration_liquidations', 'timestamp', if_not_exists => TRUE);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_liq_time ON hydration_liquidations(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_liq_liquidator ON hydration_liquidations(liquidator, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_liq_borrower ON hydration_liquidations(borrower, timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_liq_profit ON hydration_liquidations(liquidator_profit DESC) WHERE liquidator_profit > 0;

-- ============================================
-- CONTINUOUS AGGREGATES
-- ============================================
-- Pre-computed views for fast dashboard queries

-- Hourly detector statistics
CREATE MATERIALIZED VIEW IF NOT EXISTS detector_stats_hourly
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', timestamp) AS hour,
    detector_name,
    chain,
    attack_pattern,
    COUNT(*) as detection_count,
    AVG(confidence) as avg_confidence,
    COUNT(*) FILTER (WHERE severity = 'critical') as critical_count,
    COUNT(*) FILTER (WHERE severity = 'high') as high_count,
    COUNT(*) FILTER (WHERE severity = 'medium') as medium_count,
    COUNT(*) FILTER (WHERE severity = 'low') as low_count,
    COUNT(*) FILTER (WHERE acknowledged = true) as acknowledged_count
FROM detections
GROUP BY hour, detector_name, chain, attack_pattern
WITH NO DATA;

-- Refresh policy for continuous aggregate
SELECT add_continuous_aggregate_policy('detector_stats_hourly',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => TRUE
);

-- Hourly transaction volume per chain
CREATE MATERIALIZED VIEW IF NOT EXISTS tx_volume_hourly
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', timestamp) AS hour,
    chain,
    pallet,
    COUNT(*) as tx_count,
    COUNT(*) FILTER (WHERE success = true) as success_count,
    COUNT(*) FILTER (WHERE success = false) as failed_count,
    SUM(gas_used) as total_gas_used,
    AVG(gas_used) as avg_gas_used
FROM transactions
GROUP BY hour, chain, pallet
WITH NO DATA;

SELECT add_continuous_aggregate_policy('tx_volume_hourly',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => TRUE
);

-- Rolling 24h bridge volume and statistics
CREATE MATERIALIZED VIEW IF NOT EXISTS bridge_volume_24h
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', timestamp) AS hour,
    source_chain,
    dest_chain,
    COUNT(*) as message_count,
    COUNT(*) FILTER (WHERE proof_verified = true) as verified_count,
    COUNT(*) FILTER (WHERE status = 'executed') as executed_count,
    COUNT(*) FILTER (WHERE status = 'failed') as failed_count,
    SUM(relayer_fee) as total_fees,
    SUM(amount) FILTER (WHERE amount IS NOT NULL) as total_volume,
    AVG(EXTRACT(EPOCH FROM (response_timestamp - timestamp))) FILTER (WHERE response_timestamp IS NOT NULL) as avg_response_time_seconds
FROM hyperbridge_messages
GROUP BY hour, source_chain, dest_chain
WITH NO DATA;

SELECT add_continuous_aggregate_policy('bridge_volume_24h',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => TRUE
);

-- Hydration pool metrics (hourly aggregates)
CREATE MATERIALIZED VIEW IF NOT EXISTS pool_metrics_hourly
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', timestamp) AS hour,
    pool_id,
    pool_type,
    AVG(total_liquidity) as avg_liquidity,
    MAX(oracle_deviation) as max_oracle_deviation,
    SUM(volume_1h) as total_volume,
    AVG(avg_price_impact_1h) as avg_price_impact,
    MAX(max_price_impact_1h) as max_price_impact
FROM hydration_pool_state
GROUP BY hour, pool_id, pool_type
WITH NO DATA;

SELECT add_continuous_aggregate_policy('pool_metrics_hourly',
    start_offset => INTERVAL '3 hours',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => TRUE
);

-- ============================================
-- DATA RETENTION POLICIES
-- ============================================
-- Automatically delete old raw data, keep aggregates longer

-- Raw transactions: keep 90 days
SELECT add_retention_policy('transactions', INTERVAL '90 days', if_not_exists => TRUE);

-- Events: keep 90 days
SELECT add_retention_policy('events', INTERVAL '90 days', if_not_exists => TRUE);

-- Detections: keep 365 days (alerts are important historical data)
SELECT add_retention_policy('detections', INTERVAL '365 days', if_not_exists => TRUE);

-- ML features: keep 180 days (for retraining)
SELECT add_retention_policy('ml_features', INTERVAL '180 days', if_not_exists => TRUE);

-- Hyperbridge messages: keep 180 days
SELECT add_retention_policy('hyperbridge_messages', INTERVAL '180 days', if_not_exists => TRUE);

-- Pool state: keep 90 days
SELECT add_retention_policy('hydration_pool_state', INTERVAL '90 days', if_not_exists => TRUE);

-- Liquidations: keep 365 days
SELECT add_retention_policy('hydration_liquidations', INTERVAL '365 days', if_not_exists => TRUE);

-- ============================================
-- COMPRESSION POLICIES
-- ============================================
-- Compress old chunks to save disk space

-- Compress transactions older than 7 days
ALTER TABLE transactions SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'chain,pallet',
    timescaledb.compress_orderby = 'timestamp DESC'
);
SELECT add_compression_policy('transactions', INTERVAL '7 days', if_not_exists => TRUE);

-- Compress events older than 7 days
ALTER TABLE events SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'chain,pallet',
    timescaledb.compress_orderby = 'timestamp DESC'
);
SELECT add_compression_policy('events', INTERVAL '7 days', if_not_exists => TRUE);

-- Compress detections older than 30 days
ALTER TABLE detections SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'chain,attack_pattern,severity',
    timescaledb.compress_orderby = 'timestamp DESC'
);
SELECT add_compression_policy('detections', INTERVAL '30 days', if_not_exists => TRUE);

-- Compress ML features older than 30 days
ALTER TABLE ml_features SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'chain',
    timescaledb.compress_orderby = 'timestamp DESC'
);
SELECT add_compression_policy('ml_features', INTERVAL '30 days', if_not_exists => TRUE);

-- ============================================
-- HELPER FUNCTIONS
-- ============================================

-- Function to get recent detections with pagination
CREATE OR REPLACE FUNCTION get_recent_detections(
    p_chain TEXT DEFAULT NULL,
    p_severity TEXT DEFAULT NULL,
    p_pattern TEXT DEFAULT NULL,
    p_acknowledged BOOLEAN DEFAULT NULL,
    p_limit INT DEFAULT 50,
    p_offset INT DEFAULT 0
)
RETURNS TABLE (
    detection_id TEXT,
    timestamp TIMESTAMPTZ,
    chain TEXT,
    detector_name TEXT,
    attack_pattern TEXT,
    confidence DOUBLE PRECISION,
    severity TEXT,
    description TEXT,
    acknowledged BOOLEAN
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        d.detection_id,
        d.timestamp,
        d.chain,
        d.detector_name,
        d.attack_pattern,
        d.confidence,
        d.severity,
        d.description,
        d.acknowledged
    FROM detections d
    WHERE
        (p_chain IS NULL OR d.chain = p_chain) AND
        (p_severity IS NULL OR d.severity = p_severity) AND
        (p_pattern IS NULL OR d.attack_pattern = p_pattern) AND
        (p_acknowledged IS NULL OR d.acknowledged = p_acknowledged)
    ORDER BY d.timestamp DESC
    LIMIT p_limit
    OFFSET p_offset;
END;
$$ LANGUAGE plpgsql;

-- Function to get attack statistics by time period
CREATE OR REPLACE FUNCTION get_attack_stats(
    p_start_time TIMESTAMPTZ,
    p_end_time TIMESTAMPTZ,
    p_chain TEXT DEFAULT NULL
)
RETURNS TABLE (
    attack_pattern TEXT,
    total_count BIGINT,
    critical_count BIGINT,
    high_count BIGINT,
    avg_confidence DOUBLE PRECISION
) AS $$
BEGIN
    RETURN QUERY
    SELECT
        d.attack_pattern,
        COUNT(*) as total_count,
        COUNT(*) FILTER (WHERE d.severity = 'critical') as critical_count,
        COUNT(*) FILTER (WHERE d.severity = 'high') as high_count,
        AVG(d.confidence) as avg_confidence
    FROM detections d
    WHERE
        d.timestamp >= p_start_time AND
        d.timestamp <= p_end_time AND
        (p_chain IS NULL OR d.chain = p_chain)
    GROUP BY d.attack_pattern
    ORDER BY total_count DESC;
END;
$$ LANGUAGE plpgsql;

-- ============================================
-- INITIAL REFRESH
-- ============================================
-- Refresh continuous aggregates with current data
CALL refresh_continuous_aggregate('detector_stats_hourly', NULL, NULL);
CALL refresh_continuous_aggregate('tx_volume_hourly', NULL, NULL);
CALL refresh_continuous_aggregate('bridge_volume_24h', NULL, NULL);
CALL refresh_continuous_aggregate('pool_metrics_hourly', NULL, NULL);

-- ============================================
-- SCHEMA VERSION
-- ============================================
CREATE TABLE IF NOT EXISTS schema_version (
    version TEXT PRIMARY KEY,
    applied_at TIMESTAMPTZ DEFAULT NOW(),
    description TEXT
);

INSERT INTO schema_version (version, description)
VALUES ('1.0.0', 'Initial schema with TimescaleDB hypertables, continuous aggregates, and compression policies')
ON CONFLICT (version) DO NOTHING;
