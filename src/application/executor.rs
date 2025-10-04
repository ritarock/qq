use crate::domain::entity::{Record, Table};
use crate::domain::repository::TableRepository;
use crate::application::query::{Query, SelectFields, WhereClause, Condition, Operator};
use anyhow::Result;
use std::collections::HashMap;

/// クエリを実行するエグゼキューター
pub struct QueryExecutor<R: TableRepository> {
    repository: R,
}

impl<R: TableRepository> QueryExecutor<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub fn execute(&self, query: &Query) -> Result<Table> {
        // データソースからテーブルを読み込む
        let table = self.repository.load(&query.from)?;

        // WHERE句を適用
        let filtered_table = if let Some(where_clause) = &query.where_clause {
            self.apply_where(&table, where_clause)?
        } else {
            table
        };

        // SELECT句を適用
        let result_table = self.apply_select(&filtered_table, &query.select_fields)?;

        Ok(result_table)
    }

    fn apply_select(&self, table: &Table, select_fields: &SelectFields) -> Result<Table> {
        match select_fields {
            SelectFields::All => Ok(table.clone()),
            SelectFields::Fields(fields) => {
                // 指定されたフィールドのみを抽出
                let filtered_records: Vec<Record> = table
                    .records
                    .iter()
                    .map(|record| {
                        let mut new_fields = HashMap::new();
                        for field in fields {
                            if let Some(value) = record.get(field) {
                                new_fields.insert(field.clone(), value.clone());
                            }
                        }
                        Record::new(new_fields)
                    })
                    .collect();

                Ok(Table::new(fields.clone(), filtered_records))
            }
        }
    }

    fn apply_where(&self, table: &Table, where_clause: &WhereClause) -> Result<Table> {
        let filtered_records: Vec<Record> = table
            .records
            .iter()
            .filter(|record| self.evaluate_conditions(record, &where_clause.conditions))
            .cloned()
            .collect();

        Ok(Table::new(table.headers.clone(), filtered_records))
    }

    fn evaluate_conditions(&self, record: &Record, conditions: &[Condition]) -> bool {
        // 全ての条件がANDで結合されている
        conditions.iter().all(|condition| self.evaluate_condition(record, condition))
    }

    fn evaluate_condition(&self, record: &Record, condition: &Condition) -> bool {
        let record_value = match record.get(&condition.field) {
            Some(v) => v,
            None => return false,
        };

        match &condition.operator {
            Operator::Equal => record_value == &condition.value,
            Operator::NotEqual => record_value != &condition.value,
            Operator::GreaterThan => {
                // 数値比較を試みる
                if let (Ok(rv), Ok(cv)) = (record_value.parse::<f64>(), condition.value.parse::<f64>()) {
                    rv > cv
                } else {
                    // 文字列として比較
                    record_value > &condition.value
                }
            }
            Operator::GreaterOrEqual => {
                if let (Ok(rv), Ok(cv)) = (record_value.parse::<f64>(), condition.value.parse::<f64>()) {
                    rv >= cv
                } else {
                    record_value >= &condition.value
                }
            }
            Operator::LessThan => {
                if let (Ok(rv), Ok(cv)) = (record_value.parse::<f64>(), condition.value.parse::<f64>()) {
                    rv < cv
                } else {
                    record_value < &condition.value
                }
            }
            Operator::LessOrEqual => {
                if let (Ok(rv), Ok(cv)) = (record_value.parse::<f64>(), condition.value.parse::<f64>()) {
                    rv <= cv
                } else {
                    record_value <= &condition.value
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entity::{Record, Table};
    use crate::domain::repository::TableRepository;
    use anyhow::Result;

    // モックリポジトリ
    struct MockRepository {
        table: Table,
    }

    impl MockRepository {
        fn new(table: Table) -> Self {
            Self { table }
        }
    }

    impl TableRepository for MockRepository {
        fn load(&self, _source: &str) -> Result<Table> {
            Ok(self.table.clone())
        }
    }

    fn create_test_table() -> Table {
        let headers = vec!["id".to_string(), "name".to_string(), "team_id".to_string()];
        let records = vec![
            Record::new(HashMap::from([
                ("id".to_string(), "1".to_string()),
                ("name".to_string(), "Alice".to_string()),
                ("team_id".to_string(), "1".to_string()),
            ])),
            Record::new(HashMap::from([
                ("id".to_string(), "2".to_string()),
                ("name".to_string(), "Bob".to_string()),
                ("team_id".to_string(), "2".to_string()),
            ])),
        ];
        Table::new(headers, records)
    }

    #[test]
    fn test_execute_select_all() {
        let table = create_test_table();
        let repository = MockRepository::new(table.clone());
        let executor = QueryExecutor::new(repository);

        let query = Query {
            select_fields: SelectFields::All,
            from: "test.csv".to_string(),
            where_clause: None,
        };

        let result = executor.execute(&query).unwrap();
        assert_eq!(result.headers, table.headers);
        assert_eq!(result.records.len(), 2);
    }

    #[test]
    fn test_execute_select_specific_fields() {
        let table = create_test_table();
        let repository = MockRepository::new(table);
        let executor = QueryExecutor::new(repository);

        let query = Query {
            select_fields: SelectFields::Fields(vec!["id".to_string(), "name".to_string()]),
            from: "test.csv".to_string(),
            where_clause: None,
        };

        let result = executor.execute(&query).unwrap();
        assert_eq!(result.headers, vec!["id", "name"]);
        assert_eq!(result.records.len(), 2);
        assert_eq!(result.records[0].get("id"), Some(&"1".to_string()));
        assert_eq!(result.records[0].get("name"), Some(&"Alice".to_string()));
        assert_eq!(result.records[0].get("team_id"), None);
    }

    #[test]
    fn test_apply_select_all() {
        let table = create_test_table();
        let repository = MockRepository::new(table.clone());
        let executor = QueryExecutor::new(repository);

        let result = executor.apply_select(&table, &SelectFields::All).unwrap();
        assert_eq!(result.headers, table.headers);
        assert_eq!(result.records.len(), table.records.len());
    }

    #[test]
    fn test_apply_select_specific_fields() {
        let table = create_test_table();
        let repository = MockRepository::new(table.clone());
        let executor = QueryExecutor::new(repository);

        let fields = vec!["name".to_string()];
        let result = executor.apply_select(&table, &SelectFields::Fields(fields.clone())).unwrap();

        assert_eq!(result.headers, fields);
        assert_eq!(result.records.len(), 2);
        assert_eq!(result.records[0].get("name"), Some(&"Alice".to_string()));
        assert_eq!(result.records[0].get("id"), None);
    }

    #[test]
    fn test_apply_where_equal() {
        let table = create_test_table();
        let repository = MockRepository::new(table.clone());
        let executor = QueryExecutor::new(repository);

        let where_clause = WhereClause {
            conditions: vec![Condition {
                field: "id".to_string(),
                operator: Operator::Equal,
                value: "1".to_string(),
            }],
        };

        let result = executor.apply_where(&table, &where_clause).unwrap();
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.records[0].get("id"), Some(&"1".to_string()));
    }

    #[test]
    fn test_apply_where_not_equal() {
        let table = create_test_table();
        let repository = MockRepository::new(table.clone());
        let executor = QueryExecutor::new(repository);

        let where_clause = WhereClause {
            conditions: vec![Condition {
                field: "id".to_string(),
                operator: Operator::NotEqual,
                value: "1".to_string(),
            }],
        };

        let result = executor.apply_where(&table, &where_clause).unwrap();
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.records[0].get("id"), Some(&"2".to_string()));
    }

    #[test]
    fn test_apply_where_greater_than() {
        let table = create_test_table();
        let repository = MockRepository::new(table.clone());
        let executor = QueryExecutor::new(repository);

        let where_clause = WhereClause {
            conditions: vec![Condition {
                field: "team_id".to_string(),
                operator: Operator::GreaterThan,
                value: "1".to_string(),
            }],
        };

        let result = executor.apply_where(&table, &where_clause).unwrap();
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.records[0].get("team_id"), Some(&"2".to_string()));
    }

    #[test]
    fn test_apply_where_multiple_conditions() {
        let table = create_test_table();
        let repository = MockRepository::new(table.clone());
        let executor = QueryExecutor::new(repository);

        let where_clause = WhereClause {
            conditions: vec![
                Condition {
                    field: "team_id".to_string(),
                    operator: Operator::GreaterOrEqual,
                    value: "1".to_string(),
                },
                Condition {
                    field: "id".to_string(),
                    operator: Operator::LessOrEqual,
                    value: "1".to_string(),
                },
            ],
        };

        let result = executor.apply_where(&table, &where_clause).unwrap();
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.records[0].get("id"), Some(&"1".to_string()));
    }

    #[test]
    fn test_execute_with_where() {
        let table = create_test_table();
        let repository = MockRepository::new(table);
        let executor = QueryExecutor::new(repository);

        let query = Query {
            select_fields: SelectFields::Fields(vec!["name".to_string()]),
            from: "test.csv".to_string(),
            where_clause: Some(WhereClause {
                conditions: vec![Condition {
                    field: "id".to_string(),
                    operator: Operator::Equal,
                    value: "1".to_string(),
                }],
            }),
        };

        let result = executor.execute(&query).unwrap();
        assert_eq!(result.headers, vec!["name"]);
        assert_eq!(result.records.len(), 1);
        assert_eq!(result.records[0].get("name"), Some(&"Alice".to_string()));
    }
}
