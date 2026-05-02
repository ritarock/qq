mod app;
mod infra;


use std::env;

use anyhow::{ Result, anyhow };

use crate::{app::CountExecutor, infra::CSVReader};

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();
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
