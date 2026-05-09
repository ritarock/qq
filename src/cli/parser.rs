use anyhow::{Result, anyhow};

use crate::app::SelectColumn;

pub fn select_parser(column: &str) -> Result<SelectColumn> {
    if column.contains(',') {
        let (before, after) = column.split_once(',').unwrap();
        let before = validate(before)?;
        let after = validate(after)?;
        return Ok(SelectColumn {
            column_number: vec![before, after],
        });
    }

    let column_number = validate(column)?;
    Ok(SelectColumn {
        column_number: vec![column_number],
    })
}

fn validate(column: &str) -> Result<usize> {
    let column_number = column
        .parse::<usize>()
        .map_err(|_| anyhow!(format!("invalid column: {}", column)))?;
    Ok(column_number)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_select_parser_pass() -> Result<()> {
        let column = "1".to_string();
        let result = select_parser(&column)?;

        assert_eq!(
            result,
            SelectColumn {
                column_number: vec![1]
            }
        );
        Ok(())
    }

    #[test]
    fn test_select_parser_pass_with_comma() -> Result<()> {
        let column = "1,2".to_string();
        let result = select_parser(&column)?;

        assert_eq!(
            result,
            SelectColumn {
                column_number: vec![1, 2]
            }
        );
        Ok(())
    }

    #[test]
    fn test_select_parser_failed_with_comma() -> Result<()> {
        let column = "1,q".to_string();
        let result = select_parser(&column);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_validate_pass() -> Result<()> {
        let column = "1".to_string();
        let result = validate(&column)?;

        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_validate_failed() -> Result<()> {
        let column = "q".to_string();
        let result = validate(&column);

        assert!(result.is_err());
        Ok(())
    }
}
