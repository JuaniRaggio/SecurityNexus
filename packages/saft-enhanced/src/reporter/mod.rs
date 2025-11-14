//! Report generation for analysis results

use crate::{AnalysisResult, Severity};
use colored::Colorize;
use std::io::Write;

pub mod formats;

/// Format for the report output
#[derive(Debug, Clone, Copy)]
pub enum ReportFormat {
    /// Human-readable text format
    Text,
    /// JSON format for programmatic consumption
    Json,
    /// HTML format for web viewing
    Html,
    /// SARIF format for integration with security tools
    Sarif,
}

/// Reporter for analysis results
pub struct Reporter {
    format: ReportFormat,
}

impl Reporter {
    /// Create a new reporter with the specified format
    pub fn new(format: ReportFormat) -> Self {
        Self { format }
    }

    /// Generate a report from analysis results
    pub fn generate<W: Write>(
        &self,
        results: &[AnalysisResult],
        output: &mut W,
    ) -> std::io::Result<()> {
        match self.format {
            ReportFormat::Text => self.generate_text(results, output),
            ReportFormat::Json => self.generate_json(results, output),
            ReportFormat::Html => self.generate_html(results, output),
            ReportFormat::Sarif => self.generate_sarif(results, output),
        }
    }

    /// Generate text report
    fn generate_text<W: Write>(
        &self,
        results: &[AnalysisResult],
        output: &mut W,
    ) -> std::io::Result<()> {
        writeln!(output, "\n{}", "SAFT Enhanced - Analysis Report".bold())?;
        writeln!(output, "{}", "=".repeat(50))?;

        for result in results {
            writeln!(output, "\nFile: {}", result.file.display().to_string().cyan())?;
            writeln!(
                output,
                "Total vulnerabilities: {}",
                result.metadata.total_vulnerabilities
            )?;
            writeln!(
                output,
                "Analysis time: {}ms",
                result.metadata.duration_ms
            )?;

            if result.vulnerabilities.is_empty() {
                writeln!(output, "{}", "  No vulnerabilities found!".green())?;
                continue;
            }

            writeln!(output, "\nVulnerabilities:")?;
            for vuln in &result.vulnerabilities {
                let severity_str = self.colorize_severity(&vuln.severity);
                writeln!(output, "\n  [{}] {}", severity_str, vuln.id)?;
                writeln!(output, "  Location: {}:{}", vuln.location.file.display(), vuln.location.line)?;
                writeln!(output, "  Message: {}", vuln.message)?;
                writeln!(output, "  Category: {:?}", vuln.category)?;

                if let Some(remediation) = &vuln.remediation {
                    writeln!(output, "  Remediation: {}", remediation.yellow())?;
                }
            }
        }

        writeln!(output, "\n{}", "=".repeat(50))?;

        // Summary
        let total_vulns: usize = results.iter().map(|r| r.metadata.total_vulnerabilities).sum();
        let total_critical: usize = results.iter().map(|r| r.metadata.severity_counts.critical).sum();
        let total_high: usize = results.iter().map(|r| r.metadata.severity_counts.high).sum();
        let total_medium: usize = results.iter().map(|r| r.metadata.severity_counts.medium).sum();
        let total_low: usize = results.iter().map(|r| r.metadata.severity_counts.low).sum();

        writeln!(output, "\nSummary:")?;
        writeln!(output, "  Total files analyzed: {}", results.len())?;
        writeln!(output, "  Total vulnerabilities: {}", total_vulns)?;
        if total_critical > 0 {
            writeln!(output, "  Critical: {}", total_critical.to_string().red().bold())?;
        }
        if total_high > 0 {
            writeln!(output, "  High: {}", total_high.to_string().red())?;
        }
        if total_medium > 0 {
            writeln!(output, "  Medium: {}", total_medium.to_string().yellow())?;
        }
        if total_low > 0 {
            writeln!(output, "  Low: {}", total_low.to_string().blue())?;
        }

        Ok(())
    }

    /// Generate JSON report
    fn generate_json<W: Write>(
        &self,
        results: &[AnalysisResult],
        output: &mut W,
    ) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(results)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        writeln!(output, "{}", json)
    }

    /// Generate HTML report
    fn generate_html<W: Write>(
        &self,
        _results: &[AnalysisResult],
        output: &mut W,
    ) -> std::io::Result<()> {
        writeln!(output, "<!-- HTML report not yet implemented -->")
    }

    /// Generate SARIF report
    fn generate_sarif<W: Write>(
        &self,
        _results: &[AnalysisResult],
        output: &mut W,
    ) -> std::io::Result<()> {
        writeln!(output, "{{\"version\": \"2.1.0\", \"$schema\": \"https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json\"}}")
    }

    /// Colorize severity level for terminal output
    fn colorize_severity(&self, severity: &Severity) -> String {
        match severity {
            Severity::Critical => severity.to_string().red().bold().to_string(),
            Severity::High => severity.to_string().red().to_string(),
            Severity::Medium => severity.to_string().yellow().to_string(),
            Severity::Low => severity.to_string().blue().to_string(),
            Severity::Info => severity.to_string().white().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnalysisMetadata, Location, SeverityCounts, Vulnerability, VulnerabilityCategory};
    use std::path::PathBuf;

    #[test]
    fn test_text_report_generation() {
        let results = vec![AnalysisResult {
            file: PathBuf::from("test.rs"),
            vulnerabilities: vec![Vulnerability {
                id: "SAFT-001".to_string(),
                severity: Severity::High,
                category: VulnerabilityCategory::IntegerOverflow,
                message: "Test vulnerability".to_string(),
                description: "Test description".to_string(),
                location: Location {
                    file: PathBuf::from("test.rs"),
                    line: 10,
                    column: 5,
                    snippet: None,
                },
                remediation: Some("Fix it".to_string()),
                references: vec![],
            }],
            metadata: AnalysisMetadata {
                total_vulnerabilities: 1,
                severity_counts: SeverityCounts {
                    critical: 0,
                    high: 1,
                    medium: 0,
                    low: 0,
                    info: 0,
                },
                duration_ms: 100,
                analyzer_version: "0.1.0".to_string(),
            },
        }];

        let reporter = Reporter::new(ReportFormat::Text);
        let mut output = Vec::new();
        reporter.generate(&results, &mut output).unwrap();

        let report = String::from_utf8(output).unwrap();
        assert!(report.contains("SAFT Enhanced"));
        assert!(report.contains("test.rs"));
    }

    #[test]
    fn test_json_report_generation() {
        let results = vec![AnalysisResult {
            file: PathBuf::from("test.rs"),
            vulnerabilities: vec![],
            metadata: AnalysisMetadata {
                total_vulnerabilities: 0,
                severity_counts: SeverityCounts::default(),
                duration_ms: 50,
                analyzer_version: "0.1.0".to_string(),
            },
        }];

        let reporter = Reporter::new(ReportFormat::Json);
        let mut output = Vec::new();
        reporter.generate(&results, &mut output).unwrap();

        let report = String::from_utf8(output).unwrap();
        assert!(serde_json::from_str::<serde_json::Value>(&report).is_ok());
    }
}
