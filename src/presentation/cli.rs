use crate::application::executor::QueryExecutor;
use crate::application::query::QueryParser;
use crate::domain::repository::TableRepository;
use crate::presentation::formatter::TableFormatter;
use anyhow::Result;
use std::env;

/// CLIアプリケーション
pub struct CliApp<R: TableRepository> {
    executor: QueryExecutor<R>,
}

impl<R: TableRepository> CliApp<R> {
    pub fn new(repository: R) -> Self {
        Self {
            executor: QueryExecutor::new(repository),
        }
    }

    pub fn run(&self) -> Result<()> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            eprintln!("Usage: qq \"SELECT * FROM <file>\"");
            std::process::exit(1);
        }

        let query_str = &args[1];

        // クエリをパース
        let query = QueryParser::parse(query_str)?;

        // クエリを実行
        let result = self.executor.execute(&query)?;

        // 結果を表示
        TableFormatter::print(&result);

        Ok(())
    }
}
