pub mod parser;

use anyhow::{Result, anyhow};

use crate::{
    app::{Action, execute},
    cli::parser::select_parser,
    infra::CSVReader,
};

pub fn run(args: &[String]) -> Result<()> {
    let action = get_action(&args)?;
    let reader = CSVReader;
    execute(action, reader)
}

fn get_action(args: &[String]) -> Result<Action> {
    let filepath = args.get(1).ok_or_else(|| anyhow!("filepath is required"))?;
    let action = args.get(2).ok_or_else(|| anyhow!("action is required"))?;

    match action.to_uppercase().as_str() {
        "COUNT" => Ok(Action::Count {
            filepath: filepath.to_string(),
        }),
        "HEADER" => Ok(Action::Header {
            filepath: filepath.to_string(),
        }),
        "SELECT" => {
            let column_str = args
                .get(3)
                .ok_or_else(|| anyhow!("column number is required"))?;

            let column_number = select_parser(column_str)?;

            Ok(Action::Select {
                filepath: filepath.to_string(),
                select_column: column_number,
            })
        }
        _ => Err(anyhow!("unknown action")),
    }
}

#[cfg(test)]
mod tests {
    use crate::app::SelectColumn;

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_get_action_pass_count() -> Result<()> {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "count".to_string(),
        ];

        let action = get_action(&args)?;

        assert_eq!(
            action,
            Action::Count {
                filepath: "file.csv".to_string()
            }
        );

        Ok(())
    }

    #[test]
    fn test_get_action_pass_header() -> Result<()> {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "header".to_string(),
        ];

        let action = get_action(&args)?;

        assert_eq!(
            action,
            Action::Header {
                filepath: "file.csv".to_string()
            }
        );

        Ok(())
    }

    #[test]
    fn test_get_action_pass_select() -> Result<()> {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "select".to_string(),
            "1".to_string(),
        ];

        let action = get_action(&args)?;

        assert_eq!(
            action,
            Action::Select {
                filepath: "file.csv".to_string(),
                select_column: SelectColumn { column_number: 1 }
            }
        );

        Ok(())
    }

    #[test]
    fn test_get_action_failed_no_filepath() -> Result<()> {
        let args = vec!["app".to_string()];

        let result = get_action(&args);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_get_action_failed_no_action() -> Result<()> {
        let args = vec!["app".to_string(), "file.csv".to_string()];

        let result = get_action(&args);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_get_action_failed_unknown_action() -> Result<()> {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "unknown".to_string(),
        ];

        let result = get_action(&args);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_get_action_failed_select_missing_column() -> Result<()> {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "select".to_string(),
        ];

        let result = get_action(&args);

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_get_action_failed_select_invalid_column() -> Result<()> {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "select".to_string(),
            "invalid".to_string(),
        ];

        let result = get_action(&args);

        assert!(result.is_err());
        Ok(())
    }
}
