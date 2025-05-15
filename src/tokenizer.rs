use crate::token::{Token, Keyword};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
    current_char: Option<char>,
}

#[derive(Debug)]
pub enum TokenizerError {
    UnexpectedChar(char),
    UnterminatedString,
    InvalidNumber(String),
}

impl std::fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizerError::UnexpectedChar(c) => write!(f, "Unexpected character: {}", c),
            TokenizerError::UnterminatedString => write!(f, "Unterminated string literal"),
            TokenizerError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
        }
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars().peekable();
        let current_char = chars.next();
        Tokenizer { input: chars, current_char }
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char {
            match c {
                ' ' | '\t' | '\n' => self.advance(),
                '(' => {
                    tokens.push(Token::LeftParentheses);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RightParentheses);
                    self.advance();
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.advance();
                }
                ';' => {
                    tokens.push(Token::Semicolon);
                    self.advance();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Star);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    self.advance();
                }
                '>' => {
                    if self.peek() == Some('=') {
                        tokens.push(Token::GreaterThanOrEqual);
                        self.advance();
                    } else {
                        tokens.push(Token::GreaterThan);
                    }
                    self.advance();
                }
                '<' => {
                    if self.peek() == Some('=') {
                        tokens.push(Token::LessThanOrEqual);
                        self.advance();
                    } else {
                        tokens.push(Token::LessThan);
                    }
                    self.advance();
                }
                '=' => {
                    tokens.push(Token::Equal);
                    self.advance();
                }
                '!' => {
                    if self.peek() == Some('=') {
                        tokens.push(Token::NotEqual);
                        self.advance();
                    } else {
                        return Err(TokenizerError::UnexpectedChar('!'));
                    }
                    self.advance();
                }
                '"' | '\'' => {
                    let quote = c;
                    self.advance();
                    let mut s = String::new();
                    let mut closed = false;
                    while let Some(ch) = self.current_char {
                        if ch == quote {
                            closed = true;
                            self.advance();
                            break;
                        }
                        s.push(ch);
                        self.advance();
                    }
                    if !closed {
                        return Err(TokenizerError::UnterminatedString);
                    }
                    tokens.push(Token::String(s));
                }
                c if c.is_alphabetic() || c == '_' => {
                    let mut ident = String::new();
                    ident.push(c);
                    self.advance();
                    while let Some(ch) = self.current_char {
                        if ch.is_alphanumeric() || ch == '_' {
                            ident.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let token = match ident.to_uppercase().as_str() {
                        "SELECT" => Token::Keyword(Keyword::Select),
                        "CREATE" => Token::Keyword(Keyword::Create),
                        "TABLE" => Token::Keyword(Keyword::Table),
                        "WHERE" => Token::Keyword(Keyword::Where),
                        "ORDER" => Token::Keyword(Keyword::Order),
                        "BY" => Token::Keyword(Keyword::By),
                        "ASC" => Token::Keyword(Keyword::Asc),
                        "DESC" => Token::Keyword(Keyword::Desc),
                        "FROM" => Token::Keyword(Keyword::From),
                        "AND" => Token::Keyword(Keyword::And),
                        "OR" => Token::Keyword(Keyword::Or),
                        "NOT" => Token::Keyword(Keyword::Not),
                        "TRUE" => Token::Keyword(Keyword::True),
                        "FALSE" => Token::Keyword(Keyword::False),
                        "PRIMARY" => Token::Keyword(Keyword::Primary),
                        "KEY" => Token::Keyword(Keyword::Key),
                        "CHECK" => Token::Keyword(Keyword::Check),
                        "INT" => Token::Keyword(Keyword::Int),
                        "BOOL" => Token::Keyword(Keyword::Bool),
                        "VARCHAR" => Token::Keyword(Keyword::Varchar),
                        "NULL" => Token::Keyword(Keyword::Null),
                        _ => Token::Identifier(ident),
                    };
                    tokens.push(token);
                }
                c if c.is_digit(10) => {
                    let mut num = String::new();
                    num.push(c);
                    self.advance();
                    while let Some(ch) = self.current_char {
                        if ch.is_digit(10) {
                            num.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    let num = num.parse::<u64>().map_err(|_| TokenizerError::InvalidNumber(num.clone()))?;
                    tokens.push(Token::Number(num));
                }
                c => {
                    tokens.push(Token::Invalid(c));
                    self.advance();
                    return Err(TokenizerError::UnexpectedChar(c));
                }
            }
        }
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}