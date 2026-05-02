mod app;
mod infra;
mod cli;

use std::env;

use anyhow::Result;

use crate::cli::run;

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();
    run(&args)
}

