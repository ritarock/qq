pub mod count;
pub mod header;
pub mod select;

pub use count::CountExecutor;
pub use header::HeaderExecutor;

use anyhow::Result;

use crate::{app::select::SelectExecutor, infra::CSVReader};

#[derive(Debug, PartialEq)]
pub enum Action {
    Count {
        filepath: String,
    },
    Header {
        filepath: String,
    },
    Select {
        filepath: String,
        select_column: SelectColumn,
    },
    Help {
        help_option: HelpOption,
    },
}

#[derive(Debug, PartialEq)]
pub struct SelectColumn {
    pub column_number: Vec<usize>,
}

#[derive(Debug, PartialEq)]
pub enum HelpOption {
    AppHelp,
    CountHelp,
    HeaderHelp,
    SelectHelp,
}

pub trait Reader {
    fn read(&self, filepath: &str, disable_header: bool) -> Result<Vec<Vec<String>>>;
}

pub fn execute(action: Action, reader: CSVReader) -> Result<()> {
    match action {
        Action::Count { filepath } => {
            let executor = CountExecutor::new(reader);
            let count = executor.execute(&filepath)?;
            println!("{} records", count);
        }

        Action::Header { filepath } => {
            let executor = HeaderExecutor::new(reader);
            let header = executor.execute(&filepath)?;
            println!("{}", header);
        }

        Action::Select {
            filepath,
            select_column,
        } => {
            let executor = SelectExecutor::new(reader);
            let result = executor.execute(&filepath, select_column)?;
            for v in &result {
                println!("{}", v);
            }
        }

        Action::Help { help_option } => match help_option {
            HelpOption::AppHelp => println!("{}", app_help()),
            HelpOption::CountHelp => println!("{}", count_help()),
            HelpOption::HeaderHelp => println!("{}", header_help()),
            HelpOption::SelectHelp => println!("{}", select_help()),
        },
    }

    Ok(())
}

fn app_help() -> &'static str {
    r#"
Usage:
    qq <filepath> <command>

Command:
    count     Count rows in csv
    header    Show csv headers
    select    Select coulumns
    "#
}

fn count_help() -> &'static str {
    r#"
Usage:
    qq <filepath> count

Description:
    Count the number of rows in a csv file.
    "#
}

fn header_help() -> &'static str {
    r#"
Usage:
    qq <filepath> header

Description:
    Display the header row of a csv file.
    "#
}

fn select_help() -> &'static str {
    r#"
Usage:
    qq <filepath> select <columns>

Description:
    Select specific columns from a csv file.

Arguments:
    <columns>
        1     Single column
        1,3   Multiple columns
        2-5   Column range
    "#
}
