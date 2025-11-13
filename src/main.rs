use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use xlsq::{list_sheets, open_excel_file, search_in_sheet, show_sheet};

#[derive(Parser)]
#[command(name = "xlsq")]
#[command(about = "A CLI tool for reading and searching Excel files")]
struct Cli {
    /// Path to the Excel file
    #[arg(short, long)]
    file: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all sheets in the workbook
    Sheets,
    /// Display contents of a sheet
    Show {
        /// Sheet name or index (0-based)
        #[arg(short, long, default_value = "0")]
        sheet: String,
        /// Number of rows to display (0 for all)
        #[arg(short, long, default_value = "10")]
        rows: usize,
    },
    /// Search for a value and display matching rows
    Search {
        /// Value to search for
        value: String,
        /// Sheet name or index to search in (default: first sheet)
        #[arg(short, long, default_value = "0")]
        sheet: String,
        /// Case-sensitive search
        #[arg(short, long)]
        case_sensitive: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut workbook = open_excel_file(&cli.file)?;

    match cli.command {
        Some(Commands::Sheets) => list_sheets(&workbook),
        Some(Commands::Show { sheet, rows }) => show_sheet(&mut workbook, &sheet, rows),
        Some(Commands::Search {
            value,
            sheet,
            case_sensitive,
        }) => search_in_sheet(&mut workbook, &sheet, &value, case_sensitive),
        None => {
            // Default behavior: show first 10 rows of first sheet
            show_sheet(&mut workbook, "0", 10)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_parsing() {
        // Test basic file argument
        let args = vec!["xlsq", "-f", "test.xlsx"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_ok());

        // Test sheets command
        let args = vec!["xlsq", "-f", "test.xlsx", "sheets"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_ok());
        if let Ok(parsed) = cli {
            assert!(matches!(parsed.command, Some(Commands::Sheets)));
        }

        // Test show command with options
        let args = vec!["xlsq", "-f", "test.xlsx", "show", "-s", "1", "-r", "5"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_ok());
        if let Ok(parsed) = cli {
            if let Some(Commands::Show { sheet, rows }) = parsed.command {
                assert_eq!(sheet, "1");
                assert_eq!(rows, 5);
            } else {
                panic!("Expected Show command");
            }
        }

        // Test search command
        let args = vec!["xlsq", "-f", "test.xlsx", "search", "test", "-c"];
        let cli = Cli::try_parse_from(args);
        assert!(cli.is_ok());
        if let Ok(parsed) = cli {
            if let Some(Commands::Search {
                value,
                case_sensitive,
                ..
            }) = parsed.command
            {
                assert_eq!(value, "test");
                assert!(case_sensitive);
            } else {
                panic!("Expected Search command");
            }
        }
    }
}
