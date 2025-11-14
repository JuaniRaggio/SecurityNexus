//! Alert management system

use crate::types::{Alert, AlertSeverity};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Alert manager handles alert creation, storage, and notifications
pub struct AlertManager {
    min_severity: AlertSeverity,
    webhook_url: Option<String>,
    alert_history: Arc<RwLock<Vec<Alert>>>,
}

impl AlertManager {
    /// Create a new alert manager
    pub fn new(min_severity: AlertSeverity, webhook_url: Option<String>) -> Self {
        Self {
            min_severity,
            webhook_url,
            alert_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Trigger a new alert
    pub async fn trigger_alert(&self, alert: Alert) {
        if alert.severity < self.min_severity {
            tracing::debug!(
                "Alert below minimum severity threshold: {:?} < {:?}",
                alert.severity,
                self.min_severity
            );
            return;
        }

        tracing::warn!(
            "ALERT: [{}] {} - {}",
            alert.severity,
            alert.pattern,
            alert.description
        );

        // Store alert in history
        let mut history = self.alert_history.write().await;
        history.push(alert.clone());

        // Send webhook notification if configured
        if let Some(webhook_url) = &self.webhook_url {
            self.send_webhook(webhook_url, &alert).await;
        }
    }

    /// Get recent alerts
    pub async fn get_recent_alerts(&self, limit: usize) -> Vec<Alert> {
        let history = self.alert_history.read().await;
        history.iter().rev().take(limit).cloned().collect()
    }

    /// Get alert count by severity
    pub async fn get_alert_counts(&self) -> AlertCounts {
        let history = self.alert_history.read().await;
        let mut counts = AlertCounts::default();

        for alert in history.iter() {
            match alert.severity {
                AlertSeverity::Critical => counts.critical += 1,
                AlertSeverity::High => counts.high += 1,
                AlertSeverity::Medium => counts.medium += 1,
                AlertSeverity::Low => counts.low += 1,
            }
        }

        counts
    }

    /// Send alert to webhook
    async fn send_webhook(&self, url: &str, alert: &Alert) {
        let client = reqwest::Client::new();

        match client.post(url).json(alert).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    tracing::info!("Alert webhook sent successfully");
                } else {
                    tracing::error!(
                        "Alert webhook failed with status: {}",
                        response.status()
                    );
                }
            }
            Err(e) => {
                tracing::error!("Failed to send alert webhook: {}", e);
            }
        }
    }

    /// Clear alert history
    pub async fn clear_history(&self) {
        let mut history = self.alert_history.write().await;
        history.clear();
        tracing::info!("Alert history cleared");
    }
}

/// Alert count statistics
#[derive(Debug, Default, Clone)]
pub struct AlertCounts {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
}

impl AlertCounts {
    pub fn total(&self) -> usize {
        self.critical + self.high + self.medium + self.low
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AttackPattern;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_alert_manager_creation() {
        let manager = AlertManager::new(AlertSeverity::Medium, None);
        let counts = manager.get_alert_counts().await;
        assert_eq!(counts.total(), 0);
    }

    #[tokio::test]
    async fn test_trigger_alert() {
        let manager = AlertManager::new(AlertSeverity::Low, None);

        let alert = Alert {
            id: "test-1".to_string(),
            timestamp: 1234567890,
            chain: "test-chain".to_string(),
            severity: AlertSeverity::High,
            pattern: AttackPattern::FlashLoan,
            description: "Test alert".to_string(),
            transaction_hash: None,
            block_number: Some(100),
            metadata: HashMap::new(),
            recommended_actions: vec![],
        };

        manager.trigger_alert(alert).await;

        let recent = manager.get_recent_alerts(10).await;
        assert_eq!(recent.len(), 1);
        assert_eq!(recent[0].id, "test-1");
    }

    #[tokio::test]
    async fn test_alert_severity_filtering() {
        let manager = AlertManager::new(AlertSeverity::High, None);

        let low_alert = Alert {
            id: "low-1".to_string(),
            timestamp: 1234567890,
            chain: "test-chain".to_string(),
            severity: AlertSeverity::Low,
            pattern: AttackPattern::VolumeAnomaly,
            description: "Low severity alert".to_string(),
            transaction_hash: None,
            block_number: Some(100),
            metadata: HashMap::new(),
            recommended_actions: vec![],
        };

        manager.trigger_alert(low_alert).await;

        let recent = manager.get_recent_alerts(10).await;
        assert_eq!(recent.len(), 0); // Low severity should be filtered out
    }

    #[tokio::test]
    async fn test_clear_history() {
        let manager = AlertManager::new(AlertSeverity::Low, None);

        let alert = Alert {
            id: "test-1".to_string(),
            timestamp: 1234567890,
            chain: "test-chain".to_string(),
            severity: AlertSeverity::Medium,
            pattern: AttackPattern::Mev,
            description: "Test alert".to_string(),
            transaction_hash: None,
            block_number: Some(100),
            metadata: HashMap::new(),
            recommended_actions: vec![],
        };

        manager.trigger_alert(alert).await;
        assert_eq!(manager.get_alert_counts().await.total(), 1);

        manager.clear_history().await;
        assert_eq!(manager.get_alert_counts().await.total(), 0);
    }
}
