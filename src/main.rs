mod b_tree;
mod database;
mod statement;

use crate::database::Table;
use crate::statement::{prepare_statement, StatementType};
use std::io;
use std::io::Write;

#[derive(Debug, PartialEq)]
enum MetaResult {
    Success,
    Failure,
    Unrecognized,
    ExitSuccess,
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
