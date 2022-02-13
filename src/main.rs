use std::io;
use std::io::Write;

struct Row {
    id: i32,
    dog_name: String,
    breed: String,
}

impl Row {
    fn new(id: i32, dog_name: String, breed: String) -> Self {
        Row {
            id,
            dog_name,
            breed,
        }
    }
}

enum MetaResult {
    Success,
    Failure,
    Unrecognized,
    ExitSuccess,
}

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
        StatementType::Insert => Some(Row::new(123, "Evie".to_string(), "Jindo".to_string())),
        _ => None,
    };

    Statement {
        statement_type,
        statement_str,
        row,
    }
}

fn main() {
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
                StatementType::Insert => println!("This is insert land"),
                StatementType::Select => println!("This is select land"),
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
        p => {
            print!("Error: {} is an unrecognized command", p);
            return MetaResult::Unrecognized;
        }
    };
}
