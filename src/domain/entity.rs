use std::collections::HashMap;

/// レコードを表すエンティティ
#[derive(Debug, Clone)]
pub struct Record {
    pub fields: HashMap<String, String>,
}

impl Record {
    pub fn new(fields: HashMap<String, String>) -> Self {
        Self { fields }
    }

    pub fn get(&self, field_name: &str) -> Option<&String> {
        self.fields.get(field_name)
    }
}

/// テーブルを表すエンティティ
#[derive(Debug, Clone)]
pub struct Table {
    pub headers: Vec<String>,
    pub records: Vec<Record>,
}

impl Table {
    pub fn new(headers: Vec<String>, records: Vec<Record>) -> Self {
        Self { headers, records }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_new_and_get() {
        let mut fields = HashMap::new();
        fields.insert("id".to_string(), "1".to_string());
        fields.insert("name".to_string(), "test".to_string());

        let record = Record::new(fields);

        assert_eq!(record.get("id"), Some(&"1".to_string()));
        assert_eq!(record.get("name"), Some(&"test".to_string()));
        assert_eq!(record.get("nonexistent"), None);
    }

    #[test]
    fn test_table_new() {
        let headers = vec!["id".to_string(), "name".to_string()];
        let records = vec![
            Record::new(HashMap::from([
                ("id".to_string(), "1".to_string()),
                ("name".to_string(), "Alice".to_string()),
            ])),
            Record::new(HashMap::from([
                ("id".to_string(), "2".to_string()),
                ("name".to_string(), "Bob".to_string()),
            ])),
        ];

        let table = Table::new(headers.clone(), records.clone());

        assert_eq!(table.headers, headers);
        assert_eq!(table.records.len(), 2);
    }
}
