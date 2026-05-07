use super::Reader;
use anyhow::Result;

pub struct CountExecutor<R: Reader> {
    reader: R,
}

impl<R: Reader> CountExecutor<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn execute(&self, filepath: &str) -> Result<usize> {
        let rows = self.reader.read(filepath, true)?;
        Ok(rows.len())
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
    fn test_count_executor_execute_pass() -> Result<()> {
        let mock_data = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["1".to_string(), "name1".to_string()],
        ];
        let reader = MockReader { data: mock_data };
        let executor = CountExecutor::new(reader);

        let result = executor.execute("")?;

        assert_eq!(result, 2);

        Ok(())
    }

    #[test]
    fn test_count_executor_execute_pass_empty() -> Result<()> {
        let mock_data = vec![];
        let reader = MockReader { data: mock_data };
        let executor = CountExecutor::new(reader);

        let result = executor.execute("")?;

        assert_eq!(result, 0);

        Ok(())
    }
}
