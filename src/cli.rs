use anyhow::{Result, anyhow};

use crate::{
    app::{Action, execute},
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
            let column = column_str
                .parse::<usize>()
                .map_err(|_| anyhow!(format!("invalid column: {}", column_str)))?;
            Ok(Action::Select {
                filepath: filepath.to_string(),
                colum: column,
            })
        }
        _ => Err(anyhow!("unknown action")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_get_action_count() -> Result<()> {
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
    fn test_get_action_header() -> Result<()> {
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
    fn test_get_action_select() -> Result<()> {
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
                colum: 1
            }
        );

        Ok(())
    }

    #[test]
    fn test_get_action_missing_filepath() {
        let args = vec!["app".to_string()];

        let result = get_action(&args);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_action_missing_action() {
        let args = vec!["app".to_string(), "file.csv".to_string()];

        let result = get_action(&args);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_action_unknown() {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "unknown".to_string(),
        ];

        let result = get_action(&args);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_action_select_missing_column() {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "select".to_string(),
        ];

        let result = get_action(&args);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_action_select_invalid_column() {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "select".to_string(),
            "invalid".to_string(),
        ];

        let result = get_action(&args);

        assert!(result.is_err());
    }
}
