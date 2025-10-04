use anyhow::{anyhow, Result};

/// SQLクエリを表す構造体
#[derive(Debug, Clone)]
pub struct Query {
    pub select_fields: SelectFields,
    pub from: String,
    pub where_clause: Option<WhereClause>,
}

#[derive(Debug, Clone)]
pub enum SelectFields {
    All,
    Fields(Vec<String>),
}

/// WHERE句を表す構造体（将来の拡張用）
#[derive(Debug, Clone)]
pub struct WhereClause {
    // 今後拡張予定
}

/// クエリパーサー
pub struct QueryParser;

impl QueryParser {
    pub fn parse(query: &str) -> Result<Query> {
        let query = query.trim();

        // 簡易的なパーサー実装
        let lower = query.to_lowercase();

        if !lower.starts_with("select") {
            return Err(anyhow!("Query must start with SELECT"));
        }

        // SELECT部分とFROM部分を分割
        let from_pos = lower.find(" from ")
            .ok_or_else(|| anyhow!("FROM clause not found"))?;

        let select_part = &query[6..from_pos].trim(); // "SELECT"の後
        let from_part = &query[from_pos + 6..].trim(); // " FROM "の後

        // SELECT フィールドのパース
        let select_fields = if select_part.trim() == "*" {
            SelectFields::All
        } else {
            let fields = select_part
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();
            SelectFields::Fields(fields)
        };

        // FROM句のパース（ファイルパス）
        let from = from_part.to_string();

        Ok(Query {
            select_fields,
            from,
            where_clause: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_select_all() {
        let query = "SELECT * FROM ./sample.csv";
        let parsed = QueryParser::parse(query).unwrap();

        assert!(matches!(parsed.select_fields, SelectFields::All));
        assert_eq!(parsed.from, "./sample.csv");
    }

    #[test]
    fn test_parse_select_fields() {
        let query = "SELECT id, name FROM ./sample.csv";
        let parsed = QueryParser::parse(query).unwrap();

        if let SelectFields::Fields(fields) = parsed.select_fields {
            assert_eq!(fields, vec!["id", "name"]);
        } else {
            panic!("Expected Fields variant");
        }
    }

    #[test]
    fn test_parse_case_insensitive() {
        let query = "select * from ./test.csv";
        let parsed = QueryParser::parse(query).unwrap();

        assert!(matches!(parsed.select_fields, SelectFields::All));
        assert_eq!(parsed.from, "./test.csv");
    }

    #[test]
    fn test_parse_mixed_case() {
        let query = "SeLeCt id, name FrOm ./data.csv";
        let parsed = QueryParser::parse(query).unwrap();

        if let SelectFields::Fields(fields) = parsed.select_fields {
            assert_eq!(fields, vec!["id", "name"]);
        } else {
            panic!("Expected Fields variant");
        }
        assert_eq!(parsed.from, "./data.csv");
    }

    #[test]
    fn test_parse_with_whitespace() {
        let query = "  SELECT   id  ,  name   FROM   ./test.csv  ";
        let parsed = QueryParser::parse(query).unwrap();

        if let SelectFields::Fields(fields) = parsed.select_fields {
            assert_eq!(fields, vec!["id", "name"]);
        } else {
            panic!("Expected Fields variant");
        }
    }

    #[test]
    fn test_parse_error_no_select() {
        let query = "FROM ./test.csv";
        let result = QueryParser::parse(query);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("SELECT"));
    }

    #[test]
    fn test_parse_error_no_from() {
        let query = "SELECT * WHERE id = 1";
        let result = QueryParser::parse(query);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("FROM"));
    }

    #[test]
    fn test_parse_single_field() {
        let query = "SELECT name FROM ./test.csv";
        let parsed = QueryParser::parse(query).unwrap();

        if let SelectFields::Fields(fields) = parsed.select_fields {
            assert_eq!(fields, vec!["name"]);
        } else {
            panic!("Expected Fields variant");
        }
    }

    #[test]
    fn test_parse_multiple_fields() {
        let query = "SELECT id, name, team_id, note FROM ./test.csv";
        let parsed = QueryParser::parse(query).unwrap();

        if let SelectFields::Fields(fields) = parsed.select_fields {
            assert_eq!(fields, vec!["id", "name", "team_id", "note"]);
        } else {
            panic!("Expected Fields variant");
        }
    }
}
