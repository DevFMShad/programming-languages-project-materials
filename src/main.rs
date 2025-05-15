mod statement;
mod token;
mod tokenizer;
mod parser;

use std::io::{self, Write};
use parser::Parser;


fn main() {
    println!("----------------------Welcome to The SQL Parser CLI🤗---------------------------");
    println!("========================Made by Fuad Mahmud Shad================================");
    println!("SQL Parser CLI. Enter SQL queries (SELECT or CREATE TABLE). Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting...");
            break;
        }

        if input.is_empty() {
            continue;
        }

        let mut parser = Parser::new(input);
        match parser.parse() {
            Ok(statement) => println!("Parsed Statement: {:#?}", statement),
            Err(e) => println!("Error: {}", e),
        }
    }
}