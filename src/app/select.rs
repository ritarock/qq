use super::Reader;
use anyhow::Result;

pub struct SelectExecutor<R: Reader> {
    reader: R,
}

impl<R: Reader> SelectExecutor<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn execute(&self, filepath: &str, column: usize) -> Result<Vec<String>> {
        let rows = self.reader.read(filepath, false)?;
        let column = column - 1;

        let result = rows
            .iter()
            .filter_map(|row| row.get(column))
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();

        Ok(result)
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
    fn test_execute_select() -> Result<()> {
        let mock_data = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["1".to_string(), "name1".to_string()],
        ];
        let reader = MockReader { data: mock_data };
        let executor = SelectExecutor::new(reader);

        let result = executor.execute("", 1)?;

        assert_eq!(result, vec!["id".to_string(), "1".to_string()]);

        Ok(())
    }

    #[test]
    fn test_execute_count_empty() -> Result<()> {
        let mock_data = vec![];
        let reader = MockReader { data: mock_data };
        let executor = SelectExecutor::new(reader);

        let result = executor.execute("", 1)?;

        assert_eq!(result, Vec::<String>::new());

        Ok(())
    }
}
