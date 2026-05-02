use anyhow::Result;

pub trait Reader {
    fn read(&self, filepath: &str, disable_header: bool) -> Result<Vec<Vec<String>>>;
}

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
    fn test_execute_counts_rows() -> Result<()> {
        let mock_data = vec![
            vec!["id".to_string(), "1".to_string()],
            vec!["name".to_string(), "name1".to_string()],
        ];
        let reader = MockReader { data: mock_data };
        let executor = CountExecutor::new(reader);

        let result = executor.execute("")?;

        assert_eq!(result, 2);

        Ok(())
    }

    #[test]
    fn test_execute_empty() -> Result<()> {
        let mock_data = vec![];
        let reader = MockReader { data: mock_data };
        let executor = CountExecutor::new(reader);

        let result = executor.execute("")?;

        assert_eq!(result, 0);

        Ok(())
    }
}
