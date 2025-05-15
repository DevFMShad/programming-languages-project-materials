// tokenizer.rs - Converts SQL query strings into a sequence of tokens for the SQL parser.
// Supports tokenization of single-character tokens, multi-character tokens, numbers, strings, keywords, and identifiers
// (Functionality #1–6, 7 points: #1 single-character tokens, #2 multi-character tokens, #3 numbers,
// #4 strings, #5 keywords, #6 identifiers).
// Includes error handling for unterminated strings and unexpected characters (Functionality #16, 2 points).

// Import token definitions and utilities for tokenization.
use crate::token::{Token, Keyword};
use std::iter::Peekable;
use std::str::Chars;

// Define errors for tokenization issues.
#[derive(Debug, PartialEq)]
pub enum TokenizerError {
    UnterminatedString, // String missing closing quote.
    UnexpectedChar(char), // Invalid character encountered.
}

// Tokenizer struct for processing input string.
pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>, // Iterator to peek and consume characters.
}

impl<'a> Tokenizer<'a> {
    // Create a new Tokenizer from input string.
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            input: input.chars().peekable(),
        }
    }

    // Get next character and advance iterator.
    fn next_char(&mut self) -> Option<char> {
        self.input.next()
    }

    // Peek at next character without advancing.
    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    // Tokenize input into a vector of tokens.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens = Vec::new();

        // Process each character in input.
        while let Some(&c) = self.peek_char() {
            match c {
                ' ' | '\t' | '\n' => {
                    self.next_char(); // Skip whitespace.
                }
                '(' => {
                    self.next_char();
                    tokens.push(Token::LeftParentheses);
                }
                ')' => {
                    self.next_char();
                    tokens.push(Token::RightParentheses);
                }
                ',' => {
                    self.next_char();
                    tokens.push(Token::Comma);
                }
                ';' => {
                    self.next_char();
                    tokens.push(Token::Semicolon);
                }
                '+' => {
                    self.next_char();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.next_char();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    self.next_char();
                    tokens.push(Token::Star); // For SELECT * or multiplication.
                }
                '/' => {
                    self.next_char();
                    tokens.push(Token::Divide);
                }
                '=' => {
                    self.next_char();
                    tokens.push(Token::Equal); // For equality comparisons.
                }
                '>' => {
                    self.next_char();
                    if self.peek_char() == Some(&'=') {
                        self.next_char();
                        tokens.push(Token::GreaterThanOrEqual);
                    } else {
                        tokens.push(Token::GreaterThan);
                    }
                }
                '<' => {
                    self.next_char();
                    if self.peek_char() == Some(&'=') {
                        self.next_char();
                        tokens.push(Token::LessThanOrEqual);
                    } else {
                        tokens.push(Token::LessThan);
                    }
                }
                '!' => {
                    self.next_char();
                    if self.peek_char() == Some(&'=') {
                        self.next_char();
                        tokens.push(Token::NotEqual);
                    } else {
                        return Err(TokenizerError::UnexpectedChar('!')); // Error for lone !.
                    }
                }
                '"' | '\'' => {
                    let quote = c;
                    self.next_char();
                    let mut string = String::new();
                    while let Some(c) = self.next_char() {
                        if c == quote {
                            break; // End of string.
                        }
                        if c == '\\' {
                            // Handle escaped characters.
                            if let Some(next) = self.next_char() {
                                match next {
                                    '"' | '\'' => string.push(next),
                                    '\\' => string.push('\\'),
                                    _ => string.push(next),
                                }
                            } else {
                                return Err(TokenizerError::UnterminatedString);
                            }
                            continue;
                        }
                        string.push(c);
                    }
                    if self.peek_char().is_none() && string.is_empty() {
                        return Err(TokenizerError::UnterminatedString); // Error for empty unterminated string.
                    }
                    tokens.push(Token::String(string)); // Store string literal.
                }
                '0'..='9' => {
                    let mut num = String::new();
                    while let Some(&c) = self.peek_char() {
                        if c.is_digit(10) {
                            num.push(c);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    let number = num.parse::<u64>().unwrap(); // Convert to u64.
                    tokens.push(Token::Number(number));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = String::new();
                    while let Some(&c) = self.peek_char() {
                        if c.is_alphabetic() || c.is_digit(10) || c == '_' {
                            ident.push(c);
                            self.next_char();
                        } else {
                            break;
                        }
                    }
                    let ident_lower = ident.to_lowercase();
                    // Map identifiers to keywords or keep as identifiers.
                    let token = match ident_lower.as_str() {
                        "select" => Token::Keyword(Keyword::Select),
                        "from" => Token::Keyword(Keyword::From),
                        "where" => Token::Keyword(Keyword::Where),
                        "order" => Token::Keyword(Keyword::Order),
                        "by" => Token::Keyword(Keyword::By),
                        "create" => Token::Keyword(Keyword::Create),
                        "table" => Token::Keyword(Keyword::Table),
                        "int" => Token::Keyword(Keyword::Int),
                        "varchar" => Token::Keyword(Keyword::Varchar),
                        "bool" => Token::Keyword(Keyword::Bool),
                        "primary" => Token::Keyword(Keyword::Primary),
                        "key" => Token::Keyword(Keyword::Key),
                        "not" => Token::Keyword(Keyword::Not),
                        "null" => Token::Keyword(Keyword::Null),
                        "check" => Token::Keyword(Keyword::Check),
                        "true" => Token::Keyword(Keyword::True),
                        "false" => Token::Keyword(Keyword::False),
                        "and" => Token::Keyword(Keyword::And),
                        "or" => Token::Keyword(Keyword::Or),
                        "asc" => Token::Keyword(Keyword::Asc),
                        "desc" => Token::Keyword(Keyword::Desc),
                        _ => Token::Identifier(ident),
                    };
                    tokens.push(token);
                }
                _ => {
                    self.next_char();
                    return Err(TokenizerError::UnexpectedChar(c)); // Error for invalid chars.
                }
            }
        }
        tokens.push(Token::Eof); // Mark end of input.
        Ok(tokens)
    }
}