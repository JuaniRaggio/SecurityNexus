use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tokio_postgres::Row;

/// Transaction record in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub timestamp: DateTime<Utc>,
    pub tx_hash: String,
    pub block_number: i64,
    pub chain: String,
    pub pallet: String,
    pub call_name: String,
    pub caller: String,
    pub success: bool,
    pub args: Option<JsonValue>,
    pub gas_used: Option<i64>,
    pub fee_paid: Option<f64>,
}

impl Transaction {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            timestamp: row.try_get("timestamp")?,
            tx_hash: row.try_get("tx_hash")?,
            block_number: row.try_get("block_number")?,
            chain: row.try_get("chain")?,
            pallet: row.try_get("pallet")?,
            call_name: row.try_get("call_name")?,
            caller: row.try_get("caller")?,
            success: row.try_get("success")?,
            args: row.try_get("args")?,
            gas_used: row.try_get("gas_used")?,
            fee_paid: row.try_get("fee_paid")?,
        })
    }
}

/// Detection record in the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    pub timestamp: DateTime<Utc>,
    pub detection_id: String,
    pub tx_hash: String,
    pub detector_name: String,
    pub attack_pattern: String,
    pub confidence: f64,
    pub severity: String,
    pub description: Option<String>,
    pub evidence: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
    pub acknowledged: bool,
}

impl Detection {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            timestamp: row.try_get("timestamp")?,
            detection_id: row.try_get("detection_id")?,
            tx_hash: row.try_get("tx_hash")?,
            detector_name: row.try_get("detector_name")?,
            attack_pattern: row.try_get("attack_pattern")?,
            confidence: row.try_get("confidence")?,
            severity: row.try_get("severity")?,
            description: row.try_get("description")?,
            evidence: row.try_get("evidence")?,
            metadata: row.try_get("metadata")?,
            acknowledged: row.try_get("acknowledged")?,
        })
    }
}

/// ML features extracted from a transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MlFeatures {
    pub timestamp: DateTime<Utc>,
    pub tx_hash: String,
    pub price_impact_percent: Option<f64>,
    pub slippage_percent: Option<f64>,
    pub has_borrow: bool,
    pub has_repay: bool,
    pub is_cross_chain: bool,
    pub source_chain: Option<String>,
    pub dest_chain: Option<String>,
    pub is_attack: Option<bool>,
    pub attack_type: Option<String>,
}

impl MlFeatures {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            timestamp: row.try_get("timestamp")?,
            tx_hash: row.try_get("tx_hash")?,
            price_impact_percent: row.try_get("price_impact_percent")?,
            slippage_percent: row.try_get("slippage_percent")?,
            has_borrow: row.try_get("has_borrow")?,
            has_repay: row.try_get("has_repay")?,
            is_cross_chain: row.try_get("is_cross_chain")?,
            source_chain: row.try_get("source_chain")?,
            dest_chain: row.try_get("dest_chain")?,
            is_attack: row.try_get("is_attack")?,
            attack_type: row.try_get("attack_type")?,
        })
    }
}

/// Hyperbridge cross-chain message tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperbridgeMessage {
    pub timestamp: DateTime<Utc>,
    pub request_commitment: String,
    pub source_chain: String,
    pub dest_chain: String,
    pub request_type: String, // "post" or "get"
    pub proof_verified: bool,
    pub relayer_address: Option<String>,
    pub status: String, // "pending", "completed", "failed", "timeout"
    pub timeout_timestamp: Option<DateTime<Utc>>,
    pub request_data: Option<JsonValue>,
    pub response_data: Option<JsonValue>,
}

impl HyperbridgeMessage {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            timestamp: row.try_get("timestamp")?,
            request_commitment: row.try_get("request_commitment")?,
            source_chain: row.try_get("source_chain")?,
            dest_chain: row.try_get("dest_chain")?,
            request_type: row.try_get("request_type")?,
            proof_verified: row.try_get("proof_verified")?,
            relayer_address: row.try_get("relayer_address")?,
            status: row.try_get("status")?,
            timeout_timestamp: row.try_get("timeout_timestamp")?,
            request_data: row.try_get("request_data")?,
            response_data: row.try_get("response_data")?,
        })
    }
}

/// Hydration pool state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrationPoolState {
    pub timestamp: DateTime<Utc>,
    pub pool_id: String,
    pub pool_type: String, // "omnipool", "stablepool", "xyk"
    pub total_liquidity: Option<f64>,
    pub oracle_price: Option<f64>,
    pub oracle_deviation: Option<f64>,
    pub asset_reserves: Option<JsonValue>,
}

impl HydrationPoolState {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            timestamp: row.try_get("timestamp")?,
            pool_id: row.try_get("pool_id")?,
            pool_type: row.try_get("pool_type")?,
            total_liquidity: row.try_get("total_liquidity")?,
            oracle_price: row.try_get("oracle_price")?,
            oracle_deviation: row.try_get("oracle_deviation")?,
            asset_reserves: row.try_get("asset_reserves")?,
        })
    }
}

/// Hydration liquidation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HydrationLiquidation {
    pub timestamp: DateTime<Utc>,
    pub liquidation_id: String,
    pub account: String,
    pub collateral_asset: String,
    pub debt_asset: String,
    pub collateral_amount: Option<f64>,
    pub debt_amount: Option<f64>,
    pub liquidator: Option<String>,
    pub liquidation_bonus: Option<f64>,
    pub health_factor_before: Option<f64>,
    pub health_factor_after: Option<f64>,
}

impl HydrationLiquidation {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            timestamp: row.try_get("timestamp")?,
            liquidation_id: row.try_get("liquidation_id")?,
            account: row.try_get("account")?,
            collateral_asset: row.try_get("collateral_asset")?,
            debt_asset: row.try_get("debt_asset")?,
            collateral_amount: row.try_get("collateral_amount")?,
            debt_amount: row.try_get("debt_amount")?,
            liquidator: row.try_get("liquidator")?,
            liquidation_bonus: row.try_get("liquidation_bonus")?,
            health_factor_before: row.try_get("health_factor_before")?,
            health_factor_after: row.try_get("health_factor_after")?,
        })
    }
}

/// Detector statistics from continuous aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorStats {
    pub detector_name: String,
    pub chain: String,
    pub attack_pattern: String,
    pub total_detections: i64,
    pub avg_confidence: Option<f64>,
    pub critical_count: i64,
}

/// Transaction statistics from continuous aggregate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStats {
    pub chain: String,
    pub pallet: String,
    pub hour: DateTime<Utc>,
    pub tx_count: i64,
    pub success_rate: Option<f64>,
    pub avg_gas_used: Option<f64>,
    pub total_fees: Option<f64>,
}

/// Attack pattern for ML training
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackPattern {
    pub pattern_id: String,
    pub pattern_name: String,
    pub features: JsonValue,
    pub confidence_threshold: f64,
}

/// Blockchain event for generic event storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainEvent {
    pub timestamp: DateTime<Utc>,
    pub event_id: String,
    pub chain: String,
    pub block_number: i64,
    pub event_type: String,
    pub pallet: String,
    pub event_name: String,
    pub event_data: JsonValue,
}

impl BlockchainEvent {
    pub fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            timestamp: row.try_get("timestamp")?,
            event_id: row.try_get("event_id")?,
            chain: row.try_get("chain")?,
            block_number: row.try_get("block_number")?,
            event_type: row.try_get("event_type")?,
            pallet: row.try_get("pallet")?,
            event_name: row.try_get("event_name")?,
            event_data: row.try_get("event_data")?,
        })
    }
}

/// Request/response for exporting data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub chains: Option<Vec<String>>,
    pub detectors: Option<Vec<String>>,
    pub severity: Option<Vec<String>>,
    pub format: String, // "json", "csv", "parquet"
}

/// Analytics query result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsResult {
    pub total_transactions: i64,
    pub total_detections: i64,
    pub detection_rate: f64,
    pub top_attack_patterns: Vec<(String, i64)>,
    pub chain_breakdown: Vec<(String, i64)>,
    pub severity_breakdown: Vec<(String, i64)>,
}
