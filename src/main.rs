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

use std::path::{Path, PathBuf};
use std::process;

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

/// Overall compliance report
#[derive(Debug)]
struct ComplianceReport {
    checks: Vec<CheckResult>,
    repository_path: PathBuf,
}

impl ComplianceReport {
    fn new(path: PathBuf) -> Self {
        Self {
            checks: Vec::new(),
            repository_path: path,
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
}

/// Check if a file exists at the given path
fn file_exists(base: &Path, filename: &str) -> bool {
    base.join(filename).is_file()
}

/// Check if a directory exists at the given path
fn dir_exists(base: &Path, dirname: &str) -> bool {
    base.join(dirname).is_dir()
}

/// Verify documentation files exist
fn check_documentation(report: &mut ComplianceReport, repo_path: &Path) {
    let required_docs = vec![
        "README.md",
        "LICENSE.txt",
        "SECURITY.md",
        "CONTRIBUTING.md",
        "CODE_OF_CONDUCT.md",
        "MAINTAINERS.md",
        "CHANGELOG.md",
    ];

    for doc in required_docs {
        let exists = file_exists(repo_path, doc);
        report.add_check("Documentation", doc, exists, ComplianceLevel::Bronze);
    }
}

/// Verify .well-known directory and required files
fn check_well_known(report: &mut ComplianceReport, repo_path: &Path) {
    let well_known_path = repo_path.join(".well-known");
    let dir_exists = well_known_path.is_dir();

    report.add_check(
        "Well-Known",
        ".well-known/ directory",
        dir_exists,
        ComplianceLevel::Bronze,
    );

    if dir_exists {
        let required_files = vec!["security.txt", "ai.txt", "humans.txt"];
        for file in required_files {
            let exists = well_known_path.join(file).is_file();
            report.add_check("Well-Known", file, exists, ComplianceLevel::Bronze);
        }
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
        let exists = file_exists(repo_path, file);
        report.add_check("Build System", file, exists, level);
    }
}

/// Verify source code structure (language-agnostic)
fn check_source_structure(report: &mut ComplianceReport, repo_path: &Path) {
    let has_src = dir_exists(repo_path, "src");
    let has_tests = dir_exists(repo_path, "tests") || dir_exists(repo_path, "test");

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

/// Print the compliance report
fn print_report(report: &ComplianceReport) {
    println!("🔍 Aletheia - RSR Compliance Verification Report");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Repository: {}", report.repository_path.display());
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

    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        "Score: {}/{} checks passed ({:.1}%)",
        report.passed_count(),
        report.total_count(),
        (report.passed_count() as f64 / report.total_count() as f64) * 100.0
    );

    if report.bronze_compliance() {
        println!("🏆 Bronze-level RSR compliance: ACHIEVED");
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

    // Exit with status code based on Bronze compliance
    if !report.bronze_compliance() {
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
}
