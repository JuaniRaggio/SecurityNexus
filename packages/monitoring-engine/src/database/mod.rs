pub mod models;

use anyhow::Result;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::{NoTls, Row};
use tracing::{error, info};

use models::*;

/// Database client for TimescaleDB operations
pub struct DatabaseClient {
    pool: Pool,
}

impl DatabaseClient {
    /// Create a new database client with connection pooling
    pub async fn new(database_url: &str, max_connections: usize) -> Result<Self> {
        info!("Connecting to TimescaleDB at {}", database_url);

        // Parse the database URL
        let config = database_url.parse::<tokio_postgres::Config>()?;

        // Create the manager with recycling method
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let manager = Manager::from_config(config, NoTls, mgr_config);

        // Build the connection pool
        let pool = Pool::builder(manager)
            .max_size(max_connections)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create connection pool: {}", e))?;

        // Test the connection
        let client = pool.get().await?;
        client.query_one("SELECT 1", &[]).await?;
        info!("Successfully connected to TimescaleDB");

        Ok(Self { pool })
    }

    /// Insert a transaction into the database
    pub async fn insert_transaction(&self, tx: &Transaction) -> Result<()> {
        let client = self.pool.get().await?;

        let stmt = client
            .prepare(
                "INSERT INTO transactions
                (timestamp, tx_hash, block_number, chain, pallet, call_name, caller, success, args, gas_used, fee_paid)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (tx_hash) DO NOTHING",
            )
            .await?;

        client
            .execute(
                &stmt,
                &[
                    &tx.timestamp,
                    &tx.tx_hash,
                    &tx.block_number,
                    &tx.chain,
                    &tx.pallet,
                    &tx.call_name,
                    &tx.caller,
                    &tx.success,
                    &tx.args,
                    &tx.gas_used,
                    &tx.fee_paid,
                ],
            )
            .await?;

        Ok(())
    }

    /// Insert a detection into the database
    pub async fn insert_detection(&self, detection: &Detection) -> Result<()> {
        let client = self.pool.get().await?;

        let stmt = client
            .prepare(
                "INSERT INTO detections
                (timestamp, detection_id, tx_hash, detector_name, attack_pattern, confidence, severity, description, evidence, metadata, acknowledged)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (detection_id) DO NOTHING",
            )
            .await?;

        client
            .execute(
                &stmt,
                &[
                    &detection.timestamp,
                    &detection.detection_id,
                    &detection.tx_hash,
                    &detection.detector_name,
                    &detection.attack_pattern,
                    &detection.confidence,
                    &detection.severity,
                    &detection.description,
                    &detection.evidence,
                    &detection.metadata,
                    &detection.acknowledged,
                ],
            )
            .await?;

        Ok(())
    }

    /// Insert ML features for a transaction
    pub async fn insert_ml_features(&self, features: &crate::ml::features::TransactionFeatures) -> Result<()> {
        let client = self.pool.get().await?;

        // Convert features to feature vector
        let feature_vector = crate::ml::FeatureExtractor::to_vector(features);

        // Convert features to JSON
        let features_json = serde_json::to_value(features)?;

        let stmt = client
            .prepare(
                "INSERT INTO ml_features
                (timestamp, tx_hash, caller, pallet, call_name,
                 features, feature_vector)
                VALUES ($1, $2, $3, $4, $5, $6, $7)",
            )
            .await?;

        client
            .execute(
                &stmt,
                &[
                    &chrono::Utc::now(),
                    &features.tx_hash,
                    &features.caller,
                    &features.pallet,
                    &features.call,
                    &features_json,
                    &feature_vector,
                ],
            )
            .await?;

        Ok(())
    }

    /// Insert a Hyperbridge message
    pub async fn insert_hyperbridge_message(&self, msg: &HyperbridgeMessage) -> Result<()> {
        let client = self.pool.get().await?;

        let stmt = client
            .prepare(
                "INSERT INTO hyperbridge_messages
                (timestamp, request_commitment, source_chain, dest_chain, request_type,
                 proof_verified, relayer_address, status, timeout_timestamp, request_data, response_data)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (request_commitment) DO UPDATE SET
                    proof_verified = EXCLUDED.proof_verified,
                    relayer_address = EXCLUDED.relayer_address,
                    status = EXCLUDED.status,
                    response_data = EXCLUDED.response_data",
            )
            .await?;

        client
            .execute(
                &stmt,
                &[
                    &msg.timestamp,
                    &msg.request_commitment,
                    &msg.source_chain,
                    &msg.dest_chain,
                    &msg.request_type,
                    &msg.proof_verified,
                    &msg.relayer_address,
                    &msg.status,
                    &msg.timeout_timestamp,
                    &msg.request_data,
                    &msg.response_data,
                ],
            )
            .await?;

        Ok(())
    }

    /// Insert Hydration pool state snapshot
    pub async fn insert_hydration_pool_state(&self, state: &HydrationPoolState) -> Result<()> {
        let client = self.pool.get().await?;

        let stmt = client
            .prepare(
                "INSERT INTO hydration_pool_state
                (timestamp, pool_id, pool_type, total_liquidity, oracle_price,
                 oracle_deviation, asset_reserves)
                VALUES ($1, $2, $3, $4, $5, $6, $7)",
            )
            .await?;

        client
            .execute(
                &stmt,
                &[
                    &state.timestamp,
                    &state.pool_id,
                    &state.pool_type,
                    &state.total_liquidity,
                    &state.oracle_price,
                    &state.oracle_deviation,
                    &state.asset_reserves,
                ],
            )
            .await?;

        Ok(())
    }

    /// Insert Hydration liquidation event
    pub async fn insert_hydration_liquidation(&self, liq: &HydrationLiquidation) -> Result<()> {
        let client = self.pool.get().await?;

        let stmt = client
            .prepare(
                "INSERT INTO hydration_liquidations
                (timestamp, liquidation_id, account, collateral_asset, debt_asset,
                 collateral_amount, debt_amount, liquidator, liquidation_bonus,
                 health_factor_before, health_factor_after)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (liquidation_id) DO NOTHING",
            )
            .await?;

        client
            .execute(
                &stmt,
                &[
                    &liq.timestamp,
                    &liq.liquidation_id,
                    &liq.account,
                    &liq.collateral_asset,
                    &liq.debt_asset,
                    &liq.collateral_amount,
                    &liq.debt_amount,
                    &liq.liquidator,
                    &liq.liquidation_bonus,
                    &liq.health_factor_before,
                    &liq.health_factor_after,
                ],
            )
            .await?;

        Ok(())
    }

    /// Get recent detections for a specific detector
    pub async fn get_detections(
        &self,
        detector_name: Option<String>,
        limit: i64,
    ) -> Result<Vec<Detection>> {
        let client = self.pool.get().await?;

        let query = match detector_name {
            Some(_) => {
                "SELECT * FROM detections
                 WHERE detector_name = $1
                 ORDER BY timestamp DESC
                 LIMIT $2"
            }
            None => {
                "SELECT * FROM detections
                 ORDER BY timestamp DESC
                 LIMIT $1"
            }
        };

        let rows: Vec<Row> = match detector_name {
            Some(name) => client.query(query, &[&name, &limit]).await?,
            None => client.query(query, &[&limit]).await?,
        };

        let detections = rows
            .iter()
            .map(|row| Detection::from_row(row))
            .collect::<Result<Vec<_>>>()?;

        Ok(detections)
    }

    /// Get detector statistics from the continuous aggregate
    pub async fn get_detector_stats(&self, hours: i32) -> Result<Vec<DetectorStats>> {
        let client = self.pool.get().await?;

        let query = "
            SELECT
                detector_name,
                chain,
                attack_pattern,
                SUM(detection_count) as total_detections,
                AVG(avg_confidence) as avg_confidence,
                SUM(critical_count) as critical_count
            FROM detector_stats_hourly
            WHERE hour >= NOW() - INTERVAL '1 hour' * $1
            GROUP BY detector_name, chain, attack_pattern
            ORDER BY total_detections DESC
        ";

        let rows = client.query(query, &[&hours]).await?;

        let stats = rows
            .iter()
            .map(|row| DetectorStats {
                detector_name: row.get(0),
                chain: row.get(1),
                attack_pattern: row.get(2),
                total_detections: row.get(3),
                avg_confidence: row.get(4),
                critical_count: row.get(5),
            })
            .collect();

        Ok(stats)
    }

    /// Get transaction volume statistics
    pub async fn get_transaction_stats(&self, hours: i32) -> Result<Vec<TransactionStats>> {
        let client = self.pool.get().await?;

        let query = "
            SELECT
                chain,
                pallet,
                hour,
                tx_count,
                success_rate,
                avg_gas_used,
                total_fees
            FROM transaction_stats_hourly
            WHERE hour >= NOW() - INTERVAL '1 hour' * $1
            ORDER BY hour DESC
        ";

        let rows = client.query(query, &[&hours]).await?;

        let stats = rows
            .iter()
            .map(|row| TransactionStats {
                chain: row.get(0),
                pallet: row.get(1),
                hour: row.get(2),
                tx_count: row.get(3),
                success_rate: row.get(4),
                avg_gas_used: row.get(5),
                total_fees: row.get(6),
            })
            .collect();

        Ok(stats)
    }

    /// Get ML feature statistics
    pub async fn get_ml_feature_stats(&self, limit: i64) -> Result<Vec<serde_json::Value>> {
        let client = self.pool.get().await?;

        let query = "
            SELECT
                timestamp,
                tx_hash,
                caller,
                pallet,
                call_name,
                features
            FROM ml_features
            ORDER BY timestamp DESC
            LIMIT $1
        ";

        let rows = client.query(query, &[&limit]).await?;

        let features: Vec<serde_json::Value> = rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "timestamp": row.get::<_, chrono::DateTime<chrono::Utc>>(0),
                    "tx_hash": row.get::<_, String>(1),
                    "caller": row.get::<_, String>(2),
                    "pallet": row.get::<_, String>(3),
                    "call_name": row.get::<_, String>(4),
                    "features": row.get::<_, serde_json::Value>(5),
                })
            })
            .collect();

        Ok(features)
    }

    /// Get attack pattern trends over time
    pub async fn get_attack_trends(&self, hours: i32) -> Result<Vec<serde_json::Value>> {
        let client = self.pool.get().await?;

        let query = "
            SELECT
                date_trunc('hour', timestamp) as hour,
                attack_pattern,
                COUNT(*) as count,
                AVG(confidence) as avg_confidence
            FROM detections
            WHERE timestamp >= NOW() - INTERVAL '1 hour' * $1
            GROUP BY hour, attack_pattern
            ORDER BY hour DESC, count DESC
        ";

        let rows = client.query(query, &[&hours]).await?;

        let trends: Vec<serde_json::Value> = rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "hour": row.get::<_, chrono::DateTime<chrono::Utc>>(0),
                    "attack_pattern": row.get::<_, String>(1),
                    "count": row.get::<_, i64>(2),
                    "avg_confidence": row.get::<_, f64>(3),
                })
            })
            .collect();

        Ok(trends)
    }

    /// Get data for export (all detections with details)
    pub async fn get_export_data(&self, hours: Option<i32>) -> Result<Vec<serde_json::Value>> {
        let client = self.pool.get().await?;

        let query = if let Some(h) = hours {
            format!(
                "SELECT
                    d.timestamp,
                    d.detection_id,
                    d.tx_hash,
                    d.detector_name,
                    d.attack_pattern,
                    d.confidence,
                    d.severity,
                    d.description,
                    d.evidence,
                    t.caller,
                    t.pallet,
                    t.call_name,
                    t.success,
                    t.chain
                FROM detections d
                LEFT JOIN transactions t ON d.tx_hash = t.tx_hash
                WHERE d.timestamp >= NOW() - INTERVAL '1 hour' * {}
                ORDER BY d.timestamp DESC",
                h
            )
        } else {
            "SELECT
                d.timestamp,
                d.detection_id,
                d.tx_hash,
                d.detector_name,
                d.attack_pattern,
                d.confidence,
                d.severity,
                d.description,
                d.evidence,
                t.caller,
                t.pallet,
                t.call_name,
                t.success,
                t.chain
            FROM detections d
            LEFT JOIN transactions t ON d.tx_hash = t.tx_hash
            ORDER BY d.timestamp DESC
            LIMIT 1000".to_string()
        };

        let rows = client.query(&query, &[]).await?;

        let data: Vec<serde_json::Value> = rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "timestamp": row.get::<_, chrono::DateTime<chrono::Utc>>(0),
                    "detection_id": row.get::<_, String>(1),
                    "tx_hash": row.get::<_, String>(2),
                    "detector_name": row.get::<_, String>(3),
                    "attack_pattern": row.get::<_, String>(4),
                    "confidence": row.get::<_, f64>(5),
                    "severity": row.get::<_, String>(6),
                    "description": row.get::<_, Option<String>>(7),
                    "evidence": row.get::<_, Option<serde_json::Value>>(8),
                    "caller": row.get::<_, Option<String>>(9),
                    "pallet": row.get::<_, Option<String>>(10),
                    "call_name": row.get::<_, Option<String>>(11),
                    "success": row.get::<_, Option<bool>>(12),
                    "chain": row.get::<_, Option<String>>(13),
                })
            })
            .collect();

        Ok(data)
    }

    /// Health check - verify database connection
    pub async fn health_check(&self) -> Result<bool> {
        match self.pool.get().await {
            Ok(client) => match client.query_one("SELECT 1", &[]).await {
                Ok(_) => Ok(true),
                Err(e) => {
                    error!("Database health check query failed: {}", e);
                    Ok(false)
                }
            },
            Err(e) => {
                error!("Database health check connection failed: {}", e);
                Ok(false)
            }
        }
    }
}
