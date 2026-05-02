pub mod count;
pub mod header;

pub use count::CountExecutor;
pub use header::HeaderExecutor;

use anyhow::Result;

pub trait Reader {
    fn read(&self, filepath: &str, disable_header: bool) -> Result<Vec<Vec<String>>>;
}
