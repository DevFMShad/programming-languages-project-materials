// main.rs - Entry point for the SQL Parser CLI application.
// Provides a command-line interface (CLI) for users to input SQL queries (SELECT and CREATE TABLE),
// parses them using the Parser module, and displays the parsed AST or errors.
// Implements Functionality #20 (1 point): CLI for user interaction.
// Author: Fuad Mahmud Shad (fuad.mahmud.shad@academic.email)

// Import required modules for the parser, tokenizer, token, and statement definitions.
// These modules contain the core logic for tokenizing and parsing SQL queries.
mod statement; // Defines the AST structures (e.g., Statement, Expression).
mod token; // Defines the Token enum for lexical analysis.
mod tokenizer; // Converts input strings into tokens.
mod parser; // Parses tokens into an AST.

// Import standard library modules for I/O operations.
// io is used for reading user input and writing output to the console.
use std::io::{self, Write};
// Import the Parser struct from the parser module to parse SQL queries.
use parser::Parser;

// Main function: Entry point of the CLI application.
// Sets up an interactive loop to read user input, parse SQL queries, and display results.
fn main() {
    // Print a welcome message and instructions to the user.
    // This enhances user experience and clarifies how to use the CLI.
    println!("----------------------Welcome to The SQL Parser CLI🤗---------------------------");
    println!("========================Made by Fuad Mahmud Shad================================");
    println!("SQL Parser CLI. Enter SQL queries (SELECT or CREATE TABLE). Type 'exit' to quit.");

    // Start an infinite loop to continuously prompt for user input until 'exit' is entered.
    // This allows multiple queries to be processed in one session.
    loop {
        // Print a prompt ("> ") to indicate the CLI is ready for input.
        // The prompt is kept simple for clarity.
        print!("> ");
        // Flush stdout to ensure the prompt is displayed immediately.
        // This is necessary because print! buffers output until a newline.
        io::stdout().flush().unwrap();

        // Create a new String to store the user's input.
        // String is used for dynamic input collection.
        let mut input = String::new();
        // Read a line of input from stdin into the input String.
        // unwrap() is used as we assume stdin is always available in a CLI context.
        io::stdin().read_line(&mut input).unwrap();
        // Trim the input to remove leading/trailing whitespace and newlines.
        // This ensures clean input for parsing (e.g., removes trailing \n from read_line).
        let input = input.trim();

        // Check if the input is "exit" (case-insensitive).
        // If true, exit the loop to terminate the program.
        if input.eq_ignore_ascii_case("exit") {
            // Print a goodbye message to confirm the program is exiting.
            println!("Exiting...");
            // Break the loop to end the program.
            break;
        }

        // Skip empty input (e.g., if the user presses Enter without typing).
        // This prevents unnecessary parsing attempts and keeps the CLI responsive.
        if input.is_empty() {
            continue;
        }

        // Create a new Parser instance with the user's input.
        // The Parser will tokenize and parse the input into an AST.
        let mut parser = Parser::new(input);
        // Parse the input and handle the result (Ok or Err).
        // match is used to handle both successful parsing and errors gracefully.
        match parser.parse() {
            // If parsing succeeds, print the parsed Statement (AST) in debug format.
            // {:#?} provides a pretty-printed, detailed view of the AST for clarity.
            Ok(statement) => println!("Parsed Statement: {:#?}", statement),
            // If parsing fails, print the error message.
            // This informs the user of syntax errors or invalid tokens.
            Err(e) => println!("Error: {}", e),
        }
    }
}