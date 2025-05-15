use crate::statement::{Statement, Expression, BinaryOperator, UnaryOperator, TableColumn, DBType, Constraint};
use crate::token::{Token, Keyword};
use crate::tokenizer::{Tokenizer, TokenizerError};
use std::fmt;
use std::convert::TryInto;

#[derive(Debug)]
pub enum ParseError {
    TokenizerError(TokenizerError),
    UnexpectedToken(Token),
    InvalidColumnType,
    NumberTooLarge(String),
    InvalidVarcharLength(String),
    UnexpectedEnd,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::TokenizerError(e) => write!(f, "Tokenizer error: {}", e),
            ParseError::UnexpectedToken(t) => write!(f, "Unexpected token: {}", t),
            ParseError::InvalidColumnType => write!(f, "Invalid column type"),
            ParseError::NumberTooLarge(s) => write!(f, "Number too large for usize: {}", s),
            ParseError::InvalidVarcharLength(s) => write!(f, "Invalid VARCHAR length: {}", s),
            ParseError::UnexpectedEnd => write!(f, "Unexpected end of input"),
        }
    }
}

pub struct Parser<'a> {
    tokens: Vec<Token>,
    position: usize,
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            tokens: Vec::new(),
            position: 0,
            input,
        }
    }

    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        if let Some(token) = self.current_token() {
            if *token == expected {
                self.advance();
                Ok(())
            } else {
                Err(ParseError::UnexpectedToken(token.clone()))
            }
        } else {
            Err(ParseError::UnexpectedEnd)
        }
    }

    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        if let Some(Token::Identifier(ident)) = self.current_token() {
            let ident = ident.clone();
            self.advance();
            Ok(ident)
        } else {
            Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof)))
        }
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<Expression, ParseError> {
        let mut left = self.parse_prefix()?;

        while let Some(token) = self.current_token() {
            // Stop parsing expressions if we encounter tokens that belong to the outer SELECT structure
            if matches!(token, Token::Keyword(Keyword::From | Keyword::Where | Keyword::Order) | Token::Semicolon | Token::Comma | Token::RightParentheses) {
                break;
            }

            let next_precedence = self.get_precedence(token)?;
            if next_precedence <= precedence {
                break;
            }

            left = self.parse_infix(left, next_precedence)?;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expression, ParseError> {
        let token = self.current_token().cloned().ok_or(ParseError::UnexpectedEnd)?;
        self.advance();

        match token {
            Token::Number(n) => Ok(Expression::Number(n)),
            Token::String(s) => Ok(Expression::String(s)),
            Token::Identifier(ident) => Ok(Expression::Identifier(ident)),
            Token::Keyword(Keyword::True) => Ok(Expression::Bool(true)),
            Token::Keyword(Keyword::False) => Ok(Expression::Bool(false)),
            Token::LeftParentheses => {
                let expr = self.parse_expression(0)?;
                self.expect_token(Token::RightParentheses)?;
                Ok(expr)
            }
            Token::Keyword(Keyword::Not) | Token::Minus | Token::Plus => {
                let operator = match token {
                    Token::Keyword(Keyword::Not) => UnaryOperator::Not,
                    Token::Minus => UnaryOperator::Minus,
                    Token::Plus => UnaryOperator::Plus,
                    _ => unreachable!(),
                };
                let operand = self.parse_expression(50)?; // High precedence for unary
                Ok(Expression::UnaryOperation {
                    operand: Box::new(operand),
                    operator,
                })
            }
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn parse_infix(&mut self, left: Expression, precedence: u8) -> Result<Expression, ParseError> {
        let token = self.current_token().cloned().ok_or(ParseError::UnexpectedEnd)?;
        self.advance();

        let operator = match token {
            Token::Plus => BinaryOperator::Plus,
            Token::Minus => BinaryOperator::Minus,
            Token::Star => BinaryOperator::Multiply,
            Token::Divide => BinaryOperator::Divide,
            Token::GreaterThan => BinaryOperator::GreaterThan,
            Token::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
            Token::LessThan => BinaryOperator::LessThan,
            Token::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
            Token::Equal => BinaryOperator::Equal,
            Token::NotEqual => BinaryOperator::NotEqual,
            Token::Keyword(Keyword::And) => BinaryOperator::And,
            Token::Keyword(Keyword::Or) => BinaryOperator::Or,
            Token::Keyword(Keyword::Asc) => {
                return Ok(Expression::UnaryOperation {
                    operand: Box::new(left),
                    operator: UnaryOperator::Asc,
                });
            }
            Token::Keyword(Keyword::Desc) => {
                return Ok(Expression::UnaryOperation {
                    operand: Box::new(left),
                    operator: UnaryOperator::Desc,
                });
            }
            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        let right = self.parse_expression(precedence)?;
        Ok(Expression::BinaryOperation {
            left_operand: Box::new(left),
            operator,
            right_operand: Box::new(right),
        })
    }

    fn get_precedence(&self, token: &Token) -> Result<u8, ParseError> {
        match token {
            Token::Keyword(Keyword::Or) => Ok(10),
            Token::Keyword(Keyword::And) => Ok(20),
            Token::Equal | Token::NotEqual => Ok(30),
            Token::GreaterThan | Token::GreaterThanOrEqual | Token::LessThan | Token::LessThanOrEqual => Ok(40),
            Token::Plus | Token::Minus => Ok(50),
            Token::Star | Token::Divide => Ok(60),
            Token::Keyword(Keyword::Asc) | Token::Keyword(Keyword::Desc) => Ok(5),
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    fn parse_select(&mut self) -> Result<Statement, ParseError> {
        // Parse columns
        let mut columns = Vec::new();
        if self.current_token() == Some(&Token::Keyword(Keyword::From)) {
            return Err(ParseError::UnexpectedToken(Token::Keyword(Keyword::From)));
        }

        loop {
            let expr = self.parse_expression(0)?;
            columns.push(expr);
            if self.current_token() == Some(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        // Expect FROM
        self.expect_token(Token::Keyword(Keyword::From))?;
        let from = self.parse_identifier()?;

        // Parse optional WHERE
        let r#where = if self.current_token() == Some(&Token::Keyword(Keyword::Where)) {
            self.advance();
            Some(self.parse_expression(0)?)
        } else {
            None
        };

        // Parse optional ORDER BY
        let mut orderby = Vec::new();
        if self.current_token() == Some(&Token::Keyword(Keyword::Order)) {
            self.advance();
            self.expect_token(Token::Keyword(Keyword::By))?;
            loop {
                let expr = self.parse_expression(0)?;
                orderby.push(expr);
                if self.current_token() == Some(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Expect semicolon
        if self.current_token() != Some(&Token::Semicolon) {
            return Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof)));
        }
        self.advance();

        Ok(Statement::Select {
            columns,
            from,
            r#where,
            orderby,
        })
    }

    fn parse_create_table(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Keyword(Keyword::Table))?;
        let table_name = self.parse_identifier()?;
        self.expect_token(Token::LeftParentheses)?;

        let mut column_list = Vec::new();
        loop {
            let column_name = self.parse_identifier()?;
            let column_type = match self.current_token() {
                Some(Token::Keyword(Keyword::Int)) => {
                    self.advance();
                    DBType::Int
                }
                Some(Token::Keyword(Keyword::Bool)) => {
                    self.advance();
                    DBType::Bool
                }
                Some(Token::Keyword(Keyword::Varchar)) => {
                    self.advance();
                    self.expect_token(Token::LeftParentheses)?;
                    if let Some(Token::Number(len)) = self.current_token() {
                        // Dereference len to get u64, then convert to usize
                        let len_usize = (*len).try_into().map_err(|_| ParseError::NumberTooLarge(len.to_string()))?;
                        // Validate VARCHAR length
                        if len_usize == 0 {
                            return Err(ParseError::InvalidVarcharLength("VARCHAR length must be greater than 0".to_string()));
                        }
                        if len_usize > 65535 {
                            return Err(ParseError::InvalidVarcharLength("VARCHAR length too large (max 65535)".to_string()));
                        }
                        self.advance();
                        self.expect_token(Token::RightParentheses)?;
                        DBType::Varchar(len_usize)
                    } else {
                        return Err(ParseError::InvalidColumnType);
                    }
                }
                _ => return Err(ParseError::InvalidColumnType),
            };

            let mut constraints = Vec::new();
            while let Some(token) = self.current_token() {
                match token {
                    Token::Keyword(Keyword::Primary) => {
                        self.advance();
                        self.expect_token(Token::Keyword(Keyword::Key))?;
                        constraints.push(Constraint::PrimaryKey);
                    }
                    Token::Keyword(Keyword::Not) => {
                        self.advance();
                        self.expect_token(Token::Keyword(Keyword::Null))?;
                        constraints.push(Constraint::NotNull);
                    }
                    Token::Keyword(Keyword::Check) => {
                        self.advance();
                        self.expect_token(Token::LeftParentheses)?;
                        let expr = self.parse_expression(0)?;
                        self.expect_token(Token::RightParentheses)?;
                        constraints.push(Constraint::Check(expr));
                    }
                    _ => break,
                }
            }

            column_list.push(TableColumn {
                column_name,
                column_type,
                constraints,
            });

            if self.current_token() == Some(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        self.expect_token(Token::RightParentheses)?;

        // Check for semicolon and provide a specific error message if missing
        if self.current_token() != Some(&Token::Semicolon) {
            return Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof)));
        }
        self.advance();

        Ok(Statement::CreateTable {
            table_name,
            column_list,
        })
    }

    pub fn parse(&mut self) -> Result<Statement, ParseError> {
        let mut tokenizer = Tokenizer::new(self.input);
        self.tokens = tokenizer.tokenize().map_err(ParseError::TokenizerError)?;
        self.position = 0;

        match self.current_token() {
            Some(Token::Keyword(Keyword::Select)) => {
                self.advance();
                self.parse_select()
            }
            Some(Token::Keyword(Keyword::Create)) => {
                self.advance();
                self.parse_create_table()
            }
            Some(token) => Err(ParseError::UnexpectedToken(token.clone())),
            None => Err(ParseError::UnexpectedEnd),
        }
    }
}