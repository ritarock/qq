use anyhow::{ Result, anyhow };

use crate::{app::CountExecutor, infra::CSVReader};

pub fn run(args: &[String]) -> Result<()> {
    let action = get_action(&args)?;

    let reader = CSVReader;

    match action {
        Action::Count { filepath } => {
            let executor = CountExecutor::new(reader);
            let count = executor.execute(&filepath)?;
            println!("{} records", count);
        }
    }

    Ok(())
}

#[derive(Debug)]
enum Action {
    Count { filepath: String },
}

fn get_action(args: &[String]) -> Result<Action> {
    let action = args.get(1)
        .ok_or_else(|| anyhow!("action is required"))?;

    match action.to_uppercase().as_str() {
        "COUNT" => {
            let filepath = args
                .get(2)
                .ok_or_else(|| anyhow!("filepath is required"))?;
        Ok(Action::Count { filepath: filepath.to_string() })
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

        match action {
            Action::Count { filepath } => {
                assert_eq!(filepath, "file.csv");
            }
        }

        Ok(())
    }

    #[test]
    fn test_get_action_missing_action() {
        let args = vec![
            "app".to_string(),
        ];

        let result= get_action(&args);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_action_missing_filepath() {
        let args = vec![
            "app".to_string(),
            "count".to_string(),
        ];

        let result= get_action(&args);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_action_unknown() {
        let args = vec![
            "app".to_string(),
            "unknown".to_string(),
            "file.csv".to_string(),
        ];

        let result= get_action(&args);

        assert!(result.is_err());
    }
}

