use crate::domain::entity::Table;

/// テーブルをフォーマットして表示
pub struct TableFormatter;

impl TableFormatter {
    pub fn print(table: &Table) {
        if table.records.is_empty() {
            println!("No records found");
            return;
        }

        // ヘッダーを表示
        println!("{}", table.headers.join(","));

        // レコードを表示
        for record in &table.records {
            let values: Vec<String> = table
                .headers
                .iter()
                .map(|header| {
                    record
                        .get(header)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| String::new())
                })
                .collect();

            println!("{}", values.join(","));
        }
    }
}
