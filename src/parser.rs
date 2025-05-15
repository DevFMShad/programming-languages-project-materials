// parser.rs - Implements the Pratt parser for expressions and the SQL parser for SELECT and CREATE TABLE statements.
// Supports the Pratt parser (Functionality #8–11, 10 points):
// #8: Order of operations (2 points)
// #9: Parentheses (1 point)
// #10: Binary operations (4 points)
// #11: Unary operations (3 points)
// Supports the SQL parser (Functionality #12–15, 13 points):
// #12: SELECT without WHERE/ORDER BY (3 points)
// #13: SELECT with optional WHERE/ORDER BY (3 points)
// #14: CREATE TABLE with types (4 points)
// #15: CREATE TABLE with constraints (3 points)
// Includes error handling for invalid tokens, expressions, and syntax (Functionality #17–19, 7 points).
// Supports SELECT * syntax for bonus points (2 points).

// Import necessary types from other modules for parsing and AST construction.
use crate::statement::{Statement, Expression, BinaryOperator, UnaryOperator, TableColumn, DBType, Constraint};
use crate::token::{Token, Keyword};
use crate::tokenizer::{Tokenizer};
// Use Peekable to inspect tokens without consuming them.
use std::iter::Peekable;

// Define possible parsing errors for invalid tokens or syntax.
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    InvalidVarcharLength,
    InvalidColumnType,
}

// Implement Display for ParseError to show user-friendly error messages.
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(t) => write!(f, "Unexpected token: {}", t),
            ParseError::InvalidVarcharLength => write!(f, "Invalid VARCHAR length"),
            ParseError::InvalidColumnType => write!(f, "Invalid column type"),
        }
    }
}

// Parser struct holds a Peekable iterator of tokens for parsing.
pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    // Create a new Parser from an input string, tokenizing it first.
    pub fn new(input: &str) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        // Tokenize input; return [Eof] on error to avoid panics.
        let tokens = tokenizer.tokenize().unwrap_or_else(|_| vec![Token::Eof]);
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }

    // Get the current token without consuming it.
    fn current_token(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }

    // Move to the next token.
    fn advance(&mut self) {
        self.tokens.next();
    }

    // Check if the current token matches the expected one; error if not.
    fn expect_token(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.current_token() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof)))
        }
    }

    // Parse an identifier (e.g., table or column name); error if not an identifier.
    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        if let Some(Token::Identifier(ident)) = self.current_token() {
            let ident = ident.clone();
            self.advance();
            Ok(ident)
        } else {
            Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof)))
        }
    }

    // Define operator precedence for the Pratt parser.
    fn get_precedence(&self, token: &Token) -> u8 {
        // Lower numbers mean lower precedence (e.g., OR < AND < comparisons).
        match token {
            Token::Keyword(Keyword::Or) => 10,
            Token::Keyword(Keyword::And) => 20,
            Token::Equal | Token::NotEqual | Token::GreaterThan | Token::GreaterThanOrEqual | Token::LessThan | Token::LessThanOrEqual => 30,
            Token::Plus | Token::Minus => 50,
            Token::Star | Token::Divide => 60,
            _ => 0, // Non-operators have no precedence.
        }
    }

    // Parse prefix expressions (e.g., numbers, strings, identifiers, unary ops).
    fn parse_prefix(&mut self) -> Result<Expression, ParseError> {
        match self.current_token() {
            Some(Token::Number(num)) => {
                let num = *num;
                self.advance();
                Ok(Expression::Number(num))
            }
            Some(Token::String(s)) => {
                let s = s.clone();
                self.advance();
                Ok(Expression::String(s))
            }
            Some(Token::Identifier(ident)) => {
                let ident = ident.clone();
                self.advance();
                Ok(Expression::Identifier(ident))
            }
            Some(Token::Keyword(Keyword::True)) => {
                self.advance();
                Ok(Expression::Bool(true))
            }
            Some(Token::Keyword(Keyword::False)) => {
                self.advance();
                Ok(Expression::Bool(false))
            }
            Some(Token::LeftParentheses) => {
                self.advance();
                let expr = self.parse_expression(0)?; // Parse inside parentheses.
                self.expect_token(Token::RightParentheses)?;
                Ok(expr)
            }
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_expression(80)?; // High precedence for unary minus.
                Ok(Expression::UnaryOperation {
                    operator: UnaryOperator::Minus,
                    operand: Box::new(expr),
                })
            }
            Some(Token::Plus) => {
                self.advance();
                let expr = self.parse_expression(80)?;
                Ok(Expression::UnaryOperation {
                    operator: UnaryOperator::Plus,
                    operand: Box::new(expr),
                })
            }
            Some(Token::Keyword(Keyword::Not)) => {
                self.advance();
                let expr = self.parse_expression(80)?;
                Ok(Expression::UnaryOperation {
                    operator: UnaryOperator::Not,
                    operand: Box::new(expr),
                })
            }
            _ => Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof))),
        }
    }

    // Parse infix operators (e.g., +, -, *, =, AND) based on precedence.
    fn parse_infix(&mut self, left: Expression, precedence: u8) -> Result<Expression, ParseError> {
        match self.current_token() {
            Some(Token::Plus) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::Plus,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::Minus) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::Minus,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::Star) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::Multiply,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::Divide) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::Divide,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::GreaterThan) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::GreaterThan,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::GreaterThanOrEqual) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::GreaterThanOrEqual,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::LessThan) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::LessThan,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::LessThanOrEqual) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::LessThanOrEqual,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::Equal) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::Equal, // Matches Token::Equal for '='.
                    right_operand: Box::new(right),
                })
            }
            Some(Token::NotEqual) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::NotEqual,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::Keyword(Keyword::And)) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::And,
                    right_operand: Box::new(right),
                })
            }
            Some(Token::Keyword(Keyword::Or)) => {
                self.advance();
                let right = self.parse_expression(precedence)?;
                Ok(Expression::BinaryOperation {
                    left_operand: Box::new(left),
                    operator: BinaryOperator::Or,
                    right_operand: Box::new(right),
                })
            }
            _ => Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof))),
        }
    }

    // Parse an expression using the Pratt algorithm, handling precedence.
    fn parse_expression(&mut self, precedence: u8) -> Result<Expression, ParseError> {
        // Start with a prefix expression (e.g., number, identifier).
        let mut left = self.parse_prefix()?;
        // Continue parsing infix operators with higher precedence.
        while self.current_token().is_some() {
            let token = self.current_token().cloned().unwrap_or(Token::Eof);
            let next_precedence = self.get_precedence(&token);
            if next_precedence <= precedence {
                break;
            }
            left = self.parse_infix(left, next_precedence)?;
        }
        // Handle ASC/DESC for ORDER BY after the full expression.
        if self.current_token() == Some(&Token::Keyword(Keyword::Asc)) {
            self.advance();
            return Ok(Expression::UnaryOperation {
                operator: UnaryOperator::Asc,
                operand: Box::new(left),
            });
        }
        if self.current_token() == Some(&Token::Keyword(Keyword::Desc)) {
            self.advance();
            return Ok(Expression::UnaryOperation {
                operator: UnaryOperator::Desc,
                operand: Box::new(left),
            });
        }
        Ok(left)
    }

    // Parse SELECT statements, including SELECT * for bonus points.
    fn parse_select(&mut self) -> Result<Statement, ParseError> {
        // Check for SELECT * (2 bonus points).
        if self.current_token() == Some(&Token::Star) {
            self.advance(); // Consume *.
            self.expect_token(Token::Keyword(Keyword::From))?;
            let from = self.parse_identifier()?;

            // Parse optional WHERE clause.
            let r#where = if self.current_token() == Some(&Token::Keyword(Keyword::Where)) {
                self.advance();
                Some(self.parse_expression(0)?)
            } else {
                None
            };

            // Parse optional ORDER BY clause.
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

            // Ensure query ends with a semicolon.
            if self.current_token() != Some(&Token::Semicolon) {
                return Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof)));
            }
            self.advance();

            // Return SelectAll for SELECT * queries.
            return Ok(Statement::SelectAll {
                from,
                r#where,
                orderby,
            });
        }

        // Parse regular SELECT with column expressions.
        let mut columns = Vec::new();
        if self.current_token() == Some(&Token::Keyword(Keyword::From)) {
            return Err(ParseError::UnexpectedToken(Token::Keyword(Keyword::From))); // Require at least one column.
        }

        // Parse comma-separated column expressions.
        loop {
            let expr = self.parse_expression(0)?;
            columns.push(expr);
            if self.current_token() == Some(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        // Expect FROM clause.
        self.expect_token(Token::Keyword(Keyword::From))?;
        let from = self.parse_identifier()?;

        // Parse optional WHERE clause.
        let r#where = if self.current_token() == Some(&Token::Keyword(Keyword::Where)) {
            self.advance();
            Some(self.parse_expression(0)?)
        } else {
            None
        };

        // Parse optional ORDER BY clause.
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

        // Ensure query ends with a semicolon.
        if self.current_token() != Some(&Token::Semicolon) {
            return Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof)));
        }
        self.advance();

        // Return Select statement for regular SELECT queries.
        Ok(Statement::Select {
            columns,
            from,
            r#where,
            orderby,
        })
    }

    // Parse CREATE TABLE statements with column types and constraints.
    fn parse_create_table(&mut self) -> Result<Statement, ParseError> {
        self.expect_token(Token::Keyword(Keyword::Table))?;
        let table_name = self.parse_identifier()?;
        self.expect_token(Token::LeftParentheses)?;

        // Parse comma-separated column definitions.
        let mut column_list = Vec::new();
        loop {
            let column_name = self.parse_identifier()?;

            // Parse column type (INT, BOOL, VARCHAR).
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
                    let len = match self.current_token() {
                        Some(Token::Number(len)) => {
                            let len_val = *len;
                            // Validate VARCHAR length (1 to 65535).
                            if len_val == 0 || len_val > 65535 {
                                return Err(ParseError::InvalidVarcharLength);
                            }
                            len_val
                        }
                        _ => return Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof))),
                    };
                    self.advance();
                    self.expect_token(Token::RightParentheses)?;
                    DBType::Varchar(len as usize)
                }
                _ => return Err(ParseError::InvalidColumnType),
            };

            // Parse optional constraints (PRIMARY KEY, NOT NULL, CHECK).
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

            // Add column to the list.
            column_list.push(TableColumn {
                column_name,
                column_type,
                constraints,
            });

            // Continue if more columns (comma); otherwise, break.
            if self.current_token() == Some(&Token::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        // Expect closing parenthesis and semicolon.
        self.expect_token(Token::RightParentheses)?;
        if self.current_token() != Some(&Token::Semicolon) {
            return Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof)));
        }
        self.advance();

        // Return CreateTable statement.
        Ok(Statement::CreateTable {
            table_name,
            column_list,
        })
    }

    // Entry point for parsing: dispatch to SELECT or CREATE TABLE.
    pub fn parse(&mut self) -> Result<Statement, ParseError> {
        match self.current_token() {
            Some(Token::Keyword(Keyword::Select)) => {
                self.advance();
                self.parse_select()
            }
            Some(Token::Keyword(Keyword::Create)) => {
                self.advance();
                self.parse_create_table()
            }
            _ => Err(ParseError::UnexpectedToken(self.current_token().cloned().unwrap_or(Token::Eof))),
        }
    }
}