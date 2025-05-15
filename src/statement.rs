use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Select {
        columns: Vec<Expression>,
        from: String,
        r#where: Option<Expression>,
        orderby: Vec<Expression>,
    },
    CreateTable {
        table_name: String,
        column_list: Vec<TableColumn>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryOperation {
        left_operand: Box<Expression>,
        operator: BinaryOperator,
        right_operand: Box<Expression>,
    },
    UnaryOperation {
        operand: Box<Expression>,
        operator: UnaryOperator,
    },
    Number(u64),
    Bool(bool),
    Identifier(String),
    String(String),
}

#[derive(Debug, PartialEq)]
pub struct TableColumn {
    pub column_name: String,
    pub column_type: DBType,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, PartialEq)]
pub enum DBType {
    Int,
    Varchar(usize),
    Bool,
}

#[derive(Debug, PartialEq)]
pub enum Constraint {
    NotNull,
    PrimaryKey,
    Check(Expression),
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Plus,
    Minus,
    Asc,
    Desc,
}

impl Display for UnaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Plus => write!(f, "+"),
            UnaryOperator::Desc => write!(f, "DESC"),
            UnaryOperator::Asc => write!(f, "ASC"),
            UnaryOperator::Not => write!(f, "NOT"),
        }
    }
}

impl Display for BinaryOperator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::Equal => write!(f, "="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Minus => write!(f, "-"),
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::And => write!(f, "AND"),
            BinaryOperator::Or => write!(f, "OR"),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::BinaryOperation { left_operand, operator, right_operand } => {
                write!(f, "({} {} {})", left_operand, operator, right_operand)
            }
            Expression::UnaryOperation { operand, operator } => {
                write!(f, "{} {}", operator, operand)
            }
            Expression::Number(num) => write!(f, "{}", num),
            Expression::Identifier(iden) => write!(f, "{}", iden),
            Expression::String(s) => write!(f, "\"{}\"", s),
            Expression::Bool(b) => write!(f, "{}", if *b { "TRUE" } else { "FALSE" }),
        }
    }
}