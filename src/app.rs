use anyhow::Result;

pub trait Reader {
    fn read(&self, filepath: &str) -> Result<Vec<Vec<String>>>;
}

pub struct CountExecutor<R: Reader> {
    reader: R,
}

impl<R: Reader> CountExecutor<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn execute(&self, filepath: &str) -> Result<usize> {
        let rows = self.reader.read(filepath)?;
        Ok(rows.len())
    }
}
