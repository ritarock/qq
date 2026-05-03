mod app;
mod cli;
mod infra;

use std::env;

use anyhow::Result;

use crate::cli::run;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    run(&args)
}
