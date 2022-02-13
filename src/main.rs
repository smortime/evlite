use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to EVLite Beta\nType 'help' for usage hints");
    loop {
        match prompt("evlite> ").as_str() {
            "quit" => {
                println!("Goodbye"); 
                break;
            },
            _ => println!("Invalid command"),
        };
        println!();
    }
}

fn prompt(name: &str) -> String {
    let mut user_input = String::new();
    print!("{}", name);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_input)
        .expect("Error: could not read line");
    user_input.trim().to_string()
}
