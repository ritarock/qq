use super::Reader;
use anyhow::{Result, anyhow};

pub struct HeaderExecutor<R: Reader> {
    reader: R,
}

impl<R: Reader> HeaderExecutor<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn execute(&self, filepath: &str) -> Result<String> {
        let rows = self.reader.read(filepath, false)?;

        let first = rows.get(0).ok_or_else(|| anyhow!("no rows"))?;

        let header = first
            .iter()
            .enumerate()
            .map(|(i, s)| format!("{}:{}", i + 1, s.trim()))
            .collect::<Vec<_>>()
            .join(", ");

        Ok(header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    struct MockReader {
        data: Vec<Vec<String>>,
    }

    impl Reader for MockReader {
        fn read(&self, _filepath: &str, _disable_header: bool) -> Result<Vec<Vec<String>>> {
            Ok(self.data.clone())
        }
    }

    #[test]
    fn test_execute_header() -> Result<()> {
        let mock_data = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["1".to_string(), "name1".to_string()],
        ];
        let reader = MockReader { data: mock_data };
        let executor = HeaderExecutor::new(reader);

        let result = executor.execute("")?;

        assert_eq!(result, "1:id, 2:name");

        Ok(())
    }

    #[test]
    fn test_execute_header_empty() -> Result<()> {
        let mock_data = vec![];
        let reader = MockReader { data: mock_data };
        let executor = HeaderExecutor::new(reader);

        let result = executor.execute("");

        assert!(result.is_err());

        Ok(())
    }
}
