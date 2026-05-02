use std::fs::File;

use anyhow::Result;
use csv::ReaderBuilder;

use crate::app::Reader;

pub struct CSVReader;

impl Reader for CSVReader {
    fn read(&self, filepath: &str) -> Result<Vec<Vec<String>>> {
        let file = File::open(filepath)?;
        let mut rdr = ReaderBuilder::new()
            .from_reader(file);
    
        let mut rows = Vec::new();
    
        for result in rdr.records() {
            let record = result?;
            rows.push(record.iter().map(|s| s.to_string()).collect());
        }
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_csv() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "id,name")?;
        writeln!(file, "1,name1")?;
        writeln!(file, "2,name2")?;

        let reader = CSVReader;

        let result = reader.read(file.path().to_str().unwrap())?;
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec!["1", "name1"]);
        assert_eq!(result[1], vec!["2", "name2"]);

        Ok(())
    }

    #[test]
    fn test_read_empty_csv() -> Result<()> {
        let file = NamedTempFile::new()?;

        let reader = CSVReader;

        let result = reader.read(file.path().to_str().unwrap())?;
        assert_eq!(result.len(), 0);

        Ok(())
    }
}
