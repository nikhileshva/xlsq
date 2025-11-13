use anyhow::{Context, Result};
use calamine::{open_workbook, Data, Reader, Xlsx};
use std::path::Path;

pub fn format_cell(cell: &Data) -> String {
    match cell {
        Data::Empty => String::new(),
        Data::String(s) => s.clone(),
        Data::Float(f) => {
            if f.fract() == 0.0 {
                format!("{}", *f as i64)
            } else {
                format!("{}", f)
            }
        }
        Data::Int(i) => format!("{}", i),
        Data::Bool(b) => format!("{}", b),
        Data::Error(e) => format!("ERROR: {:?}", e),
        Data::DateTime(dt) => format!("{}", dt),
        Data::DateTimeIso(dt) => dt.clone(),
        Data::DurationIso(d) => d.clone(),
    }
}

pub fn get_sheet_name(
    workbook: &Xlsx<std::io::BufReader<std::fs::File>>,
    identifier: &str,
) -> Result<String> {
    let sheet_names = workbook.sheet_names();

    // Try to parse as index first
    if let Ok(index) = identifier.parse::<usize>() {
        if index < sheet_names.len() {
            return Ok(sheet_names[index].clone());
        } else {
            return Err(anyhow::anyhow!(
                "Sheet index {} out of range. Available sheets: 0-{}",
                index,
                sheet_names.len() - 1
            ));
        }
    }

    // Try to find by name
    if sheet_names.contains(&identifier.to_string()) {
        return Ok(identifier.to_string());
    }

    Err(anyhow::anyhow!(
        "Sheet '{}' not found. Available sheets: {:?}",
        identifier,
        sheet_names
    ))
}

pub fn list_sheets(workbook: &Xlsx<std::io::BufReader<std::fs::File>>) -> Result<()> {
    println!("Available sheets:");
    for (index, sheet_name) in workbook.sheet_names().iter().enumerate() {
        println!("  {}: {}", index, sheet_name);
    }
    Ok(())
}

pub fn show_sheet(
    workbook: &mut Xlsx<std::io::BufReader<std::fs::File>>,
    sheet_identifier: &str,
    max_rows: usize,
) -> Result<()> {
    let sheet_name = get_sheet_name(workbook, sheet_identifier)?;

    if let Ok(range) = workbook.worksheet_range(&sheet_name) {
        println!("Sheet: {}", sheet_name);
        println!(
            "Dimensions: {} rows x {} columns",
            range.height(),
            range.width()
        );
        println!();

        let rows_to_show = if max_rows == 0 {
            range.height()
        } else {
            max_rows.min(range.height())
        };

        for (row_idx, row) in range.rows().enumerate().take(rows_to_show) {
            print!("Row {}: ", row_idx + 1);
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx > 0 {
                    print!(" | ");
                }
                print!("{}", format_cell(cell));
            }
            println!();
        }

        if rows_to_show < range.height() {
            println!("\n... and {} more rows", range.height() - rows_to_show);
        }
    } else {
        println!("Could not read sheet: {}", sheet_name);
    }

    Ok(())
}

pub fn search_in_sheet(
    workbook: &mut Xlsx<std::io::BufReader<std::fs::File>>,
    sheet_identifier: &str,
    search_value: &str,
    case_sensitive: bool,
) -> Result<()> {
    let sheet_name = get_sheet_name(workbook, sheet_identifier)?;

    if let Ok(range) = workbook.worksheet_range(&sheet_name) {
        println!("Searching for '{}' in sheet: {}", search_value, sheet_name);
        println!();

        let mut matches_found = 0;
        let search_term = if case_sensitive {
            search_value.to_string()
        } else {
            search_value.to_lowercase()
        };

        for (row_idx, row) in range.rows().enumerate() {
            let mut row_matches = false;
            let mut matching_columns = Vec::new();

            // Check each cell in the row for matches
            for (col_idx, cell) in row.iter().enumerate() {
                let cell_value = format_cell(cell);
                let comparison_value = if case_sensitive {
                    cell_value.clone()
                } else {
                    cell_value.to_lowercase()
                };

                if comparison_value.contains(&search_term) {
                    row_matches = true;
                    matching_columns.push(col_idx);
                }
            }

            if row_matches {
                matches_found += 1;
                println!("Match {} - Row {}: ", matches_found, row_idx + 1);

                for (col_idx, cell) in row.iter().enumerate() {
                    if col_idx > 0 {
                        print!(" | ");
                    }
                    let cell_value = format_cell(cell);

                    // Highlight matching cells
                    if matching_columns.contains(&col_idx) {
                        print!("***{}***", cell_value);
                    } else {
                        print!("{}", cell_value);
                    }
                }
                println!();
                println!();
            }
        }

        if matches_found == 0 {
            println!("No matches found for '{}'", search_value);
        } else {
            println!("Found {} matching row(s)", matches_found);
        }
    } else {
        println!("Could not read sheet: {}", sheet_name);
    }

    Ok(())
}

pub fn open_excel_file<P: AsRef<Path>>(path: P) -> Result<Xlsx<std::io::BufReader<std::fs::File>>> {
    open_workbook(&path).with_context(|| format!("Failed to open Excel file: {:?}", path.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use calamine::Data;
    use tempfile::NamedTempFile;

    #[test]
    fn test_format_cell_string() {
        let cell = Data::String("Hello World".to_string());
        assert_eq!(format_cell(&cell), "Hello World");
    }

    #[test]
    fn test_format_cell_int() {
        let cell = Data::Int(42);
        assert_eq!(format_cell(&cell), "42");
    }

    #[test]
    fn test_format_cell_float_whole_number() {
        let cell = Data::Float(42.0);
        assert_eq!(format_cell(&cell), "42");
    }

    #[test]
    fn test_format_cell_float_decimal() {
        let cell = Data::Float(42.5);
        assert_eq!(format_cell(&cell), "42.5");
    }

    #[test]
    fn test_format_cell_bool() {
        let cell = Data::Bool(true);
        assert_eq!(format_cell(&cell), "true");

        let cell = Data::Bool(false);
        assert_eq!(format_cell(&cell), "false");
    }

    #[test]
    fn test_format_cell_empty() {
        let cell = Data::Empty;
        assert_eq!(format_cell(&cell), "");
    }

    #[test]
    fn test_get_sheet_name_by_index() {
        let temp_file = create_test_excel_file().expect("Failed to create test file");
        let workbook: Xlsx<_> = open_workbook(temp_file.path()).unwrap();

        // Test valid index
        let result = get_sheet_name(&workbook, "0");
        assert!(result.is_ok());

        // Test invalid index
        let result = get_sheet_name(&workbook, "99");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of range"));
    }

    #[test]
    fn test_get_sheet_name_by_name() {
        let temp_file = create_test_excel_file().expect("Failed to create test file");
        let workbook: Xlsx<_> = open_workbook(temp_file.path()).unwrap();

        let result = get_sheet_name(&workbook, "Sheet1");
        assert!(result.is_ok());

        let result = get_sheet_name(&workbook, "NonExistentSheet");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_workbook_reading() {
        let temp_file = create_test_excel_file().expect("Failed to create test file");
        let mut workbook: Xlsx<_> = open_workbook(temp_file.path()).unwrap();

        let sheet_names = workbook.sheet_names();
        assert!(!sheet_names.is_empty());
        assert_eq!(sheet_names[0], "Sheet1");

        let range = workbook.worksheet_range("Sheet1").unwrap();
        assert!(range.height() > 0);
        assert!(range.width() > 0);

        let cell = range.get((0, 0)).unwrap();
        assert_eq!(format_cell(cell), "Name");

        let cell = range.get((1, 0)).unwrap();
        assert_eq!(format_cell(cell), "Alice");

        let cell = range.get((1, 1)).unwrap();
        assert_eq!(format_cell(cell), "30");
    }

    fn create_test_excel_file() -> Result<NamedTempFile> {
        use xlsxwriter::*;

        let temp_file = NamedTempFile::with_suffix(".xlsx")?;
        let path = temp_file.path().to_str().unwrap();

        let workbook = Workbook::new(path)?;
        let mut worksheet = workbook.add_worksheet(None)?;

        worksheet.write_string(0, 0, "Name", None)?;
        worksheet.write_string(0, 1, "Age", None)?;
        worksheet.write_string(0, 2, "City", None)?;

        worksheet.write_string(1, 0, "Alice", None)?;
        worksheet.write_number(1, 1, 30.0, None)?;
        worksheet.write_string(1, 2, "New York", None)?;

        worksheet.write_string(2, 0, "Bob", None)?;
        worksheet.write_number(2, 1, 25.0, None)?;
        worksheet.write_string(2, 2, "Los Angeles", None)?;

        worksheet.write_string(3, 0, "Charlie", None)?;
        worksheet.write_number(3, 1, 35.0, None)?;
        worksheet.write_string(3, 2, "Chicago", None)?;

        workbook.close()?;

        Ok(temp_file)
    }
}
