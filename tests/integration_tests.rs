//! Integration tests for Aletheia RSR compliance verification
//!
//! These tests verify the complete end-to-end functionality of Aletheia.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Helper to create a temporary test repository
fn create_test_repo(name: &str) -> PathBuf {
    let test_dir = std::env::temp_dir().join(format!("aletheia_test_{}", name));

    // Clean up if it exists
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).ok();
    }

    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    test_dir
}

/// Helper to create a file in the test repo
fn create_file(base: &Path, path: &str, content: &str) {
    let file_path = base.join(path);

    // Create parent directories if needed
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).ok();
    }

    fs::write(file_path, content).expect("Failed to create file");
}

/// Test verification of a fully compliant repository
#[test]
fn test_fully_compliant_repository() {
    let repo = create_test_repo("compliant");

    // Create all required files
    create_file(&repo, "README.md", "# Test Project");
    create_file(&repo, "LICENSE.txt", "MIT License");
    create_file(&repo, "SECURITY.md", "# Security Policy");
    create_file(&repo, "CONTRIBUTING.md", "# Contributing");
    create_file(&repo, "CODE_OF_CONDUCT.md", "# Code of Conduct");
    create_file(&repo, "MAINTAINERS.md", "# Maintainers");
    create_file(&repo, "CHANGELOG.md", "# Changelog");

    // Create .well-known directory
    create_file(
        &repo,
        ".well-known/security.txt",
        "Contact: security@example.org",
    );
    create_file(&repo, ".well-known/ai.txt", "# AI Policy");
    create_file(&repo, ".well-known/humans.txt", "# Humans");

    // Create build system files
    create_file(&repo, "justfile", "build:\n\techo 'building'");
    create_file(&repo, "flake.nix", "{}");
    create_file(&repo, ".gitlab-ci.yml", "test:\n  script: echo 'test'");

    // Create source structure
    create_file(&repo, "src/main.rs", "fn main() {}");
    create_file(&repo, "tests/test.rs", "#[test] fn test() {}");

    // Run aletheia on the test repository
    let output = Command::new("cargo")
        .args(&["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    // Should exit with success
    assert!(
        output.status.success(),
        "Fully compliant repository should pass verification"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("16/16 checks passed"),
        "Should pass all checks"
    );
    assert!(
        stdout.contains("Bronze-level RSR compliance: ACHIEVED"),
        "Should achieve Bronze compliance"
    );

    // Clean up
    fs::remove_dir_all(repo).ok();
}

/// Test verification of a partially compliant repository
#[test]
fn test_partially_compliant_repository() {
    let repo = create_test_repo("partial");

    // Create only some required files
    create_file(&repo, "README.md", "# Test Project");
    create_file(&repo, "LICENSE.txt", "MIT License");
    create_file(&repo, "src/main.rs", "fn main() {}");

    // Run aletheia on the test repository
    let output = Command::new("cargo")
        .args(&["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    // Should exit with failure
    assert!(
        !output.status.success(),
        "Partially compliant repository should fail verification"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Bronze-level RSR compliance: NOT MET"),
        "Should not achieve Bronze compliance"
    );

    // Clean up
    fs::remove_dir_all(repo).ok();
}

/// Test verification of empty repository
#[test]
fn test_empty_repository() {
    let repo = create_test_repo("empty");

    // Run aletheia on empty repository
    let output = Command::new("cargo")
        .args(&["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    // Should exit with failure
    assert!(
        !output.status.success(),
        "Empty repository should fail verification"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Empty repo should fail Bronze compliance
    assert!(
        stdout.contains("Bronze-level RSR compliance: NOT MET"),
        "Should not meet Bronze compliance"
    );

    // Clean up
    fs::remove_dir_all(repo).ok();
}

/// Test handling of non-existent path
#[test]
fn test_nonexistent_path() {
    let output = Command::new("cargo")
        .args(&["run", "--", "/nonexistent/path/that/does/not/exist"])
        .output()
        .expect("Failed to run aletheia");

    // Should exit with error
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("does not exist") || stderr.contains("Error"),
        "Should report path error"
    );
}

/// Test self-verification (Aletheia verifying itself)
#[test]
fn test_self_verification() {
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to run aletheia self-verification");

    // Aletheia should verify itself successfully
    assert!(
        output.status.success(),
        "Aletheia should verify itself successfully"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("16/16 checks passed"),
        "Aletheia should pass all self-checks"
    );
    assert!(
        stdout.contains("Bronze-level RSR compliance: ACHIEVED"),
        "Aletheia should achieve Bronze compliance"
    );
}

/// Test output format consistency
#[test]
fn test_output_format() {
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for expected output sections
    assert!(
        stdout.contains("Aletheia - RSR Compliance Verification Report"),
        "Should have report header"
    );
    assert!(
        stdout.contains("Repository:"),
        "Should show repository path"
    );
    assert!(
        stdout.contains("Documentation"),
        "Should have Documentation section"
    );
    assert!(
        stdout.contains("Well-Known"),
        "Should have Well-Known section"
    );
    assert!(
        stdout.contains("Build System"),
        "Should have Build System section"
    );
    assert!(
        stdout.contains("Source Structure"),
        "Should have Source Structure section"
    );
    assert!(stdout.contains("Score:"), "Should show score");
    assert!(
        stdout.contains("Bronze-level RSR compliance:"),
        "Should show compliance status"
    );
}

/// Test that tests directory can be named 'test' or 'tests'
#[test]
fn test_alternate_test_directory_names() {
    // Test with 'tests' directory
    let repo1 = create_test_repo("with_tests");
    create_file(&repo1, "src/main.rs", "fn main() {}");
    create_file(&repo1, "tests/test.rs", "#[test] fn test() {}");

    let output1 = Command::new("cargo")
        .args(&["run", "--", repo1.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    let stdout1 = String::from_utf8_lossy(&output1.stdout);
    assert!(
        stdout1.contains("✅ tests/ directory"),
        "Should accept 'tests' directory"
    );

    // Test with 'test' directory
    let repo2 = create_test_repo("with_test");
    create_file(&repo2, "src/main.rs", "fn main() {}");
    create_file(&repo2, "test/test.rs", "#[test] fn test() {}");

    let output2 = Command::new("cargo")
        .args(&["run", "--", repo2.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    let stdout2 = String::from_utf8_lossy(&output2.stdout);
    assert!(
        stdout2.contains("✅ tests/ directory"),
        "Should accept 'test' directory"
    );

    // Clean up
    fs::remove_dir_all(repo1).ok();
    fs::remove_dir_all(repo2).ok();
}

/// Test JSON output format
#[test]
fn test_json_output() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--format", "json"])
        .output()
        .expect("Failed to run aletheia with JSON format");

    assert!(output.status.success(), "Should succeed with JSON format");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify JSON structure
    assert!(stdout.contains("\"version\":"), "Should have version field");
    assert!(
        stdout.contains("\"repository\":"),
        "Should have repository field"
    );
    assert!(
        stdout.contains("\"verified_at\":"),
        "Should have verified_at field"
    );
    assert!(stdout.contains("\"score\":"), "Should have score field");
    assert!(
        stdout.contains("\"bronze_compliant\":"),
        "Should have bronze_compliant field"
    );
    assert!(stdout.contains("\"checks\":"), "Should have checks array");
    assert!(
        stdout.contains("\"warnings\":"),
        "Should have warnings array"
    );
}

/// Test quiet mode output
#[test]
fn test_quiet_mode() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-q"])
        .output()
        .expect("Failed to run aletheia in quiet mode");

    assert!(output.status.success(), "Should succeed in quiet mode");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.trim(), "PASS", "Quiet mode should only output PASS");
}

/// Test verbose mode output
#[test]
fn test_verbose_mode() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-v"])
        .output()
        .expect("Failed to run aletheia in verbose mode");

    assert!(output.status.success(), "Should succeed in verbose mode");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("(Verbose)"), "Should indicate verbose mode");
    assert!(
        stdout.contains("Version:"),
        "Should show version in verbose"
    );
    assert!(
        stdout.contains("Exit code:"),
        "Should show exit code explanation"
    );
}

/// Test version flag
#[test]
fn test_version_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--version"])
        .output()
        .expect("Failed to run aletheia with --version");

    assert!(output.status.success(), "Should succeed with --version");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("aletheia"), "Should show program name");
    assert!(stdout.contains("0.1.0"), "Should show version number");
}

/// Test help flag
#[test]
fn test_help_flag() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to run aletheia with --help");

    assert!(output.status.success(), "Should succeed with --help");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("USAGE:"), "Should show usage");
    assert!(stdout.contains("OPTIONS:"), "Should show options");
    assert!(stdout.contains("EXIT CODES:"), "Should show exit codes");
    assert!(stdout.contains("EXAMPLES:"), "Should show examples");
}

/// Test exit codes for non-compliant repository
#[test]
fn test_exit_code_compliance_failed() {
    let repo = create_test_repo("exit_code_fail");

    // Create minimal non-compliant repo
    create_file(&repo, "README.md", "# Test");

    let output = Command::new("cargo")
        .args(&["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    // Exit code 1 = compliance failed
    assert_eq!(
        output.status.code(),
        Some(1),
        "Should exit with code 1 for compliance failure"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test exit code for invalid path
#[test]
fn test_exit_code_invalid_path() {
    let output = Command::new("cargo")
        .args(&["run", "--", "/nonexistent/path/12345"])
        .output()
        .expect("Failed to run aletheia");

    // Exit code 3 = invalid path
    assert_eq!(
        output.status.code(),
        Some(3),
        "Should exit with code 3 for invalid path"
    );
}

/// Test exit code for invalid arguments
#[test]
fn test_exit_code_invalid_args() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--invalid-option"])
        .output()
        .expect("Failed to run aletheia");

    // Exit code 4 = invalid arguments
    assert_eq!(
        output.status.code(),
        Some(4),
        "Should exit with code 4 for invalid arguments"
    );
}

/// Test combined short format flag
#[test]
fn test_format_equals_syntax() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--format=json"])
        .output()
        .expect("Failed to run aletheia with --format=json");

    assert!(output.status.success(), "Should succeed with --format=json");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("{"), "Should output JSON");
}

/// Test README.adoc alternative
#[test]
fn test_readme_adoc_alternative() {
    let repo = create_test_repo("readme_adoc");

    // Create with README.adoc instead of README.md
    create_file(&repo, "README.adoc", "= Test Project");
    create_file(&repo, "LICENSE.txt", "MIT");
    create_file(&repo, "src/main.rs", "fn main() {}");

    let output = Command::new("cargo")
        .args(&["run", "--", repo.to_str().unwrap()])
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("✅ README.md"),
        "Should accept README.adoc as README.md alternative"
    );

    fs::remove_dir_all(repo).ok();
}

/// Test timestamp is present in output
#[test]
fn test_timestamp_in_output() {
    let output = Command::new("cargo")
        .args(&["run"])
        .output()
        .expect("Failed to run aletheia");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("Verified:"),
        "Should show verification timestamp"
    );
    // Check ISO 8601 format (contains T and Z)
    assert!(
        stdout.contains("T") && stdout.contains("Z"),
        "Timestamp should be in ISO 8601 format"
    );
}
