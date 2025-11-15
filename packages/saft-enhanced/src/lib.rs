//! SAFT Enhanced - Static Analysis for FRAME Toolkit
//!
//! A comprehensive static analysis tool for detecting security vulnerabilities
//! in Substrate FRAME pallets before deployment.
//!
//! # Features
//!
//! - Parse FRAME pallet code using syn
//! - Detect common vulnerabilities (overflow, underflow, reentrancy, etc.)
//! - Generate detailed vulnerability reports
//! - Configurable security rules
//! - CLI interface for integration into CI/CD pipelines
//!
//! # Example
//!
//! ```no_run
//! use saft_enhanced::{Analyzer, AnalyzerConfig};
//!
//! let config = AnalyzerConfig::default();
//! let analyzer = Analyzer::new(config);
//! let results = analyzer.analyze_file("path/to/pallet.rs")?;
//!
//! for vulnerability in results.vulnerabilities {
//!     println!("{}: {}", vulnerability.severity, vulnerability.message);
//! }
//! # Ok::<(), saft_enhanced::Error>(())
//! ```

pub mod analyzers;
pub mod parser;
pub mod reporter;
pub mod rules;

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Main error type for SAFT Enhanced
#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse Rust file: {0}")]
    ParseError(String),

    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid FRAME pallet structure: {0}")]
    InvalidPallet(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Result type alias for SAFT Enhanced operations
pub type Result<T> = std::result::Result<T, Error>;

/// Severity level of a vulnerability
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Informational findings
    Info,
    /// Low severity issues
    Low,
    /// Medium severity vulnerabilities
    Medium,
    /// High severity vulnerabilities
    High,
    /// Critical vulnerabilities requiring immediate attention
    Critical,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Low => write!(f, "LOW"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::High => write!(f, "HIGH"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Location of a vulnerability in source code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub snippet: Option<String>,
}

/// Category of vulnerability
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VulnerabilityCategory {
    /// Integer overflow or underflow
    IntegerOverflow,
    /// Reentrancy vulnerabilities
    Reentrancy,
    /// Access control issues
    AccessControl,
    /// Unchecked external calls
    UncheckedCall,
    /// Incorrect error handling
    ErrorHandling,
    /// Storage manipulation issues
    StorageManipulation,
    /// Timestamp dependence
    TimestampDependence,
    /// Randomness issues
    WeakRandomness,
    /// Denial of service vulnerabilities
    DenialOfService,
    /// Best practices violations
    BestPractice,
    /// XCM decimal precision issues
    XcmDecimalPrecision,
}

/// A detected vulnerability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// Unique identifier for this vulnerability type
    pub id: String,
    /// Severity level
    pub severity: Severity,
    /// Category of the vulnerability
    pub category: VulnerabilityCategory,
    /// Human-readable message
    pub message: String,
    /// Detailed description
    pub description: String,
    /// Location in source code
    pub location: Location,
    /// Suggested fix or remediation
    pub remediation: Option<String>,
    /// References to documentation or CVEs
    pub references: Vec<String>,
}

/// Analysis results for a single file or pallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    /// Path to the analyzed file
    pub file: PathBuf,
    /// List of detected vulnerabilities
    pub vulnerabilities: Vec<Vulnerability>,
    /// Analysis metadata
    pub metadata: AnalysisMetadata,
}

/// Metadata about the analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisMetadata {
    /// Total number of vulnerabilities found
    pub total_vulnerabilities: usize,
    /// Count by severity
    pub severity_counts: SeverityCounts,
    /// Analysis duration in milliseconds
    pub duration_ms: u64,
    /// SAFT Enhanced version
    pub analyzer_version: String,
}

/// Count of vulnerabilities by severity
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SeverityCounts {
    pub critical: usize,
    pub high: usize,
    pub medium: usize,
    pub low: usize,
    pub info: usize,
}

impl SeverityCounts {
    /// Increment the count for a given severity
    pub fn increment(&mut self, severity: Severity) {
        match severity {
            Severity::Critical => self.critical += 1,
            Severity::High => self.high += 1,
            Severity::Medium => self.medium += 1,
            Severity::Low => self.low += 1,
            Severity::Info => self.info += 1,
        }
    }
}

/// Configuration for the analyzer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerConfig {
    /// Minimum severity level to report
    pub min_severity: Severity,
    /// Enable/disable specific rule categories
    pub enabled_categories: Vec<VulnerabilityCategory>,
    /// Paths to exclude from analysis
    pub exclude_paths: Vec<PathBuf>,
    /// Maximum file size to analyze (in bytes)
    pub max_file_size: usize,
    /// Enable verbose output
    pub verbose: bool,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            min_severity: Severity::Info,
            enabled_categories: vec![
                VulnerabilityCategory::IntegerOverflow,
                VulnerabilityCategory::Reentrancy,
                VulnerabilityCategory::AccessControl,
                VulnerabilityCategory::UncheckedCall,
                VulnerabilityCategory::ErrorHandling,
                VulnerabilityCategory::StorageManipulation,
                VulnerabilityCategory::TimestampDependence,
                VulnerabilityCategory::WeakRandomness,
                VulnerabilityCategory::DenialOfService,
                VulnerabilityCategory::BestPractice,
            ],
            exclude_paths: vec![],
            max_file_size: 10 * 1024 * 1024, // 10 MB
            verbose: false,
        }
    }
}

/// Main analyzer for FRAME pallets
pub struct Analyzer {
    config: AnalyzerConfig,
}

impl Analyzer {
    /// Create a new analyzer with the given configuration
    pub fn new(config: AnalyzerConfig) -> Self {
        Self { config }
    }

    /// Create an analyzer with default configuration
    pub fn default() -> Self {
        Self::new(AnalyzerConfig::default())
    }

    /// Analyze a single Rust file
    pub fn analyze_file<P: AsRef<Path>>(&self, path: P) -> Result<AnalysisResult> {
        let path = path.as_ref();
        let start = std::time::Instant::now();

        tracing::info!("Analyzing file: {}", path.display());

        // Check file size
        let metadata = std::fs::metadata(path)?;
        if metadata.len() as usize > self.config.max_file_size {
            return Err(Error::ConfigError(format!(
                "File size {} exceeds maximum allowed size {}",
                metadata.len(),
                self.config.max_file_size
            )));
        }

        // Parse the file
        let ast = parser::parse_file(path)?;

        // Run analyzers
        let mut vulnerabilities = Vec::new();

        if self.config.verbose {
            tracing::debug!("Running vulnerability analyzers...");
        }

        // Integer overflow/underflow analysis
        if self.is_category_enabled(&VulnerabilityCategory::IntegerOverflow) {
            let overflow_vulns = analyzers::overflow::analyze(&ast, path)?;
            vulnerabilities.extend(overflow_vulns);
        }

        // Access control analysis
        if self.is_category_enabled(&VulnerabilityCategory::AccessControl) {
            let access_vulns = analyzers::access_control::analyze(&ast, path)?;
            vulnerabilities.extend(access_vulns);
        }

        // Reentrancy analysis
        if self.is_category_enabled(&VulnerabilityCategory::Reentrancy) {
            let reentrancy_vulns = analyzers::reentrancy::analyze(&ast, path)?;
            vulnerabilities.extend(reentrancy_vulns);
        }

        // Filter by minimum severity
        vulnerabilities.retain(|v| v.severity >= self.config.min_severity);

        // Calculate metadata
        let mut severity_counts = SeverityCounts::default();
        for vuln in &vulnerabilities {
            severity_counts.increment(vuln.severity);
        }

        let duration_ms = start.elapsed().as_millis() as u64;

        Ok(AnalysisResult {
            file: path.to_path_buf(),
            vulnerabilities,
            metadata: AnalysisMetadata {
                total_vulnerabilities: severity_counts.critical
                    + severity_counts.high
                    + severity_counts.medium
                    + severity_counts.low
                    + severity_counts.info,
                severity_counts,
                duration_ms,
                analyzer_version: env!("CARGO_PKG_VERSION").to_string(),
            },
        })
    }

    /// Analyze all Rust files in a directory recursively
    pub fn analyze_directory<P: AsRef<Path>>(&self, dir: P) -> Result<Vec<AnalysisResult>> {
        let dir = dir.as_ref();
        let mut results = Vec::new();

        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Skip excluded paths
            if self.is_excluded(path) {
                continue;
            }

            // Only analyze .rs files
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                match self.analyze_file(path) {
                    Ok(result) => results.push(result),
                    Err(e) => {
                        tracing::warn!("Failed to analyze {}: {}", path.display(), e);
                        if self.config.verbose {
                            tracing::error!("Error details: {:?}", e);
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Check if a category is enabled in the configuration
    fn is_category_enabled(&self, category: &VulnerabilityCategory) -> bool {
        self.config.enabled_categories.contains(category)
    }

    /// Check if a path should be excluded from analysis
    fn is_excluded(&self, path: &Path) -> bool {
        self.config
            .exclude_paths
            .iter()
            .any(|excluded| path.starts_with(excluded))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
        assert!(Severity::Low > Severity::Info);
    }

    #[test]
    fn test_severity_counts() {
        let mut counts = SeverityCounts::default();
        counts.increment(Severity::Critical);
        counts.increment(Severity::High);
        counts.increment(Severity::High);

        assert_eq!(counts.critical, 1);
        assert_eq!(counts.high, 2);
        assert_eq!(counts.medium, 0);
    }

    #[test]
    fn test_default_config() {
        let config = AnalyzerConfig::default();
        assert_eq!(config.min_severity, Severity::Info);
        assert_eq!(config.enabled_categories.len(), 10);
        assert!(config.max_file_size > 0);
    }
}
