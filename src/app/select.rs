use crate::app::SelectColumn;

use super::Reader;
use anyhow::Result;

pub struct SelectExecutor<R: Reader> {
    reader: R,
}

impl<R: Reader> SelectExecutor<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn execute(&self, filepath: &str, select_column: SelectColumn) -> Result<Vec<String>> {
        let rows = self.reader.read(filepath, false)?;
        let column = select_column.column_number - 1;

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
    fn test_select_executor_execute_pass() -> Result<()> {
        let mock_data = vec![
            vec!["id".to_string(), "name".to_string()],
            vec!["1".to_string(), "name1".to_string()],
        ];
        let reader = MockReader { data: mock_data };
        let executor = SelectExecutor::new(reader);

        let result = executor.execute("", SelectColumn { column_number: 1 })?;

        assert_eq!(result, vec!["id".to_string(), "1".to_string()]);

        Ok(())
    }

    #[test]
    fn test_select_executor_execute_pass_empty() -> Result<()> {
        let mock_data = vec![];
        let reader = MockReader { data: mock_data };
        let executor = SelectExecutor::new(reader);

        let result = executor.execute("", SelectColumn { column_number: 1 })?;

        assert_eq!(result, Vec::<String>::new());

        Ok(())
    }
}
