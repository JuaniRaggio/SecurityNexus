//! REST API for monitoring engine
//!
//! Provides HTTP endpoints to access monitoring statistics and status

use crate::{MonitoringEngine, Result};
use actix_web::{web, App, HttpResponse, HttpServer, middleware};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiStats {
    pub is_running: bool,
    pub blocks_processed: u64,
    pub transactions_analyzed: u64,
    pub alerts_triggered: u64,
    pub chain_name: String,
    pub endpoint: String,
    pub reconnect_attempts: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
}

/// State shared across API handlers
struct ApiState {
    engine: Arc<MonitoringEngine>,
    start_time: std::time::Instant,
}

/// GET /api/stats - Get monitoring engine statistics
async fn get_stats(data: web::Data<ApiState>) -> HttpResponse {
    let stats = data.engine.get_stats().await;
    let config = &data.engine.config;

    let api_stats = ApiStats {
        is_running: stats.is_running,
        blocks_processed: stats.blocks_processed,
        transactions_analyzed: stats.transactions_analyzed,
        alerts_triggered: stats.alerts_triggered,
        chain_name: config.chain_name.clone(),
        endpoint: config.ws_endpoint.clone(),
        reconnect_attempts: data.engine.connection.get_reconnect_attempts(),
    };

    HttpResponse::Ok().json(api_stats)
}

/// GET /api/health - Health check endpoint
async fn health_check(data: web::Data<ApiState>) -> HttpResponse {
    let uptime = data.start_time.elapsed().as_secs();

    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: uptime,
    })
}

/// Configure API routes
fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/health", web::get().to(health_check))
        .route("/stats", web::get().to(get_stats));
}

/// Start the API server
pub async fn start_api_server(
    engine: Arc<MonitoringEngine>,
    bind_address: &str,
) -> Result<()> {
    tracing::info!("Starting API server on {}", bind_address);

    let api_state = web::Data::new(ApiState {
        engine,
        start_time: std::time::Instant::now(),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            // Restrict this configuration for production
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(api_state.clone())
            .service(web::scope("/api").configure(configure_routes))
    })
    .bind(bind_address)
    .map_err(|e| crate::Error::IoError(e))?
    .run()
    .await
    .map_err(|e| crate::Error::IoError(e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MonitorConfig;

    #[tokio::test]
    async fn test_api_stats_serialization() {
        let stats = ApiStats {
            is_running: true,
            blocks_processed: 100,
            transactions_analyzed: 500,
            alerts_triggered: 5,
            chain_name: "test".to_string(),
            endpoint: "ws://localhost:9944".to_string(),
            reconnect_attempts: 0,
        };

        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("\"is_running\":true"));
        assert!(json.contains("\"blocks_processed\":100"));
    }

    #[tokio::test]
    async fn test_health_response_serialization() {
        let health = HealthResponse {
            status: "healthy".to_string(),
            version: "0.1.0".to_string(),
            uptime_seconds: 3600,
        };

        let json = serde_json::to_string(&health).unwrap();
        assert!(json.contains("\"status\":\"healthy\""));
    }
}
