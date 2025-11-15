// In-memory storage for analysis results
// For demo purposes - in production, use a database

import type { AnalysisHistoryItem, DashboardStats, SAFTAnalysisResult } from '@/types/api';

class AnalysisStorage {
  private history: AnalysisHistoryItem[] = [];

  constructor() {
    // Initialize with empty history
    this.history = [];
  }

  /**
   * Add a new analysis result to history
   */
  addAnalysis(filename: string, result: SAFTAnalysisResult): string {
    const id = `analysis-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

    const item: AnalysisHistoryItem = {
      id,
      filename,
      uploadedAt: new Date().toISOString(),
      result,
      status: 'success',
    };

    this.history.unshift(item); // Add to beginning

    // Keep only last 50 analyses
    if (this.history.length > 50) {
      this.history = this.history.slice(0, 50);
    }

    return id;
  }

  /**
   * Get all analysis history
   */
  getHistory(limit: number = 20): AnalysisHistoryItem[] {
    return this.history.slice(0, limit);
  }

  /**
   * Get analysis by ID
   */
  getAnalysisById(id: string): AnalysisHistoryItem | undefined {
    return this.history.find(item => item.id === id);
  }

  /**
   * Calculate dashboard statistics from history
   */
  getStats(): DashboardStats {
    const totalPalletsAnalyzed = this.history.length;

    // Count active alerts (critical or high severity)
    const activeAlerts = this.history.filter(item => {
      const counts = item.result.metadata.severity_counts;
      return counts.critical > 0 || counts.high > 0;
    }).length;

    // Calculate security score (0-100)
    // Formula: 100 - (weighted sum of vulnerabilities / total pallets)
    let totalWeightedVulns = 0;
    this.history.forEach(item => {
      const counts = item.result.metadata.severity_counts;
      totalWeightedVulns +=
        counts.critical * 10 +
        counts.high * 5 +
        counts.medium * 2 +
        counts.low * 1;
    });

    const avgWeightedVulns = totalPalletsAnalyzed > 0
      ? totalWeightedVulns / totalPalletsAnalyzed
      : 0;

    const securityScore = Math.max(0, Math.min(100, 100 - avgWeightedVulns));

    return {
      totalPalletsAnalyzed,
      activeAlerts,
      securityScore: Math.round(securityScore),
      chainsMonitored: 5, // Hardcoded for demo (Polkadot, Kusama, Hydration, AssetHub, Rococo)
    };
  }

  /**
   * Get recent high/critical vulnerabilities as alerts
   */
  getRecentAlerts(limit: number = 10): Array<{
    id: string;
    severity: 'critical' | 'high' | 'medium' | 'low' | 'info';
    title: string;
    description: string;
    chain: string;
    timestamp: string;
  }> {
    const alerts: Array<{
      id: string;
      severity: 'critical' | 'high' | 'medium' | 'low' | 'info';
      title: string;
      description: string;
      chain: string;
      timestamp: string;
    }> = [];

    for (const item of this.history) {
      const criticalAndHigh = item.result.vulnerabilities.filter(
        v => v.severity === 'critical' || v.severity === 'high'
      );

      for (const vuln of criticalAndHigh) {
        alerts.push({
          id: `${item.id}-${vuln.id}`,
          severity: vuln.severity,
          title: vuln.message,
          description: vuln.description,
          chain: this.extractChainName(vuln.location.file),
          timestamp: item.uploadedAt,
        });

        if (alerts.length >= limit) {
          return alerts;
        }
      }
    }

    return alerts;
  }

  /**
   * Extract chain name from file path (best effort)
   */
  private extractChainName(filepath: string): string {
    const filename = filepath.toLowerCase();

    if (filename.includes('polkadot')) return 'Polkadot';
    if (filename.includes('kusama')) return 'Kusama';
    if (filename.includes('hydration')) return 'Hydration';
    if (filename.includes('assethub')) return 'AssetHub';
    if (filename.includes('rococo')) return 'Rococo';

    return 'Unknown';
  }

  /**
   * Clear all history (useful for testing)
   */
  clear(): void {
    this.history = [];
  }
}

// Singleton instance
const storage = new AnalysisStorage();

export default storage;
