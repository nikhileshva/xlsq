use std::process::Command;
use tempfile::NamedTempFile;
use xlsxwriter::*;

fn create_sample_excel() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let temp_file = NamedTempFile::with_suffix(".xlsx")?;
    let path = temp_file.path().to_str().unwrap();
    
    let workbook = Workbook::new(path)?;
    
    // Create first sheet with employee data
    let mut sheet1 = workbook.add_worksheet(Some("Employees"))?;
    sheet1.write_string(0, 0, "ID", None)?;
    sheet1.write_string(0, 1, "Name", None)?;
    sheet1.write_string(0, 2, "Department", None)?;
    sheet1.write_string(0, 3, "Salary", None)?;
    
    sheet1.write_number(1, 0, 1.0, None)?;
    sheet1.write_string(1, 1, "John Doe", None)?;
    sheet1.write_string(1, 2, "Engineering", None)?;
    sheet1.write_number(1, 3, 85000.0, None)?;
    
    sheet1.write_number(2, 0, 2.0, None)?;
    sheet1.write_string(2, 1, "Jane Smith", None)?;
    sheet1.write_string(2, 2, "Marketing", None)?;
    sheet1.write_number(2, 3, 75000.0, None)?;
    
    sheet1.write_number(3, 0, 3.0, None)?;
    sheet1.write_string(3, 1, "Bob Johnson", None)?;
    sheet1.write_string(3, 2, "Engineering", None)?;
    sheet1.write_number(3, 3, 90000.0, None)?;
    
    // Create second sheet with product data
    let mut sheet2 = workbook.add_worksheet(Some("Products"))?;
    sheet2.write_string(0, 0, "Product", None)?;
    sheet2.write_string(0, 1, "Category", None)?;
    sheet2.write_string(0, 2, "Price", None)?;
    sheet2.write_string(0, 3, "Stock", None)?;
    
    sheet2.write_string(1, 0, "Laptop", None)?;
    sheet2.write_string(1, 1, "Electronics", None)?;
    sheet2.write_number(1, 2, 1200.0, None)?;
    sheet2.write_number(1, 3, 50.0, None)?;
    
    sheet2.write_string(2, 0, "Mouse", None)?;
    sheet2.write_string(2, 1, "Electronics", None)?;
    sheet2.write_number(2, 2, 25.0, None)?;
    sheet2.write_number(2, 3, 200.0, None)?;
    
    workbook.close()?;
    Ok(temp_file)
}

#[test]
fn test_sheets_command() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "sheets"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Available sheets:"));
    assert!(stdout.contains("Employees"));
    assert!(stdout.contains("Products"));
}

#[test]
fn test_show_command_default() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "show"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Sheet: Employees"));
    assert!(stdout.contains("Dimensions:"));
    assert!(stdout.contains("John Doe"));
}

#[test]
fn test_show_command_with_sheet_name() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "show", "-s", "Products"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Sheet: Products"));
    assert!(stdout.contains("Laptop"));
    assert!(stdout.contains("Electronics"));
}

#[test]
fn test_show_command_with_sheet_index() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "show", "-s", "1", "-r", "2"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Sheet: Products"));
    assert!(stdout.contains("Product"));
    // Should only show 2 rows due to -r 2 limit
    let lines: Vec<&str> = stdout.lines().collect();
    let row_lines: Vec<&str> = lines.iter().filter(|line| line.starts_with("Row")).copied().collect();
    assert!(row_lines.len() <= 2);
}

#[test]
fn test_search_command_case_insensitive() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "search", "engineering"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Searching for 'engineering'"));
    assert!(stdout.contains("John Doe"));
    assert!(stdout.contains("Bob Johnson"));
    assert!(stdout.contains("Found 2 matching row(s)"));
}

#[test]
fn test_search_command_case_sensitive() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "search", "Engineering", "-c"])
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        panic!("Command failed with stderr: {}", stderr);
    }
    
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Searching for 'Engineering'"));
    assert!(stdout.contains("***Engineering***")); // Should highlight matches
    assert!(stdout.contains("Found 2 matching row(s)"));
}

#[test]
fn test_search_command_no_matches() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "search", "NonExistentValue"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("No matches found"));
}

#[test]
fn test_search_in_specific_sheet() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "search", "Laptop", "-s", "Products"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Searching for 'Laptop' in sheet: Products"));
    assert!(stdout.contains("***Laptop***"));
    assert!(stdout.contains("Found 1 matching row(s)"));
}

#[test]
fn test_invalid_sheet_name() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "show", "-s", "InvalidSheet"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("not found"));
}

#[test]
fn test_invalid_sheet_index() {
    let temp_file = create_sample_excel().expect("Failed to create test file");
    let file_path = temp_file.path().to_str().unwrap();
    
    let output = Command::new("cargo")
        .args(&["run", "--", "-f", file_path, "show", "-s", "99"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("out of range"));
}