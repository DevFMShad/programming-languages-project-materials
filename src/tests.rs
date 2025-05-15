// tests.rs - Automated unit tests for the SQL Parser project.
// Provides comprehensive tests for the Tokenizer, Pratt Parser, and SQL Parser to validate
// correct tokenization, expression parsing, and SQL statement parsing.
// Covers all required functionalities to earn up to 3 bonus points for unit testing:
// - Tokenizer: Tests single/multi-character tokens, numbers, strings, keywords, identifiers.
// - Pratt Parser: Tests precedence, parentheses, unary/binary operations.
// - SQL Parser: Tests SELECT (with/without WHERE/ORDER BY), SELECT *, and CREATE TABLE (with types/constraints).
// Includes test for SELECT * to support bonus points (2 points).

// Define test module to run tests only when testing.
#[cfg(test)]
mod tests {
    // Import necessary types and modules for testing.
    use crate::parser::Parser;
    use crate::statement::{Statement, Expression, BinaryOperator, UnaryOperator, TableColumn, DBType, Constraint};
    use crate::tokenizer::{Tokenizer, TokenizerError};
    use crate::token::{Token, Keyword};

    // Test single-character tokens (e.g., (, ), +) for tokenizer.
    #[test]
    fn test_tokenizer_single_char_tokens() {
        let input = "( ) , ; + - * / = > <";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LeftParentheses,
                Token::RightParentheses,
                Token::Comma,
                Token::Semicolon,
                Token::Plus,
                Token::Minus,
                Token::Star,
                Token::Divide,
                Token::Equal,
                Token::GreaterThan,
                Token::LessThan,
                Token::Eof,
            ]
        );
    }

    // Test multi-character tokens (e.g., >=, <=, !=) for tokenizer.
    #[test]
    fn test_tokenizer_multi_char_tokens() {
        let input = ">= <= !=";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::GreaterThanOrEqual,
                Token::LessThanOrEqual,
                Token::NotEqual,
                Token::Eof,
            ]
        );
    }

    // Test number tokenization (e.g., 42, 123).
    #[test]
    fn test_tokenizer_numbers() {
        let input = "42 123";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(42),
                Token::Number(123),
                Token::Eof,
            ]
        );
    }

    // Test string literal tokenization (e.g., "hello").
    #[test]
    fn test_tokenizer_strings() {
        let input = r#""hello" 'world'"#;
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::String("hello".to_string()),
                Token::String("world".to_string()),
                Token::Eof,
            ]
        );
    }

    // Test keyword and identifier tokenization (e.g., SELECT, users).
    #[test]
    fn test_tokenizer_keywords_and_identifiers() {
        let input = "SELECT FROM users age";
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Keyword(Keyword::Select),
                Token::Keyword(Keyword::From),
                Token::Identifier("users".to_string()),
                Token::Identifier("age".to_string()),
                Token::Eof,
            ]
        );
    }

    // Test error handling for unterminated strings.
    #[test]
    fn test_tokenizer_error_unterminated_string() {
        let input = r#""hello"#;
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert!(matches!(result, Err(TokenizerError::UnterminatedString)));
    }

    // Test error handling for invalid characters.
    #[test]
    fn test_tokenizer_error_invalid_char() {
        let input = "#";
        let mut tokenizer = Tokenizer::new(input);
        let result = tokenizer.tokenize();
        assert!(matches!(result, Err(TokenizerError::UnexpectedChar('#'))));
    }

    // Test Pratt parser precedence (e.g., 2 * 3 evaluated before +).
    #[test]
    fn test_pratt_parser_precedence() {
        let input = "1 + 2 * 3;";
        let mut parser = Parser::new(input);
        let statement = parser.parse().unwrap();
        assert_eq!(
            statement,
            Statement::Select {
                columns: vec![
                    Expression::BinaryOperation {
                        left_operand: Box::new(Expression::Number(1)),
                        operator: BinaryOperator::Plus,
                        right_operand: Box::new(Expression::BinaryOperation {
                            left_operand: Box::new(Expression::Number(2)),
                            operator: BinaryOperator::Multiply,
                            right_operand: Box::new(Expression::Number(3)),
                        }),
                    }
                ],
                from: "".to_string(), // Empty table name (simplified test).
                r#where: None,
                orderby: vec![],
            }
        );
    }

    // Test Pratt parser handling of parentheses.
    #[test]
    fn test_pratt_parser_parentheses() {
        let input = "(1 + 2) * 3;";
        let mut parser = Parser::new(input);
        let statement = parser.parse().unwrap();
        assert_eq!(
            statement,
            Statement::Select {
                columns: vec![
                    Expression::BinaryOperation {
                        left_operand: Box::new(Expression::BinaryOperation {
                            left_operand: Box::new(Expression::Number(1)),
                            operator: BinaryOperator::Plus,
                            right_operand: Box::new(Expression::Number(2)),
                        }),
                        operator: BinaryOperator::Multiply,
                        right_operand: Box::new(Expression::Number(3)),
                    }
                ],
                from: "".to_string(),
                r#where: None,
                orderby: vec![],
            }
        );
    }

    // Test Pratt parser handling of unary operations.
    #[test]
    fn test_pratt_parser_unary_operation() {
        let input = "-5 + 6;";
        let mut parser = Parser::new(input);
        let statement = parser.parse().unwrap();
        assert_eq!(
            statement,
            Statement::Select {
                columns: vec![
                    Expression::BinaryOperation {
                        left_operand: Box::new(Expression::UnaryOperation {
                            operand: Box::new(Expression::Number(5)),
                            operator: UnaryOperator::Minus,
                        }),
                        operator: BinaryOperator::Plus,
                        right_operand: Box::new(Expression::Number(6)),
                    }
                ],
                from: "".to_string(),
                r#where: None,
                orderby: vec![],
            }
        );
    }

    // Test Pratt parser error handling for invalid tokens.
    #[test]
    fn test_pratt_parser_error_invalid_token() {
        let input = "1 + #;";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(matches!(result, Err(_))); // Error depends on parser state.
    }

    // Test simple SELECT statement parsing.
    #[test]
    fn test_sql_parser_select_simple() {
        let input = "SELECT id, name FROM users;";
        let mut parser = Parser::new(input);
        let statement = parser.parse().unwrap();
        assert_eq!(
            statement,
            Statement::Select {
                columns: vec![
                    Expression::Identifier("id".to_string()),
                    Expression::Identifier("name".to_string()),
                ],
                from: "users".to_string(),
                r#where: None,
                orderby: vec![],
            }
        );
    }

    // Test SELECT with WHERE and ORDER BY clauses.
    #[test]
    fn test_sql_parser_select_with_where_order_by() {
        let input = "SELECT name FROM users WHERE age > 18 ORDER BY name ASC;";
        let mut parser = Parser::new(input);
        let statement = parser.parse().unwrap();
        assert_eq!(
            statement,
            Statement::Select {
                columns: vec![Expression::Identifier("name".to_string())],
                from: "users".to_string(),
                r#where: Some(Expression::BinaryOperation {
                    left_operand: Box::new(Expression::Identifier("age".to_string())),
                    operator: BinaryOperator::GreaterThan,
                    right_operand: Box::new(Expression::Number(18)),
                }),
                orderby: vec![
                    Expression::UnaryOperation {
                        operand: Box::new(Expression::Identifier("name".to_string())),
                        operator: UnaryOperator::Asc,
                    }
                ],
            }
        );
    }

    // Test CREATE TABLE with types and constraints.
    #[test]
    fn test_sql_parser_create_table() {
        let input = "CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(50) NOT NULL, age INT CHECK (age > 0));";
        let mut parser = Parser::new(input);
        let statement = parser.parse().unwrap();
        assert_eq!(
            statement,
            Statement::CreateTable {
                table_name: "users".to_string(),
                column_list: vec![
                    TableColumn {
                        column_name: "id".to_string(),
                        column_type: DBType::Int,
                        constraints: vec![Constraint::PrimaryKey],
                    },
                    TableColumn {
                        column_name: "name".to_string(),
                        column_type: DBType::Varchar(50),
                        constraints: vec![Constraint::NotNull],
                    },
                    TableColumn {
                        column_name: "age".to_string(),
                        column_type: DBType::Int,
                        constraints: vec![Constraint::Check(
                            Expression::BinaryOperation {
                                left_operand: Box::new(Expression::Identifier("age".to_string())),
                                operator: BinaryOperator::GreaterThan,
                                right_operand: Box::new(Expression::Number(0)),
                            }
                        )],
                    },
                ],
            }
        );
    }

    // Test error handling for SELECT without FROM.
    #[test]
    fn test_sql_parser_select_error_no_from() {
        let input = "SELECT name;";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(matches!(result, Err(_)));
    }

    // Test error handling for invalid column type in CREATE TABLE.
    #[test]
    fn test_sql_parser_create_table_error_invalid_type() {
        let input = "CREATE TABLE users (id INVALID);";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(matches!(result, Err(_)));
    }

    // Test SELECT * parsing (2 bonus points).
    #[test]
    fn test_sql_parser_select_star() {
        let input = "SELECT * FROM users WHERE age > 18;";
        let mut parser = Parser::new(input);
        let statement = parser.parse().unwrap();
        assert_eq!(
            statement,
            Statement::SelectAll {
                from: "users".to_string(),
                r#where: Some(Expression::BinaryOperation {
                    left_operand: Box::new(Expression::Identifier("age".to_string())),
                    operator: BinaryOperator::GreaterThan,
                    right_operand: Box::new(Expression::Number(18)),
                }),
                orderby: vec![],
            }
        );
    }
}