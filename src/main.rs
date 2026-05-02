use std::{env, fs::File};
use anyhow::{ Result, anyhow };
use csv::ReaderBuilder;

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();
    let action = get_action(&args)?;

    match action {
        Action::Count { filepath } => {
            let rows = read(&filepath)?;
            println!("{} records", rows.len());
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

fn read(filepath: &str) -> Result<Vec<Vec<String>>> {
    let file = File::open(filepath)?;
    let mut rdr = ReaderBuilder::new()
        .from_reader(file);

    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result?;
        rows.push(record.iter().map(|s| s.to_string()).collect());
    }

    Ok(rows)
}
