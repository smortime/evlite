use crate::database::Row;

use sqlparser::dialect::SQLiteDialect;
use sqlparser::parser::Parser;

#[derive(Debug, PartialEq)]
pub enum StatementType {
    Select,
    Insert,
    Unrecognized,
}

impl StatementType {
    fn get_statement_type(statement: &String) -> StatementType {
        if statement.starts_with("select") {
            return StatementType::Select;
        } else if statement.starts_with("insert") {
            return StatementType::Insert;
        }
        StatementType::Unrecognized
    }
}

pub struct Statement {
    pub statement_type: StatementType,
    pub statement_str: String,
    pub row: Option<Row>,
}

pub fn prepare_statement(statement_str: String) -> Statement {
    let statement_type = StatementType::get_statement_type(&statement_str);
    let row = match statement_type {
        StatementType::Insert => Some(Row::new(&statement_str)),
        _ => None,
    };

    Statement {
        statement_type,
        statement_str,
        row,
    }
}

pub fn parse_statement(statement_str: String) {
    let dialect = SQLiteDialect {};
    let ast = Parser::parse_sql(&dialect, &statement_str).unwrap();
    print!("{:?}", ast);
}

#[cfg(test)]
mod tests {
    use crate::statement::*;

    #[test]
    fn statement_tests() {
        let select = prepare_statement("select *".to_string());
        assert_eq!(select.statement_type, StatementType::Select);
        assert!(select.row.is_none());
        let insert = prepare_statement("insert 1 evie jindo".to_string());
        assert_eq!(insert.statement_type, StatementType::Insert);
        assert!(!insert.row.is_none());
    }

    #[test]
    fn parse_statement_tests() {
        let statement = "select * from dogs";
        parse_statement(statement.to_string());
    }
}
