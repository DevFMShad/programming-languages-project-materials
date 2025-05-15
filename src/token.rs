// token.rs - Defines token types for the SQL tokenizer.
// Represents lexical units (e.g., keywords, identifiers, operators) used by the tokenizer and parser.
// Supports Functionality #1–6 (7 points): tokenizing single/multi-char tokens, keywords, identifiers, numbers, strings.
// Used by tokenizer.rs to produce tokens and parser.rs to parse them.

// Import fmt for displaying tokens as strings.
use std::fmt::{Debug, Display, Formatter};

// Define Token enum for all possible token types.
#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Token {
    Keyword(Keyword), // SQL keywords (e.g., SELECT).
    Identifier(String), // User-defined names (e.g., table names).
    String(String), // String literals (e.g., "Voldemort").
    Number(u64), // Numeric literals (e.g., 42).
    Invalid(char), // Invalid characters for error handling.
    RightParentheses, // ).
    LeftParentheses, // (.
    GreaterThan, // >.
    GreaterThanOrEqual, // >=.
    LessThan, // <.
    LessThanOrEqual, // <=.
    Equal, // =, matches BinaryOperator::Equal.
    NotEqual, // !=.
    Star, // * (for SELECT * and multiplication).
    Divide, // /.
    Minus, // -.
    Plus, // +.
    Comma, // ,.
    Semicolon, // ;.
    Eof, // End of input.
}

// Define Keyword enum for SQL reserved words.
#[derive(PartialEq, Clone, Debug)]
pub(crate) enum Keyword {
    Select, // SELECT for queries.
    Create, // CREATE for table creation.
    Table, // TABLE for CREATE TABLE.
    Where, // WHERE for conditions.
    Order, // ORDER for ORDER BY.
    By, // BY for ORDER BY.
    Asc, // ASC for sorting.
    Desc, // DESC for sorting.
    From, // FROM for table selection.
    And, // AND for logical operations.
    Or, // OR for logical operations.
    Not, // NOT for negation.
    True, // TRUE for boolean.
    False, // FALSE for boolean.
    Primary, // PRIMARY for constraints.
    Key, // KEY for constraints.
    Check, // CHECK for constraints.
    Int, // INT for column type.
    Bool, // BOOL for column type.
    Varchar, // VARCHAR for column type.
    Null, // NULL for constraints.
}

// Implement Display for Token to format tokens as strings.
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Keyword(keyword) => write!(f, "{}", keyword),
            Token::Identifier(iden) => write!(f, "{}", iden),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Number(num) => write!(f, "{}", num),
            Token::RightParentheses => write!(f, ")"),
            Token::LeftParentheses => write!(f, "("),
            Token::GreaterThan => write!(f, ">"),
            Token::GreaterThanOrEqual => write!(f, ">="),
            Token::LessThan => write!(f, "<"),
            Token::LessThanOrEqual => write!(f, "<="),
            Token::Equal => write!(f, "="), // Matches BinaryOperator::Equal.
            Token::NotEqual => write!(f, "!="),
            Token::Star => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Minus => write!(f, "-"),
            Token::Plus => write!(f, "+"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Eof => write!(f, "EOF"),
            Token::Invalid(c) => write!(f, "Invalid({})", c),
        }
    }
}

// Implement Display for Keyword to format keywords as SQL syntax.
impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Select => write!(f, "SELECT"),
            Keyword::Create => write!(f, "CREATE"),
            Keyword::Table => write!(f, "TABLE"),
            Keyword::Where => write!(f, "WHERE"),
            Keyword::Order => write!(f, "ORDER"),
            Keyword::By => write!(f, "BY"),
            Keyword::Asc => write!(f, "ASC"),
            Keyword::Desc => write!(f, "DESC"),
            Keyword::From => write!(f, "FROM"),
            Keyword::And => write!(f, "AND"),
            Keyword::Or => write!(f, "OR"),
            Keyword::Not => write!(f, "NOT"),
            Keyword::True => write!(f, "TRUE"),
            Keyword::False => write!(f, "FALSE"),
            Keyword::Primary => write!(f, "PRIMARY"),
            Keyword::Key => write!(f, "KEY"),
            Keyword::Check => write!(f, "CHECK"),
            Keyword::Int => write!(f, "INT"),
            Keyword::Bool => write!(f, "BOOL"),
            Keyword::Varchar => write!(f, "VARCHAR"),
            Keyword::Null => write!(f, "NULL"),
        }
    }
}