mod database;

use crate::database::{Row, Table};
use std::io;
use std::io::Write;

#[derive(Debug, PartialEq)]
enum MetaResult {
    Success,
    Failure,
    Unrecognized,
    ExitSuccess,
}

#[derive(Debug, PartialEq)]
enum StatementType {
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

struct Statement {
    statement_type: StatementType,
    statement_str: String,
    row: Option<Row>,
}

fn prepare_statement(statement_str: String) -> Statement {
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

fn main() {
    let mut table = Table::new();
    println!("Welcome to EVLite Beta\nType 'help' for usage hints");
    loop {
        let user_input = prompt("evlite> ");

        // meta commands lead with a "."
        if user_input.starts_with(".") {
            match handle_meta_command(&user_input) {
                MetaResult::Success => println!("Not implemented yet!"),
                MetaResult::Failure => println!("Not implemented yet!"),
                MetaResult::Unrecognized => {
                    println!("Error: {} is unrecognizable command", user_input)
                }
                MetaResult::ExitSuccess => break,
            }
        } else {
            let statement = prepare_statement(user_input);
            match statement.statement_type {
                StatementType::Insert => match statement.row {
                    Some(x) => table.insert_row(&x),
                    None => println!("No row to insert!"),
                },
                StatementType::Select => {
                    table.select_rows();
                }
                StatementType::Unrecognized => {
                    println!(
                        "Error: {} is unrecognized statement",
                        statement.statement_str
                    );
                    break;
                }
            }
        }
        println!();
    }
}

// Provides the REPL prompt
fn prompt(name: &str) -> String {
    let mut user_input = String::new();
    print!("{}", name);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error: could not read user input");
    user_input.trim().to_string()
}

fn handle_meta_command(command: &String) -> MetaResult {
    match command.as_str() {
        ".quit" => {
            println!("Goodbye");
            return MetaResult::ExitSuccess;
        }
        _ => return MetaResult::Unrecognized,
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

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
    fn test_handle_meta_command() {
        assert_eq!(
            handle_meta_command(&".fake".to_string()),
            MetaResult::Unrecognized
        );
        assert_eq!(
            handle_meta_command(&".quit".to_string()),
            MetaResult::ExitSuccess
        );
    }
}
