use super::entity::Table;
use anyhow::Result;

/// データソースからテーブルを読み込むためのリポジトリインターフェース
pub trait TableRepository {
    fn load(&self, source: &str) -> Result<Table>;
}
