use anyhow::{Result, anyhow};

use crate::app::SelectColumn;

pub fn select_parser(column: &str) -> Result<SelectColumn> {
    let column_number = column
        .parse::<usize>()
        .map_err(|_| anyhow!(format!("invalid column: {}", column)))?;
    Ok(SelectColumn {
        column_number: column_number,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_select_parser_pass() -> Result<()> {
        let column = "1".to_string();
        let result = select_parser(&column)?;

        assert_eq!(result, SelectColumn { column_number: 1 });
        Ok(())
    }

    #[test]
    fn test_select_parser_failed_invalid_column() -> Result<()> {
        let column = "q".to_string();
        let result = select_parser(&column);

        assert!(result.is_err());
        Ok(())
    }
}
