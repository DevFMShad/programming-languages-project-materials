// statement.rs - Defines the AST structures for SQL statements and expressions.
// Used by the parser to construct the parsed representation of SQL queries.
// Supports SELECT and CREATE TABLE statements, expressions, and constraints
// (Functionality #7, 2 points: AST definition).
// Includes SelectAll variant for SELECT * bonus points (2 points).

// Import fmt for displaying AST structures as strings.
use std::fmt;

// Define Expression enum for SQL expressions (e.g., numbers, strings, operations).
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(u64), // Integer values.
    String(String), // String literals.
    Bool(bool), // Boolean values.
    Identifier(String), // Column or table names.
    BinaryOperation { // Binary operations (e.g., a + b).
        left_operand: Box<Expression>,
        operator: BinaryOperator,
        right_operand: Box<Expression>,
    },
    UnaryOperation { // Unary operations (e.g., -x, ASC).
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
}

// Define BinaryOperator enum for binary operations.
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Plus, // Addition (+).
    Minus, // Subtraction (-).
    Multiply, // Multiplication (*).
    Divide, // Division (/).
    Equal, // Equality (=), matches Token::Equal.
    NotEqual, // Inequality (!=).
    GreaterThan, // Greater than (>).
    GreaterThanOrEqual, // Greater than or equal (>=).
    LessThan, // Less than (<).
    LessThanOrEqual, // Less than or equal (<=).
    And, // Logical AND.
    Or, // Logical OR.
}

// Define UnaryOperator enum for unary operations.
#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Plus, // Unary plus (+).
    Minus, // Unary minus (-).
    Not, // Logical NOT.
    Asc, // Ascending order for ORDER BY.
    Desc, // Descending order for ORDER BY.
}

// Define DBType enum for column data types.
#[derive(Debug, PartialEq, Clone)]
pub enum DBType {
    Int, // Integer type.
    Bool, // Boolean type.
    Varchar(usize), // Variable-length string with length.
}

// Define Constraint enum for column constraints.
#[derive(Debug, PartialEq, Clone)]
pub enum Constraint {
    PrimaryKey, // Primary key constraint.
    NotNull, // Not null constraint.
    Check(Expression), // Check constraint with expression.
}

// Define TableColumn struct for CREATE TABLE columns.
#[derive(Debug, PartialEq, Clone)]
pub struct TableColumn {
    pub column_name: String, // Column name.
    pub column_type: DBType, // Column data type.
    pub constraints: Vec<Constraint>, // List of constraints.
}

// Define Statement enum for SQL statements.
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Select { // SELECT with specific columns.
        columns: Vec<Expression>, // Selected columns or expressions.
        from: String, // Table name.
        r#where: Option<Expression>, // Optional WHERE condition.
        orderby: Vec<Expression>, // Optional ORDER BY expressions.
    },
    SelectAll { // SELECT * (2 bonus points).
        from: String, // Table name.
        r#where: Option<Expression>, // Optional WHERE condition.
        orderby: Vec<Expression>, // Optional ORDER BY expressions.
    },
    CreateTable { // CREATE TABLE statement.
        table_name: String, // Table name.
        column_list: Vec<TableColumn>, // List of columns.
    },
}

// Implement Display for Expression to format as SQL-like string.
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::Bool(b) => write!(f, "{}", b),
            Expression::Identifier(id) => write!(f, "{}", id),
            Expression::BinaryOperation { left_operand, operator, right_operand } => {
                write!(f, "({} {} {})", left_operand, operator, right_operand) // Parentheses for clarity.
            }
            Expression::UnaryOperation { operator, operand } => {
                write!(f, "{} {}", operator, operand)
            }
        }
    }
}

// Implement Display for BinaryOperator to show operator symbols.
impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::Minus => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Equal => write!(f, "="), // Matches Token::Equal.
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::And => write!(f, "AND"),
            BinaryOperator::Or => write!(f, "OR"),
        }
    }
}

// Implement Display for UnaryOperator to show operator symbols.
impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Not => write!(f, "NOT"),
            UnaryOperator::Asc => write!(f, "ASC"),
            UnaryOperator::Desc => write!(f, "DESC"),
        }
    }
}

// Implement Display for DBType to show SQL type syntax.
impl fmt::Display for DBType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBType::Int => write!(f, "INT"),
            DBType::Bool => write!(f, "BOOL"),
            DBType::Varchar(len) => write!(f, "VARCHAR({})", len),
        }
    }
}

// Implement Display for Constraint to show SQL constraint syntax.
impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constraint::PrimaryKey => write!(f, "PRIMARY KEY"),
            Constraint::NotNull => write!(f, "NOT NULL"),
            Constraint::Check(expr) => write!(f, "CHECK ({})", expr),
        }
    }
}

// Implement Display for TableColumn to show column definition.
impl fmt::Display for TableColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.column_name, self.column_type)?;
        for constraint in &self.constraints {
            write!(f, " {}", constraint)?;
        }
        Ok(())
    }
}

// Implement Display for Statement to format as SQL query.
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Select { columns, from, r#where, orderby } => {
                write!(f, "SELECT ")?;
                for (i, col) in columns.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", col)?;
                }
                write!(f, " FROM {}", from)?;
                if let Some(w) = r#where {
                    write!(f, " WHERE {}", w)?;
                }
                if !orderby.is_empty() {
                    write!(f, " ORDER BY ")?;
                    for (i, ord) in orderby.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", ord)?;
                    }
                }
                Ok(())
            }
            Statement::SelectAll { from, r#where, orderby } => {
                // Format SELECT * queries (2 bonus points).
                write!(f, "SELECT * FROM {}", from)?;
                if let Some(w) = r#where {
                    write!(f, " WHERE {}", w)?;
                }
                if !orderby.is_empty() {
                    write!(f, " ORDER BY ")?;
                    for (i, ord) in orderby.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", ord)?;
                    }
                }
                Ok(())
            }
            Statement::CreateTable { table_name, column_list } => {
                write!(f, "CREATE TABLE {} (", table_name)?;
                for (i, col) in column_list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", col)?;
                }
                write!(f, ")")
            }
        }
    }
}