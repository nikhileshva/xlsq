use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("A CLI tool for reading and searching Excel files"));
    assert!(stdout.contains("sheets"));
    assert!(stdout.contains("show"));
    assert!(stdout.contains("search"));
}

#[test]
fn test_cli_with_invalid_file() {
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", "nonexistent.xlsx"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("Failed to open Excel file"));
}

#[test]
fn test_cli_missing_file_argument() {
    let output = Command::new("cargo")
        .args(&["run", "--", "sheets"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("required") || stderr.contains("file"));
}

// Test that ensures the binary can be built
#[test]
fn test_build() {
    let output = Command::new("cargo")
        .args(&["build"])
        .output()
        .expect("Failed to execute cargo build");

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("Build failed: {}", stderr);
    }
}

// Test that all dependencies resolve correctly
#[test]
fn test_check() {
    let output = Command::new("cargo")
        .args(&["check"])
        .output()
        .expect("Failed to execute cargo check");

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("Check failed: {}", stderr);
    }
}