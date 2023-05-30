use std::{fmt::Display, write};

use crate::{parser::Parser, token::Token};

fn generate_left_pad(depth: usize) -> String {
    return if depth > 0 {
        "│  ".repeat(depth - 1) + "├─ "
    } else {
        "".to_owned()
    };
}

#[derive(Debug)]
pub enum Expression<'a> {
    Binary(BinaryExpression<'a>),
    Unary(UnaryExpression<'a>),
    Literal(LiteralExpression<'a>),
    Grouping(GroupingExpression<'a>),
}

impl<'a> Expression<'a> {
    fn format(&self, depth: usize) -> String {
        return match self {
            Expression::Binary(ex) => ex.format(depth),
            Expression::Unary(ex) => ex.format(depth),
            Expression::Literal(ex) => ex.format(depth),
            Expression::Grouping(ex) => ex.format(depth),
        };
    }
}

#[derive(Debug)]
pub struct BinaryExpression<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: &'a Token,
    pub right: Box<Expression<'a>>,
}

impl<'a> BinaryExpression<'a> {
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

#[derive(Debug)]
pub struct UnaryExpression<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: &'a Token,
}

impl<'a> UnaryExpression<'a> {
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

#[derive(Debug)]
pub struct LiteralExpression<'a> {
    pub value: &'a Token,
}

impl<'a> LiteralExpression<'a> {
    fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        return format!("{}{}", left_pad, self.value.lexeme);
    }
}

#[derive(Debug)]
pub struct GroupingExpression<'a> {
    pub expression: Box<Expression<'a>>,
}

impl<'a> GroupingExpression<'a> {
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
pub struct AST<'a> {
    root: Expression<'a>,
}

impl<'a> AST<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> AST {
        let mut current_index = 0;
        let root = Parser::expression(tokens, &mut current_index);

        return AST { root };
    }
}

impl<'a> Display for AST<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root.format(0))
    }
}
