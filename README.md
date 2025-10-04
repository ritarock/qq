# qq

A command-line tool to execute SQL-like queries on CSV files

## Install

```bash
cargo build --release
```

## Usage

```bash
qq "SELECT <columns> FROM <csv_file_path> [WHERE <conditions>] [ORDER BY <columns>] [LIMIT <number>]"
```

## Example

```bash
# Display all data
qq "SELECT * FROM sample.csv"

# Display specific columns
qq "SELECT id, name FROM sample.csv"

# Filter with conditions
qq "SELECT * FROM sample.csv WHERE team_id = 1"

# Sort results
qq "SELECT * FROM sample.csv ORDER BY id DESC"

# Complex query
qq "SELECT id, name FROM sample.csv WHERE team_id >= 1 ORDER BY id DESC LIMIT 5"
```
