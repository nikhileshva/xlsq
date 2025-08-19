# xlsq

[![CI](https://github.com/nikhileshva/xlsq/workflows/CI/badge.svg)](https://github.com/nikhileshva/xlsq/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast and lightweight CLI tool for reading and searching Excel files (.xlsx) from the command line.

## Features

- **List sheets**: View all sheets in an Excel workbook
- **Display content**: Show the contents of any sheet with customizable row limits
- **Search functionality**: Search for values across sheets with case-sensitive options
- **Flexible sheet selection**: Access sheets by name or index
- **Clean output**: Formatted display with clear row/column indicators

## Installation

### 📦 Pre-built Binaries (Recommended)

Download the latest release for your platform from the [releases page](https://github.com/nikhileshva/xlsq/releases).

**Quick install script (Linux/macOS):**
```bash
curl -sSL https://raw.githubusercontent.com/nikhileshva/xlsq/main/install.sh | bash
```

### 🦀 Cargo (Rust Package Manager)

```bash
cargo install xlsq
```

### 🍺 Homebrew (macOS/Linux)

```bash
# Add the tap (once available)
brew tap nikhileshva/xlsq
brew install xlsq
```

### 🐳 Docker

```bash
# Pull and run
docker run --rm -v $(pwd):/data ghcr.io/nikhileshva/xlsq:latest -f /data/your-file.xlsx sheets

# Or build locally
docker build -t xlsq .
docker run --rm -v $(pwd):/data xlsq -f /data/your-file.xlsx sheets
```

### 🔧 From Source

```bash
git clone https://github.com/nikhileshva/xlsq.git
cd xlsq
cargo build --release
```

The binary will be available at `target/release/xlsq`.

## Usage

### Basic Usage

Display the first 10 rows of the first sheet:
```bash
xlsq -f data.xlsx
```

### Commands

#### List all sheets
```bash
xlsq -f data.xlsx sheets
```

#### Show sheet contents
```bash
# Show first 10 rows of first sheet
xlsq -f data.xlsx show

# Show first 5 rows of sheet by index
xlsq -f data.xlsx show -s 0 -r 5

# Show all rows of a named sheet
xlsq -f data.xlsx show -s "Sheet1" -r 0

# Show first 20 rows of second sheet
xlsq -f data.xlsx show -s 1 -r 20
```

#### Search within sheets
```bash
# Search for "apple" in the first sheet
xlsq -f data.xlsx search "apple"

# Case-sensitive search in a specific sheet
xlsq -f data.xlsx search "Apple" -s "Products" -c true

# Search in sheet by index
xlsq -f data.xlsx search "data" -s 2
```

### Options

| Flag | Long Form | Description |
|------|-----------|-------------|
| `-f` | `--file` | Path to the Excel file (required) |
| `-s` | `--sheet` | Sheet name or index (0-based, default: 0) |
| `-r` | `--rows` | Number of rows to display (0 for all, default: 10) |
| `-c` | `--case-sensitive` | Enable case-sensitive search (default: false) |

## Examples

```bash
# List all available sheets
xlsq -f sales_data.xlsx sheets

# Show the first 5 rows of the "Summary" sheet
xlsq -f sales_data.xlsx show -s "Summary" -r 5

# Search for "Revenue" in all cells of the second sheet
xlsq -f sales_data.xlsx search "Revenue" -s 1

# Case-sensitive search for "Q1" in the "Quarterly" sheet
xlsq -f report.xlsx search "Q1" -s "Quarterly" -c true
```

## Output Format

### Sheet Display
```
Sheet: Products
Dimensions: 100 rows x 5 columns

Row 1: ID | Name | Category | Price | Stock
Row 2: 1 | Apple iPhone | Electronics | 999 | 50
Row 3: 2 | Samsung TV | Electronics | 1299 | 25
...
```

### Search Results
```
Searching for 'apple' in sheet: Products

Match 1 - Row 2: 
1 | ***Apple iPhone*** | Electronics | 999 | 50

Match 2 - Row 15: 
14 | ***Apple Watch*** | Accessories | 399 | 75

Found 2 matching row(s)
```

## Dependencies

- [calamine](https://crates.io/crates/calamine) - Excel file reading
- [clap](https://crates.io/crates/clap) - Command line argument parsing  
- [anyhow](https://crates.io/crates/anyhow) - Error handling

## Requirements

- Rust 2021 edition or later
- Support for .xlsx files (Excel 2007+)

## Development

### Running Tests

The project includes comprehensive unit and integration tests:

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test '*'

# Run tests with output
cargo test -- --nocapture
```

### Test Coverage

The test suite covers:

- **Unit tests**: Core functions like `format_cell`, `get_sheet_name`, and CLI argument parsing
- **Integration tests**: End-to-end command-line functionality with sample Excel files
- **Excel file operations**: Reading sheets, searching content, and error handling
- **Edge cases**: Invalid files, missing sheets, and malformed arguments

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check code without building
cargo check
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### v0.1.0
- Initial release
- Basic sheet listing and display functionality
- Search functionality with case-sensitive options
- Support for sheet access by name or index