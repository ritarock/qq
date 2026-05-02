use std::fs::File;

use anyhow::Result;
use csv::ReaderBuilder;

use crate::app::Reader;

pub struct CSVReader;

impl Reader for CSVReader {
    fn read(&self, filepath: &str) -> Result<Vec<Vec<String>>> {
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
}
