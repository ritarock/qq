use crate::domain::entity::{Record, Table};
use crate::domain::repository::TableRepository;
use anyhow::{Context, Result};
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::fs::File;

/// CSVファイルからデータを読み込むリポジトリ実装
pub struct CsvTableRepository;

impl CsvTableRepository {
    pub fn new() -> Self {
        Self
    }
}

impl TableRepository for CsvTableRepository {
    fn load(&self, source: &str) -> Result<Table> {
        let file = File::open(source)
            .with_context(|| format!("Failed to open file: {}", source))?;

        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        // ヘッダーを取得
        let headers = reader
            .headers()
            .context("Failed to read CSV headers")?
            .iter()
            .map(|h| h.to_string())
            .collect::<Vec<_>>();

        // レコードを読み込む
        let mut records = Vec::new();
        for result in reader.records() {
            let csv_record = result.context("Failed to read CSV record")?;

            let mut fields = HashMap::new();
            for (i, value) in csv_record.iter().enumerate() {
                if let Some(header) = headers.get(i) {
                    fields.insert(header.clone(), value.to_string());
                }
            }

            records.push(Record::new(fields));
        }

        Ok(Table::new(headers, records))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_csv_success() {
        // 一時CSVファイルを作成
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "id,name,team_id").unwrap();
        writeln!(temp_file, "1,Alice,10").unwrap();
        writeln!(temp_file, "2,Bob,20").unwrap();
        temp_file.flush().unwrap();

        let repository = CsvTableRepository::new();
        let table = repository.load(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(table.headers, vec!["id", "name", "team_id"]);
        assert_eq!(table.records.len(), 2);
        assert_eq!(table.records[0].get("id"), Some(&"1".to_string()));
        assert_eq!(table.records[0].get("name"), Some(&"Alice".to_string()));
        assert_eq!(table.records[1].get("id"), Some(&"2".to_string()));
    }

    #[test]
    fn test_load_csv_empty() {
        // ヘッダーのみのCSVファイル
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "id,name").unwrap();
        temp_file.flush().unwrap();

        let repository = CsvTableRepository::new();
        let table = repository.load(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(table.headers, vec!["id", "name"]);
        assert_eq!(table.records.len(), 0);
    }

    #[test]
    fn test_load_csv_file_not_found() {
        let repository = CsvTableRepository::new();
        let result = repository.load("/nonexistent/file.csv");

        assert!(result.is_err());
    }

    #[test]
    fn test_load_csv_with_special_characters() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "id,name,note").unwrap();
        writeln!(temp_file, "1,\"Alice, Bob\",\"note with, comma\"").unwrap();
        temp_file.flush().unwrap();

        let repository = CsvTableRepository::new();
        let table = repository.load(temp_file.path().to_str().unwrap()).unwrap();

        assert_eq!(table.records.len(), 1);
        assert_eq!(table.records[0].get("name"), Some(&"Alice, Bob".to_string()));
        assert_eq!(table.records[0].get("note"), Some(&"note with, comma".to_string()));
    }
}
