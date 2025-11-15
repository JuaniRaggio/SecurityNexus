// API Types for SAFT Enhanced Integration
// Matches the JSON output from SAFT Enhanced binary

export type SeverityLevel = 'critical' | 'high' | 'medium' | 'low' | 'info';

export type VulnerabilityCategory =
  | 'integer_overflow'
  | 'reentrancy'
  | 'access_control'
  | 'unchecked_error'
  | 'decimal_precision'
  | 'unsafe_operation'
  | 'logic_error'
  | 'other';

export interface VulnerabilityLocation {
  file: string;
  line: number;
  column: number;
  snippet: string | null;
}

export interface Vulnerability {
  id: string;
  severity: SeverityLevel;
  category: VulnerabilityCategory;
  message: string;
  description: string;
  location: VulnerabilityLocation;
  remediation: string;
  references: string[];
}

export interface SeverityCounts {
  critical: number;
  high: number;
  medium: number;
  low: number;
  info: number;
}

export interface AnalysisMetadata {
  total_vulnerabilities: number;
  severity_counts: SeverityCounts;
  duration_ms: number;
  analyzer_version: string;
}

export interface SAFTAnalysisResult {
  file: string;
  vulnerabilities: Vulnerability[];
  metadata: AnalysisMetadata;
}

// Dashboard-specific types

export interface DashboardStats {
  totalPalletsAnalyzed: number;
  activeAlerts: number;
  securityScore: number;
  chainsMonitored: number;
}

export interface Alert {
  id: string;
  severity: SeverityLevel;
  title: string;
  description: string;
  chain: string;
  timestamp: string;
}

export interface AnalysisHistoryItem {
  id: string;
  filename: string;
  uploadedAt: string;
  result: SAFTAnalysisResult;
  status: 'success' | 'error';
}

// API Request/Response types

export interface AnalyzeRequest {
  filename: string;
  content: string; // Base64 encoded file content
}

export interface AnalyzeResponse {
  success: boolean;
  analysisId: string;
  result?: SAFTAnalysisResult;
  error?: string;
}

export interface StatsResponse {
  success: boolean;
  stats: DashboardStats;
}

export interface HistoryResponse {
  success: boolean;
  history: AnalysisHistoryItem[];
}
