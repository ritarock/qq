pub mod parser;

use anyhow::{Result, anyhow};

use crate::{
    app::{Action, HelpOption, execute},
    cli::parser::select_parser,
    infra::CSVReader,
};

pub fn run(args: &[String]) -> Result<()> {
    let action = get_action(&args)?;
    let reader = CSVReader;
    execute(action, reader)
}

fn get_action(args: &[String]) -> Result<Action> {
    let arg = args.get(1).ok_or_else(|| anyhow!("filepath is required"))?;

    if arg.to_uppercase().as_str() == "HELP".to_string() {
        return Ok(Action::Help {
            help_option: HelpOption::AppHelp,
        });
    }

    let filepath = arg;

    let action = args.get(2).ok_or_else(|| anyhow!("action is required"))?;

    match action.to_uppercase().as_str() {
        "COUNT" => match args.get(3) {
            Some(_) => {
                return Ok(Action::Help {
                    help_option: HelpOption::CountHelp,
                });
            }
            None => {
                return Ok(Action::Count {
                    filepath: filepath.to_string(),
                });
            }
        },

        "HEADER" => match args.get(3) {
            Some(_) => {
                return Ok(Action::Help {
                    help_option: HelpOption::HeaderHelp,
                });
            }
            None => {
                return Ok(Action::Header {
                    filepath: filepath.to_string(),
                });
            }
        },

        "SELECT" => {
            let arg = args
                .get(3)
                .ok_or_else(|| anyhow!("column number is required"))?;
            if arg.to_uppercase().as_str() == "HELP".to_string() {
                return Ok(Action::Help {
                    help_option: HelpOption::SelectHelp,
                });
            }

            let column_number = select_parser(arg)?;

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
                select_column: SelectColumn {
                    column_number: vec![1]
                }
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

    #[test]
    fn test_get_action_help_app() -> Result<()> {
        let args = vec!["app".to_string(), "help".to_string()];

        let result = get_action(&args)?;

        assert_eq!(
            result,
            Action::Help {
                help_option: HelpOption::AppHelp
            }
        );
        Ok(())
    }

    #[test]
    fn test_get_action_help_count() -> Result<()> {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "count".to_string(),
            "help".to_string(),
        ];

        let result = get_action(&args)?;

        assert_eq!(
            result,
            Action::Help {
                help_option: HelpOption::CountHelp
            }
        );
        Ok(())
    }

    #[test]
    fn test_get_action_help_header() -> Result<()> {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "header".to_string(),
            "help".to_string(),
        ];

        let result = get_action(&args)?;

        assert_eq!(
            result,
            Action::Help {
                help_option: HelpOption::HeaderHelp
            }
        );
        Ok(())
    }

    #[test]
    fn test_get_action_help_select() -> Result<()> {
        let args = vec![
            "app".to_string(),
            "file.csv".to_string(),
            "select".to_string(),
            "help".to_string(),
        ];

        let result = get_action(&args)?;

        assert_eq!(
            result,
            Action::Help {
                help_option: HelpOption::SelectHelp
            }
        );
        Ok(())
    }
}
