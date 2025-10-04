mod domain;
mod application;
mod infrastructure;
mod presentation;

use infrastructure::csv_repository::CsvTableRepository;
use presentation::cli::CliApp;

fn main() {
    let repository = CsvTableRepository::new();
    let app = CliApp::new(repository);

    if let Err(e) = app.run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
