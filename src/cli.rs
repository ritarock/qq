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
    let action = args.get(1).ok_or_else(|| anyhow!("action is required"))?;

    match action.to_uppercase().as_str() {
        "COUNT" => {
            let filepath = args.get(2).ok_or_else(|| anyhow!("filepath is required"))?;
            Ok(Action::Count {
                filepath: filepath.to_string(),
            })
        }
        "HEADER" => {
            let filepath = args.get(2).ok_or_else(|| anyhow!("filepath is required"))?;
            Ok(Action::Header {
                filepath: filepath.to_string(),
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
            "count".to_string(),
            "file.csv".to_string(),
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
            "header".to_string(),
            "file.csv".to_string(),
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
    fn test_get_action_missing_action() {
        let args = vec!["app".to_string()];

        let result = get_action(&args);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_action_count_missing_filepath() {
        let args = vec!["app".to_string(), "count".to_string()];

        let result = get_action(&args);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_action_header_missing_filepath() {
        let args = vec!["app".to_string(), "header".to_string()];

        let result = get_action(&args);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_action_unknown() {
        let args = vec![
            "app".to_string(),
            "unknown".to_string(),
            "file.csv".to_string(),
        ];

        let result = get_action(&args);

        assert!(result.is_err());
    }
}
