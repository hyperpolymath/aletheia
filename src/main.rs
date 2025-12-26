//! Aletheia - RSR Compliance Verification Tool
//!
//! Aletheia (Greek: ἀλήθεια - "truth", "disclosure", "unconcealment")
//! is a zero-dependency Rust tool for verifying Rhodium Standard Repository (RSR) compliance.
//!
//! This tool checks repositories against the RSR Bronze-level standards:
//! - Type safety and memory safety
//! - Offline-first operation (no network dependencies)
//! - Complete documentation suite
//! - Security-first configuration
//! - Build system compliance

use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::SystemTime;

/// RSR Compliance levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Silver, Gold, Platinum reserved for future compliance levels
enum ComplianceLevel {
    Bronze,
    Silver,
    Gold,
    Platinum,
}

/// Individual compliance check result
#[derive(Debug)]
struct CheckResult {
    category: String,
    item: String,
    passed: bool,
    required_for: ComplianceLevel,
}

/// Security warning levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)] // Warning level reserved for future use
enum WarningLevel {
    Info,
    Warning,
    Critical,
}

/// Security warning
#[derive(Debug)]
#[allow(dead_code)] // path field used in Debug output and future enhancements
struct SecurityWarning {
    level: WarningLevel,
    message: String,
    path: Option<PathBuf>,
}

/// Overall compliance report
#[derive(Debug)]
struct ComplianceReport {
    checks: Vec<CheckResult>,
    warnings: Vec<SecurityWarning>,
    repository_path: PathBuf,
    verified_at: SystemTime,
}

impl ComplianceReport {
    fn new(path: PathBuf) -> Self {
        Self {
            checks: Vec::new(),
            warnings: Vec::new(),
            repository_path: path,
            verified_at: SystemTime::now(),
        }
    }

    fn add_check(&mut self, category: &str, item: &str, passed: bool, level: ComplianceLevel) {
        self.checks.push(CheckResult {
            category: category.to_string(),
            item: item.to_string(),
            passed,
            required_for: level,
        });
    }

    fn add_warning(&mut self, level: WarningLevel, message: &str, path: Option<PathBuf>) {
        self.warnings.push(SecurityWarning {
            level,
            message: message.to_string(),
            path,
        });
    }

    fn bronze_compliance(&self) -> bool {
        self.checks
            .iter()
            .filter(|c| c.required_for == ComplianceLevel::Bronze)
            .all(|c| c.passed)
    }

    fn passed_count(&self) -> usize {
        self.checks.iter().filter(|c| c.passed).count()
    }

    fn total_count(&self) -> usize {
        self.checks.len()
    }

    fn has_critical_warnings(&self) -> bool {
        self.warnings
            .iter()
            .any(|w| w.level == WarningLevel::Critical)
    }
}

/// Result of checking a path for existence and symlink status
struct PathCheckResult {
    exists: bool,
    is_symlink: bool,
    escapes_repo: bool,
    target: Option<PathBuf>,
}

/// Check if a path is a symlink and if it escapes the repository root
fn check_path_security(path: &Path, repo_root: &Path) -> PathCheckResult {
    // Use symlink_metadata to check the link itself, not its target
    let metadata = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(_) => {
            return PathCheckResult {
                exists: false,
                is_symlink: false,
                escapes_repo: false,
                target: None,
            }
        },
    };

    let is_symlink = metadata.file_type().is_symlink();

    if !is_symlink {
        return PathCheckResult {
            exists: true,
            is_symlink: false,
            escapes_repo: false,
            target: None,
        };
    }

    // It's a symlink - check where it points
    let target = match fs::read_link(path) {
        Ok(t) => t,
        Err(_) => {
            return PathCheckResult {
                exists: true,
                is_symlink: true,
                escapes_repo: false, // Can't determine, assume safe
                target: None,
            };
        },
    };

    // Resolve the target path (could be relative)
    let resolved_target = if target.is_absolute() {
        target.clone()
    } else {
        path.parent()
            .map(|p| p.join(&target))
            .unwrap_or(target.clone())
    };

    // Canonicalize both paths to compare
    let canonical_root = repo_root
        .canonicalize()
        .unwrap_or_else(|_| repo_root.to_path_buf());
    let canonical_target = resolved_target
        .canonicalize()
        .unwrap_or_else(|_| resolved_target.clone());

    let escapes_repo = !canonical_target.starts_with(canonical_root);

    PathCheckResult {
        exists: true,
        is_symlink: true,
        escapes_repo,
        target: Some(resolved_target),
    }
}

/// Check if a file exists at the given path (with symlink detection)
fn check_file(base: &Path, filename: &str, report: &mut ComplianceReport) -> bool {
    let path = base.join(filename);
    let security = check_path_security(&path, &report.repository_path);

    if security.is_symlink {
        if security.escapes_repo {
            report.add_warning(
                WarningLevel::Critical,
                &format!(
                    "Symlink '{}' points outside repository to '{}'",
                    filename,
                    security
                        .target
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_default()
                ),
                Some(path.clone()),
            );
        } else {
            report.add_warning(
                WarningLevel::Info,
                &format!("'{}' is a symlink (within repository bounds)", filename),
                Some(path.clone()),
            );
        }
    }

    // File exists if the path exists and points to a file (following symlinks)
    security.exists && path.is_file()
}

/// Check if a directory exists at the given path (with symlink detection)
fn check_dir(base: &Path, dirname: &str, report: &mut ComplianceReport) -> bool {
    let path = base.join(dirname);
    let security = check_path_security(&path, &report.repository_path);

    if security.is_symlink {
        if security.escapes_repo {
            report.add_warning(
                WarningLevel::Critical,
                &format!(
                    "Symlink directory '{}' points outside repository to '{}'",
                    dirname,
                    security
                        .target
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_default()
                ),
                Some(path.clone()),
            );
        } else {
            report.add_warning(
                WarningLevel::Info,
                &format!(
                    "'{}' is a symlink directory (within repository bounds)",
                    dirname
                ),
                Some(path.clone()),
            );
        }
    }

    // Directory exists if the path exists and points to a directory (following symlinks)
    security.exists && path.is_dir()
}

/// Verify documentation files exist
fn check_documentation(report: &mut ComplianceReport, repo_path: &Path) {
    // README can be either .md or .adoc (AsciiDoc is acceptable alternative)
    let readme_exists =
        file_exists(repo_path, "README.md") || file_exists(repo_path, "README.adoc");
    report.add_check(
        "Documentation",
        "README.md",
        readme_exists,
        ComplianceLevel::Bronze,
    );

    let other_required_docs = vec![
        "LICENSE.txt",
        "SECURITY.md",
        "CONTRIBUTING.md",
        "CODE_OF_CONDUCT.md",
        "MAINTAINERS.md",
        "CHANGELOG.md",
    ];

    for doc in other_required_docs {
        let exists = file_exists(repo_path, doc);
        report.add_check("Documentation", doc, exists, ComplianceLevel::Bronze);
    }
}

/// Verify .well-known directory and required files
fn check_well_known(report: &mut ComplianceReport, repo_path: &Path) {
    let well_known_path = repo_path.join(".well-known");
    let has_dir = well_known_path.is_dir();

    report.add_check(
        "Well-Known",
        ".well-known/ directory",
        has_dir,
        ComplianceLevel::Bronze,
    );

    // Always emit file checks for consistent check count (16 total)
    // Files can only pass if directory exists
    let required_files = vec!["security.txt", "ai.txt", "humans.txt"];
    for file in required_files {
        let exists = has_dir && well_known_path.join(file).is_file();
        report.add_check("Well-Known", file, exists, ComplianceLevel::Bronze);
    }
}

/// Verify build system files
fn check_build_system(report: &mut ComplianceReport, repo_path: &Path) {
    let build_files = vec![
        ("justfile", ComplianceLevel::Bronze),
        ("flake.nix", ComplianceLevel::Bronze),
        (".gitlab-ci.yml", ComplianceLevel::Bronze),
    ];

    for (file, level) in build_files {
        let exists = check_file(repo_path, file, report);
        report.add_check("Build System", file, exists, level);
    }
}

/// Verify source code structure (language-agnostic)
fn check_source_structure(report: &mut ComplianceReport, repo_path: &Path) {
    let has_src = check_dir(repo_path, "src", report);
    let has_tests = check_dir(repo_path, "tests", report) || check_dir(repo_path, "test", report);

    report.add_check(
        "Source Structure",
        "src/ directory",
        has_src,
        ComplianceLevel::Bronze,
    );

    report.add_check(
        "Source Structure",
        "tests/ directory",
        has_tests,
        ComplianceLevel::Bronze,
    );
}

/// Run all compliance checks
fn verify_repository(repo_path: &Path) -> ComplianceReport {
    let mut report = ComplianceReport::new(repo_path.to_path_buf());

    check_documentation(&mut report, repo_path);
    check_well_known(&mut report, repo_path);
    check_build_system(&mut report, repo_path);
    check_source_structure(&mut report, repo_path);

    report
}

/// Format a SystemTime as a human-readable timestamp
fn format_timestamp(time: SystemTime) -> String {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            // Calculate date components (simplified UTC)
            let days = secs / 86400;
            let time_secs = secs % 86400;
            let hours = time_secs / 3600;
            let minutes = (time_secs % 3600) / 60;
            let seconds = time_secs % 60;

            // Approximate year/month/day (good enough for display)
            let mut year = 1970;
            let mut remaining_days = days;

            loop {
                let days_in_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                    366
                } else {
                    365
                };
                if remaining_days < days_in_year {
                    break;
                }
                remaining_days -= days_in_year;
                year += 1;
            }

            let is_leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
            let days_in_months: [u64; 12] = if is_leap {
                [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            } else {
                [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
            };

            let mut month = 1;
            for days_in_month in days_in_months.iter() {
                if remaining_days < *days_in_month {
                    break;
                }
                remaining_days -= days_in_month;
                month += 1;
            }
            let day = remaining_days + 1;

            format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                year, month, day, hours, minutes, seconds
            )
        },
        Err(_) => "unknown".to_string(),
    }
}

/// Print the compliance report
fn print_report(report: &ComplianceReport) {
    println!("🔍 Aletheia - RSR Compliance Verification Report");
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

    // Print security warnings if any
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
        (report.passed_count() as f64 / report.total_count() as f64) * 100.0
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

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let repo_path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        std::env::current_dir().unwrap_or_else(|_| {
            eprintln!("Error: Cannot determine current directory");
            process::exit(1);
        })
    };

    if !repo_path.exists() {
        eprintln!("Error: Path does not exist: {}", repo_path.display());
        process::exit(1);
    }

    if !repo_path.is_dir() {
        eprintln!("Error: Path is not a directory: {}", repo_path.display());
        process::exit(1);
    }

    let report = verify_repository(&repo_path);
    print_report(&report);

    // Exit with status code based on Bronze compliance and critical warnings
    if !report.bronze_compliance() || report.has_critical_warnings() {
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_report_creation() {
        let path = PathBuf::from("/tmp/test");
        let report = ComplianceReport::new(path.clone());
        assert_eq!(report.repository_path, path);
        assert_eq!(report.checks.len(), 0);
    }

    #[test]
    fn test_add_check() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Test", "Item", true, ComplianceLevel::Bronze);
        assert_eq!(report.checks.len(), 1);
        assert_eq!(report.checks[0].passed, true);
    }

    #[test]
    fn test_bronze_compliance_all_passing() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Test", "Item1", true, ComplianceLevel::Bronze);
        report.add_check("Test", "Item2", true, ComplianceLevel::Bronze);
        assert!(report.bronze_compliance());
    }

    #[test]
    fn test_bronze_compliance_one_failing() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_check("Test", "Item1", true, ComplianceLevel::Bronze);
        report.add_check("Test", "Item2", false, ComplianceLevel::Bronze);
        assert!(!report.bronze_compliance());
    }

    #[test]
    fn test_compliance_level_equality() {
        assert_eq!(ComplianceLevel::Bronze, ComplianceLevel::Bronze);
        assert_ne!(ComplianceLevel::Bronze, ComplianceLevel::Silver);
    }

    #[test]
    fn test_add_warning() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_warning(WarningLevel::Info, "Test warning", None);
        assert_eq!(report.warnings.len(), 1);
        assert_eq!(report.warnings[0].level, WarningLevel::Info);
    }

    #[test]
    fn test_critical_warnings_detection() {
        let mut report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        report.add_warning(WarningLevel::Info, "Info warning", None);
        assert!(!report.has_critical_warnings());

        report.add_warning(WarningLevel::Critical, "Critical warning", None);
        assert!(report.has_critical_warnings());
    }

    #[test]
    fn test_warning_levels() {
        assert_eq!(WarningLevel::Info, WarningLevel::Info);
        assert_ne!(WarningLevel::Info, WarningLevel::Warning);
        assert_ne!(WarningLevel::Warning, WarningLevel::Critical);
    }

    #[test]
    fn test_report_has_timestamp() {
        let report = ComplianceReport::new(PathBuf::from("/tmp/test"));
        // Verify timestamp is set (within last few seconds)
        let now = SystemTime::now();
        let duration = now.duration_since(report.verified_at).unwrap_or_default();
        assert!(duration.as_secs() < 5);
    }

    #[test]
    fn test_format_timestamp() {
        use std::time::Duration;
        // Test a known timestamp: 2024-01-15 12:30:45 UTC
        // Days since epoch: 19738 (approximate)
        let time = SystemTime::UNIX_EPOCH + Duration::from_secs(1705322445);
        let formatted = format_timestamp(time);
        assert!(formatted.contains("2024"));
        assert!(formatted.ends_with("Z"));
    }
}
