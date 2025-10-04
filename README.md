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
id  team_id  name   note
--  -------  -----  -----
1   1        name1  note1
2   1        name2  note2
3   2        name3  note3
4   3        name4  note4
5   4        name5  note5
6   1        name6  note5
7   2        name7  note6

# Display specific columns
qq "SELECT id, name FROM sample.csv"
id  name
--  -----
1   name1
2   name2
3   name3
4   name4
5   name5
6   name6
7   name7

# Filter with conditions
qq "SELECT * FROM sample.csv WHERE team_id = 1"
id  team_id  name   note
--  -------  -----  -----
1   1        name1  note1
2   1        name2  note2
6   1        name6  note5

# Sort results
qq "SELECT * FROM sample.csv ORDER BY id DESC"
id  team_id  name   note
--  -------  -----  -----
7   2        name7  note6
6   1        name6  note5
5   4        name5  note5
4   3        name4  note4
3   2        name3  note3
2   1        name2  note2
1   1        name1  note1

# Count records
qq "SELECT COUNT(*) FROM sample.csv"
COUNT(*)
--------
7

# Complex query
qq "SELECT id, name FROM sample.csv WHERE team_id >= 1 ORDER BY id DESC LIMIT 5"
id  name
--  -----
7   name7
6   name6
5   name5
4   name4
3   name3
```
