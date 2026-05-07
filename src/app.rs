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
}

#[derive(Debug, PartialEq)]
pub struct SelectColumn {
    pub column_number: usize,
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
    }

    Ok(())
}
