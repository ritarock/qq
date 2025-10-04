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

/// WHERE句を表す構造体
#[derive(Debug, Clone, PartialEq)]
pub struct WhereClause {
    pub conditions: Vec<Condition>,
}

/// 条件式を表す構造体
#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub field: String,
    pub operator: Operator,
    pub value: String,
}

/// 比較演算子
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Equal,           // =
    NotEqual,        // !=
    GreaterThan,     // >
    GreaterOrEqual,  // >=
    LessThan,        // <
    LessOrEqual,     // <=
}

impl Operator {
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "=" => Ok(Operator::Equal),
            "!=" => Ok(Operator::NotEqual),
            ">" => Ok(Operator::GreaterThan),
            ">=" => Ok(Operator::GreaterOrEqual),
            "<" => Ok(Operator::LessThan),
            "<=" => Ok(Operator::LessOrEqual),
            _ => Err(anyhow!("Unknown operator: {}", s)),
        }
    }
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
        let remaining = &query[from_pos + 6..].trim(); // " FROM "の後

        // WHERE句の有無を確認
        let where_pos = lower[from_pos..].find(" where ");
        let (from_part, where_part) = if let Some(pos) = where_pos {
            let actual_pos = from_pos + pos;
            (&query[from_pos + 6..actual_pos].trim(), Some(&query[actual_pos + 7..].trim()))
        } else {
            (remaining, None)
        };

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

        // WHERE句のパース
        let where_clause = if let Some(where_str) = where_part {
            Some(Self::parse_where(where_str)?)
        } else {
            None
        };

        Ok(Query {
            select_fields,
            from,
            where_clause,
        })
    }

    fn parse_where(where_str: &str) -> Result<WhereClause> {
        // 現在はAND条件のみサポート
        let conditions_str: Vec<&str> = where_str.split(" and ").collect();
        let mut conditions = Vec::new();

        for cond_str in conditions_str {
            let cond_str = cond_str.trim();

            // 演算子を探す（長い順にチェック）
            let (field, operator, value) = if let Some(pos) = cond_str.find(">=") {
                let field = cond_str[..pos].trim();
                let value = cond_str[pos + 2..].trim();
                (field, Operator::GreaterOrEqual, value)
            } else if let Some(pos) = cond_str.find("<=") {
                let field = cond_str[..pos].trim();
                let value = cond_str[pos + 2..].trim();
                (field, Operator::LessOrEqual, value)
            } else if let Some(pos) = cond_str.find("!=") {
                let field = cond_str[..pos].trim();
                let value = cond_str[pos + 2..].trim();
                (field, Operator::NotEqual, value)
            } else if let Some(pos) = cond_str.find("=") {
                let field = cond_str[..pos].trim();
                let value = cond_str[pos + 1..].trim();
                (field, Operator::Equal, value)
            } else if let Some(pos) = cond_str.find(">") {
                let field = cond_str[..pos].trim();
                let value = cond_str[pos + 1..].trim();
                (field, Operator::GreaterThan, value)
            } else if let Some(pos) = cond_str.find("<") {
                let field = cond_str[..pos].trim();
                let value = cond_str[pos + 1..].trim();
                (field, Operator::LessThan, value)
            } else {
                return Err(anyhow!("Invalid condition: {}", cond_str));
            };

            conditions.push(Condition {
                field: field.to_string(),
                operator,
                value: value.to_string(),
            });
        }

        Ok(WhereClause { conditions })
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

    #[test]
    fn test_parse_where_single_condition() {
        let query = "SELECT * FROM ./test.csv WHERE id = 1";
        let parsed = QueryParser::parse(query).unwrap();

        assert!(parsed.where_clause.is_some());
        let where_clause = parsed.where_clause.unwrap();
        assert_eq!(where_clause.conditions.len(), 1);
        assert_eq!(where_clause.conditions[0].field, "id");
        assert_eq!(where_clause.conditions[0].operator, Operator::Equal);
        assert_eq!(where_clause.conditions[0].value, "1");
    }

    #[test]
    fn test_parse_where_multiple_conditions() {
        let query = "SELECT * FROM ./test.csv WHERE id = 1 and name = Alice";
        let parsed = QueryParser::parse(query).unwrap();

        let where_clause = parsed.where_clause.unwrap();
        assert_eq!(where_clause.conditions.len(), 2);
        assert_eq!(where_clause.conditions[0].field, "id");
        assert_eq!(where_clause.conditions[0].value, "1");
        assert_eq!(where_clause.conditions[1].field, "name");
        assert_eq!(where_clause.conditions[1].value, "Alice");
    }

    #[test]
    fn test_parse_where_operators() {
        let test_cases = vec![
            ("SELECT * FROM ./test.csv WHERE id = 1", Operator::Equal),
            ("SELECT * FROM ./test.csv WHERE id != 1", Operator::NotEqual),
            ("SELECT * FROM ./test.csv WHERE id > 1", Operator::GreaterThan),
            ("SELECT * FROM ./test.csv WHERE id >= 1", Operator::GreaterOrEqual),
            ("SELECT * FROM ./test.csv WHERE id < 1", Operator::LessThan),
            ("SELECT * FROM ./test.csv WHERE id <= 1", Operator::LessOrEqual),
        ];

        for (query, expected_op) in test_cases {
            let parsed = QueryParser::parse(query).unwrap();
            let where_clause = parsed.where_clause.unwrap();
            assert_eq!(where_clause.conditions[0].operator, expected_op);
        }
    }

    #[test]
    fn test_parse_where_case_insensitive() {
        let query = "SELECT * FROM ./test.csv WHERE team_id = 1";
        let parsed = QueryParser::parse(query).unwrap();

        let where_clause = parsed.where_clause.unwrap();
        assert_eq!(where_clause.conditions[0].field, "team_id");
    }

    #[test]
    fn test_parse_where_with_select_fields() {
        let query = "SELECT id, name FROM ./test.csv WHERE team_id = 1";
        let parsed = QueryParser::parse(query).unwrap();

        if let SelectFields::Fields(fields) = parsed.select_fields {
            assert_eq!(fields, vec!["id", "name"]);
        } else {
            panic!("Expected Fields variant");
        }

        let where_clause = parsed.where_clause.unwrap();
        assert_eq!(where_clause.conditions[0].field, "team_id");
        assert_eq!(where_clause.conditions[0].value, "1");
    }

    #[test]
    fn test_parse_where_complex() {
        let query = "SELECT id, name FROM ./test.csv WHERE team_id >= 1 and id < 10";
        let parsed = QueryParser::parse(query).unwrap();

        let where_clause = parsed.where_clause.unwrap();
        assert_eq!(where_clause.conditions.len(), 2);
        assert_eq!(where_clause.conditions[0].operator, Operator::GreaterOrEqual);
        assert_eq!(where_clause.conditions[1].operator, Operator::LessThan);
    }
}
