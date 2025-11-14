//! SAFT Enhanced CLI
//!
//! Command-line interface for static analysis of FRAME pallets

use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use saft_enhanced::{
    reporter::{ReportFormat, Reporter},
    Analyzer, AnalyzerConfig, Severity,
};
use std::path::PathBuf;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(name = "saft")]
#[command(author = "Juan Ignacio Raggio & Victoria Park")]
#[command(version)]
#[command(about = "Static Analysis for FRAME Toolkit - Security vulnerability detector for Substrate pallets", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Minimum severity level to report
    #[arg(short = 's', long, global = true, value_enum, default_value = "info")]
    min_severity: SeverityArg,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a single file or directory
    Analyze {
        /// Path to the file or directory to analyze
        path: PathBuf,

        /// Output format
        #[arg(short = 'f', long, value_enum, default_value = "text")]
        format: FormatArg,

        /// Output file (defaults to stdout)
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,

        /// Maximum file size to analyze in bytes
        #[arg(long, default_value = "10485760")]
        max_file_size: usize,
    },

    /// Show version information
    Version,

    /// List available security rules
    Rules,
}

#[derive(Clone, ValueEnum)]
enum FormatArg {
    Text,
    Json,
    Html,
    Sarif,
}

impl From<FormatArg> for ReportFormat {
    fn from(arg: FormatArg) -> Self {
        match arg {
            FormatArg::Text => ReportFormat::Text,
            FormatArg::Json => ReportFormat::Json,
            FormatArg::Html => ReportFormat::Html,
            FormatArg::Sarif => ReportFormat::Sarif,
        }
    }
}

#[derive(Clone, ValueEnum)]
enum SeverityArg {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl From<SeverityArg> for Severity {
    fn from(arg: SeverityArg) -> Self {
        match arg {
            SeverityArg::Info => Severity::Info,
            SeverityArg::Low => Severity::Low,
            SeverityArg::Medium => Severity::Medium,
            SeverityArg::High => Severity::High,
            SeverityArg::Critical => Severity::Critical,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    match cli.command {
        Commands::Analyze {
            path,
            format,
            output,
            max_file_size,
        } => {
            let config = AnalyzerConfig {
                min_severity: cli.min_severity.into(),
                max_file_size,
                verbose: cli.verbose,
                ..Default::default()
            };

            run_analysis(path, config, format.into(), output);
        }
        Commands::Version => {
            println!("SAFT Enhanced v{}", env!("CARGO_PKG_VERSION"));
            println!("Static Analysis for FRAME Toolkit");
            println!("\nAuthors: {}", env!("CARGO_PKG_AUTHORS"));
            println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
        }
        Commands::Rules => {
            show_rules();
        }
    }
}

fn run_analysis(
    path: PathBuf,
    config: AnalyzerConfig,
    format: ReportFormat,
    output_path: Option<PathBuf>,
) {
    println!("{}", "SAFT Enhanced - Security Analysis".bold());
    println!("{}", "=".repeat(50));
    println!();

    let analyzer = Analyzer::new(config);

    let results = if path.is_file() {
        println!("Analyzing file: {}", path.display());
        match analyzer.analyze_file(&path) {
            Ok(result) => vec![result],
            Err(e) => {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    } else if path.is_dir() {
        println!("Analyzing directory: {}", path.display());
        match analyzer.analyze_directory(&path) {
            Ok(results) => results,
            Err(e) => {
                eprintln!("{} {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{} Path does not exist: {}", "Error:".red().bold(), path.display());
        std::process::exit(1);
    };

    if results.is_empty() {
        println!("{}", "No Rust files found to analyze.".yellow());
        return;
    }

    // Generate report
    let reporter = Reporter::new(format);

    if let Some(output_path) = output_path {
        // Write to file
        match std::fs::File::create(&output_path) {
            Ok(mut file) => {
                if let Err(e) = reporter.generate(&results, &mut file) {
                    eprintln!("{} Failed to write report: {}", "Error:".red().bold(), e);
                    std::process::exit(1);
                }
                println!("\n{} Report written to: {}", "Success:".green().bold(), output_path.display());
            }
            Err(e) => {
                eprintln!("{} Failed to create output file: {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    } else {
        // Write to stdout
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        if let Err(e) = reporter.generate(&results, &mut handle) {
            eprintln!("{} Failed to generate report: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }

    // Determine exit code based on findings
    let has_critical = results.iter().any(|r| r.metadata.severity_counts.critical > 0);
    let has_high = results.iter().any(|r| r.metadata.severity_counts.high > 0);

    if has_critical || has_high {
        std::process::exit(1);
    }
}

fn show_rules() {
    use saft_enhanced::rules::RuleSet;

    println!("{}", "Available Security Rules".bold());
    println!("{}", "=".repeat(50));
    println!();

    let ruleset = RuleSet::default();

    for rule in ruleset.enabled_rules() {
        let severity_color = match rule.severity {
            Severity::Critical => rule.severity.to_string().red().bold(),
            Severity::High => rule.severity.to_string().red(),
            Severity::Medium => rule.severity.to_string().yellow(),
            Severity::Low => rule.severity.to_string().blue(),
            Severity::Info => rule.severity.to_string().white(),
        };

        println!("{} - {} [{}]", rule.id.cyan().bold(), rule.name.bold(), severity_color);
        println!("  {}", rule.description);
        println!("  Category: {:?}", rule.category);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let cli = Cli::try_parse_from(["saft", "analyze", "test.rs"]);
        assert!(cli.is_ok());
    }

    #[test]
    fn test_severity_conversion() {
        let severity: Severity = SeverityArg::High.into();
        assert_eq!(severity, Severity::High);
    }
}
