use std::{fmt::Display, write};

use crate::{
    parser::{Parser, ParserError},
    token::Token,
};

fn generate_left_pad(depth: usize) -> String {
    return if depth > 0 {
        "│  ".repeat(depth - 1) + "├─ "
    } else {
        "".to_owned()
    };
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Literal(LiteralExpression),
    Grouping(GroupingExpression),
}

impl Expression {
    fn format(&self, depth: usize) -> String {
        return match self {
            Expression::Binary(ex) => ex.format(depth),
            Expression::Unary(ex) => ex.format(depth),
            Expression::Literal(ex) => ex.format(depth),
            Expression::Grouping(ex) => ex.format(depth),
        };
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        return format!(
            "{0}{1}\n{2}\n{3}",
            left_pad,
            self.operator.lexeme,
            self.left.format(depth + 1),
            self.right.format(depth + 1)
        );
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
}

impl UnaryExpression {
    pub fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        return format!(
            "{}{}\n{}",
            left_pad,
            self.operator.lexeme,
            self.left.format(depth + 1),
        );
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpression {
    pub value: Token,
}

impl LiteralExpression {
    fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        return format!("{}{}", left_pad, self.value.lexeme);
    }
}

#[derive(Debug, Clone)]
pub struct GroupingExpression {
    pub expression: Box<Expression>,
}

impl GroupingExpression {
    fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        return format!(
            "{0}(\n{1}\n{0})",
            left_pad,
            self.expression.format(depth + 1)
        );
    }
}

#[derive(Debug)]
pub struct AST {
    pub root: Expression,
}

impl AST {
    pub fn new(parser: &mut Parser) -> Result<AST, Vec<ParserError>> {
        let root = parser.parse()?;

        return Ok(AST { root });
    }
}

impl Display for AST {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root.format(0))
    }
}
