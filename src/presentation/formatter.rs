use crate::domain::entity::Table;

/// テーブルをフォーマットして表示
pub struct TableFormatter;

impl TableFormatter {
    pub fn print(table: &Table) {
        if table.records.is_empty() {
            println!("No records found");
            return;
        }

        // 各カラムの最大幅を計算
        let mut col_widths: Vec<usize> = table.headers.iter().map(|h| h.len()).collect();

        for record in &table.records {
            for (i, header) in table.headers.iter().enumerate() {
                if let Some(value) = record.get(header) {
                    let value_str = value.to_string();
                    col_widths[i] = col_widths[i].max(value_str.len());
                }
            }
        }

        // ヘッダーを表示
        let header_row: Vec<String> = table
            .headers
            .iter()
            .enumerate()
            .map(|(i, h)| format!("{:<width$}", h, width = col_widths[i]))
            .collect();
        println!("{}", header_row.join("  "));

        // セパレーター行を表示
        let separator: Vec<String> = col_widths
            .iter()
            .map(|&w| "-".repeat(w))
            .collect();
        println!("{}", separator.join("  "));

        // レコードを表示
        for record in &table.records {
            let values: Vec<String> = table
                .headers
                .iter()
                .enumerate()
                .map(|(i, header)| {
                    let value_str = record
                        .get(header)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| String::new());
                    format!("{:<width$}", value_str, width = col_widths[i])
                })
                .collect();

            println!("{}", values.join("  "));
        }
    }
}
