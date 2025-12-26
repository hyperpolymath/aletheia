//! Rhodibot CLI - RSR Compliance Bot
//!
//! A command-line tool for verifying Rhodium Standard Repository compliance.
//! Like Dependabot but for repository standards instead of dependencies.

use rhodibot::{
    exit_codes, format_timestamp, generate_badge, generate_conformity_doc, json_escape,
    verify_repository, BotAction, BotConfig, ComplianceLevel, ComplianceReport, OutputFormat,
    Verbosity, WarningLevel, VERSION,
};
use std::path::PathBuf;
use std::process;

/// CLI options
struct CliOptions {
    repo_path: PathBuf,
    format: OutputFormat,
    verbosity: Verbosity,
    action: BotAction,
}

/// Print help message
fn print_help() {
    println!(
        r#"Rhodibot - RSR Compliance Bot

Like Dependabot but for Rhodium Standard Repository compliance.

USAGE:
    rhodibot [COMMAND] [OPTIONS] [PATH]

COMMANDS:
    check       Check RSR compliance (default)
    badge       Generate RSR badge markdown
    conformity  Generate RSR conformity document

ARGS:
    [PATH]    Repository path to verify (default: current directory)

OPTIONS:
    -f, --format <FORMAT>    Output format: human, json (default: human)
    -q, --quiet              Quiet mode: only show pass/fail result
    -v, --verbose            Verbose mode: show all details
    -h, --help               Print help information
    -V, --version            Print version information

EXIT CODES:
    0    Success - Bronze compliance achieved
    1    Failure - Bronze compliance not met
    2    Security - Critical security warnings detected
    3    Error - Invalid path provided
    4    Error - Invalid arguments

EXAMPLES:
    rhodibot                         # Check current directory
    rhodibot check /path/to/repo     # Check specific repository
    rhodibot badge                   # Generate badge for current directory
    rhodibot conformity              # Generate conformity document
    rhodibot --format json           # Output as JSON

CI/CD INTEGRATION:
    # GitHub Actions
    - uses: hyperpolymath/rhodibot@v1
      with:
        path: '.'
        fail-on-warning: true

    # GitLab CI
    rhodibot:
      image: hyperpolymath/rhodibot:latest
      script:
        - rhodibot check .
"#
    );
}

/// Print version information
fn print_version() {
    println!("rhodibot {}", VERSION);
}

/// Parse command line arguments
fn parse_args() -> Result<CliOptions, String> {
    let args: Vec<String> = std::env::args().collect();
    let mut format = OutputFormat::Human;
    let mut verbosity = Verbosity::Normal;
    let mut repo_path: Option<PathBuf> = None;
    let mut action = BotAction::Check;

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "-h" | "--help" => {
                print_help();
                process::exit(exit_codes::SUCCESS);
            }
            "-V" | "--version" => {
                print_version();
                process::exit(exit_codes::SUCCESS);
            }
            "-q" | "--quiet" => {
                verbosity = Verbosity::Quiet;
            }
            "-v" | "--verbose" => {
                verbosity = Verbosity::Verbose;
            }
            "-f" | "--format" => {
                i += 1;
                if i >= args.len() {
                    return Err("--format requires an argument".to_string());
                }
                format = match args[i].as_str() {
                    "human" => OutputFormat::Human,
                    "json" => OutputFormat::Json,
                    other => {
                        return Err(format!("Unknown format: {}. Use 'human' or 'json'", other))
                    }
                };
            }
            "check" => action = BotAction::Check,
            "badge" => action = BotAction::Badge,
            "conformity" => action = BotAction::Conformity,
            "fix" => action = BotAction::Fix,
            arg if arg.starts_with('-') => {
                if let Some(value) = arg.strip_prefix("--format=") {
                    format = match value {
                        "human" => OutputFormat::Human,
                        "json" => OutputFormat::Json,
                        other => {
                            return Err(format!("Unknown format: {}. Use 'human' or 'json'", other))
                        }
                    };
                } else {
                    return Err(format!("Unknown option: {}", arg));
                }
            }
            path => {
                if repo_path.is_some() {
                    return Err("Multiple paths provided. Only one path is allowed.".to_string());
                }
                repo_path = Some(PathBuf::from(path));
            }
        }
        i += 1;
    }

    let repo_path =
        repo_path.unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    Ok(CliOptions {
        repo_path,
        format,
        verbosity,
        action,
    })
}

/// Print the compliance report (human format)
fn print_report(report: &ComplianceReport) {
    println!("🤖 Rhodibot - RSR Compliance Report");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Repository: {}", report.repository_path.display());
    println!("Verified:   {}", format_timestamp(report.verified_at));
    println!();

    let mut current_category = String::new();
    for check in &report.checks {
        if check.category != current_category {
            println!("\n📋 {}", check.category);
            current_category = check.category.clone();
        }

        let icon = if check.passed { "✅" } else { "❌" };
        let level = format!("{:?}", check.required_for);
        println!("  {} {} [{}]", icon, check.item, level);
    }

    if !report.warnings.is_empty() {
        println!("\n🛡️  Security Warnings");
        for warning in &report.warnings {
            let icon = match warning.level {
                WarningLevel::Info => "ℹ️ ",
                WarningLevel::Warning => "⚠️ ",
                WarningLevel::Critical => "🚨",
            };
            println!("  {} {}", icon, warning.message);
        }
    }

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "Score: {}/{} checks passed ({:.1}%)",
        report.passed_count(),
        report.total_count(),
        report.percentage()
    );

    if report.has_critical_warnings() {
        println!("🚨 CRITICAL: Security warnings detected - review required");
    }

    if report.bronze_compliance() && !report.has_critical_warnings() {
        println!("🏆 Bronze-level RSR compliance: ACHIEVED");
    } else if report.bronze_compliance() && report.has_critical_warnings() {
        println!("⚠️  Bronze-level RSR compliance: ACHIEVED (with warnings)");
    } else {
        println!("⚠️  Bronze-level RSR compliance: NOT MET");
    }
    println!();
}

/// Print report as JSON
fn print_json_report(report: &ComplianceReport) {
    let timestamp = format_timestamp(report.verified_at);
    let passed = report.passed_count();
    let total = report.total_count();
    let percentage = report.percentage();
    let bronze_compliant = report.bronze_compliance();
    let has_critical = report.has_critical_warnings();

    println!("{{");
    println!("  \"tool\": \"rhodibot\",");
    println!("  \"version\": \"{}\",", VERSION);
    println!(
        "  \"repository\": \"{}\",",
        json_escape(&report.repository_path.display().to_string())
    );
    println!("  \"verified_at\": \"{}\",", timestamp);
    println!("  \"score\": {{");
    println!("    \"passed\": {},", passed);
    println!("    \"total\": {},", total);
    println!("    \"percentage\": {:.1}", percentage);
    println!("  }},");
    println!("  \"bronze_compliant\": {},", bronze_compliant);
    println!("  \"has_critical_warnings\": {},", has_critical);

    println!("  \"checks\": [");
    for (i, check) in report.checks.iter().enumerate() {
        let comma = if i < report.checks.len() - 1 { "," } else { "" };
        println!("    {{");
        println!("      \"category\": \"{}\",", json_escape(&check.category));
        println!("      \"item\": \"{}\",", json_escape(&check.item));
        println!("      \"passed\": {},", check.passed);
        println!("      \"level\": \"{:?}\"", check.required_for);
        println!("    }}{}", comma);
    }
    println!("  ],");

    println!("  \"warnings\": [");
    for (i, warning) in report.warnings.iter().enumerate() {
        let comma = if i < report.warnings.len() - 1 {
            ","
        } else {
            ""
        };
        let level = match warning.level {
            WarningLevel::Info => "info",
            WarningLevel::Warning => "warning",
            WarningLevel::Critical => "critical",
        };
        println!("    {{");
        println!("      \"level\": \"{}\",", level);
        println!("      \"message\": \"{}\"", json_escape(&warning.message));
        println!("    }}{}", comma);
    }
    println!("  ]");
    println!("}}");
}

/// Print quiet mode output
fn print_quiet_report(report: &ComplianceReport) {
    let bronze_compliant = report.bronze_compliance();
    let has_critical = report.has_critical_warnings();

    if bronze_compliant && !has_critical {
        println!("PASS");
    } else if has_critical {
        println!("FAIL (security)");
    } else {
        println!("FAIL");
    }
}

/// Print verbose report
fn print_verbose_report(report: &ComplianceReport) {
    println!("🤖 Rhodibot - RSR Compliance Report (Verbose)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Repository: {}", report.repository_path.display());
    println!("Verified:   {}", format_timestamp(report.verified_at));
    println!("Version:    {}", VERSION);
    println!();

    let mut current_category = String::new();
    for check in &report.checks {
        if check.category != current_category {
            println!("\n📋 {}", check.category);
            current_category = check.category.clone();
        }

        let icon = if check.passed { "✅" } else { "❌" };
        let level = format!("{:?}", check.required_for);
        println!("  {} {} [{}]", icon, check.item, level);
    }

    if !report.warnings.is_empty() {
        println!("\n🛡️  Security Warnings ({} total)", report.warnings.len());
        for warning in &report.warnings {
            let icon = match warning.level {
                WarningLevel::Info => "ℹ️ ",
                WarningLevel::Warning => "⚠️ ",
                WarningLevel::Critical => "🚨",
            };
            let level_str = match warning.level {
                WarningLevel::Info => "[INFO]",
                WarningLevel::Warning => "[WARN]",
                WarningLevel::Critical => "[CRITICAL]",
            };
            println!("  {} {} {}", icon, level_str, warning.message);
            if let Some(ref path) = warning.path {
                println!("      Path: {}", path.display());
            }
        }
    }

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "Score: {}/{} checks passed ({:.1}%)",
        report.passed_count(),
        report.total_count(),
        report.percentage()
    );

    if report.has_critical_warnings() {
        println!("🚨 CRITICAL: Security warnings detected - review required");
        println!(
            "   Exit code: {} (SECURITY_WARNING)",
            exit_codes::SECURITY_WARNING
        );
    }

    if report.bronze_compliance() && !report.has_critical_warnings() {
        println!("🏆 Bronze-level RSR compliance: ACHIEVED");
        println!("   Exit code: {} (SUCCESS)", exit_codes::SUCCESS);
    } else if report.bronze_compliance() && report.has_critical_warnings() {
        println!("⚠️  Bronze-level RSR compliance: ACHIEVED (with warnings)");
        println!(
            "   Exit code: {} (SECURITY_WARNING)",
            exit_codes::SECURITY_WARNING
        );
    } else {
        println!("⚠️  Bronze-level RSR compliance: NOT MET");
        println!(
            "   Exit code: {} (COMPLIANCE_FAILED)",
            exit_codes::COMPLIANCE_FAILED
        );
    }
    println!();
}

fn main() {
    let options = match parse_args() {
        Ok(opts) => opts,
        Err(e) => {
            eprintln!("Error: {}", e);
            eprintln!("Use --help for usage information.");
            process::exit(exit_codes::INVALID_ARGS);
        }
    };

    if !options.repo_path.exists() {
        eprintln!(
            "Error: Path does not exist: {}",
            options.repo_path.display()
        );
        process::exit(exit_codes::INVALID_PATH);
    }

    if !options.repo_path.is_dir() {
        eprintln!(
            "Error: Path is not a directory: {}",
            options.repo_path.display()
        );
        process::exit(exit_codes::INVALID_PATH);
    }

    let report = verify_repository(&options.repo_path);

    // Handle different actions
    match options.action {
        BotAction::Badge => {
            let level = report.highest_level().unwrap_or(ComplianceLevel::Bronze);
            println!("{}", generate_badge(level));
            process::exit(exit_codes::SUCCESS);
        }
        BotAction::Conformity => {
            println!("{}", generate_conformity_doc(&report));
            process::exit(exit_codes::SUCCESS);
        }
        BotAction::Fix => {
            eprintln!("Error: 'fix' action not yet implemented");
            eprintln!("This will automatically create missing RSR files in a future version.");
            process::exit(exit_codes::INVALID_ARGS);
        }
        BotAction::Check => {
            // Continue with normal output
        }
    }

    // Output based on format and verbosity
    match options.format {
        OutputFormat::Json => print_json_report(&report),
        OutputFormat::Human => match options.verbosity {
            Verbosity::Quiet => print_quiet_report(&report),
            Verbosity::Normal => print_report(&report),
            Verbosity::Verbose => print_verbose_report(&report),
        },
        OutputFormat::Sarif => {
            eprintln!("Error: SARIF output not yet implemented");
            process::exit(exit_codes::INVALID_ARGS);
        }
    }

    // Exit with appropriate code
    let exit_code = if report.has_critical_warnings() {
        exit_codes::SECURITY_WARNING
    } else if !report.bronze_compliance() {
        exit_codes::COMPLIANCE_FAILED
    } else {
        exit_codes::SUCCESS
    };

    process::exit(exit_code);
}
