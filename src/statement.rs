use std::fmt::Display;

use crate::token::Token;

fn generate_left_pad(depth: usize) -> String {
    return if depth > 0 {
        "│  ".repeat(depth - 1) + "├─ "
    } else {
        "".to_owned()
    };
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return writeln!(
            f,
            "{}",
            match self {
                Statement::Expression(ex) => ex.format(0),
                Statement::VariableDeclaration(declaration) => declaration.format(0),
            }
        );
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Literal(LiteralExpression),
    Grouping(GroupingExpression),
    Variable(VariableExpression),
}

impl Expression {
    fn format(&self, depth: usize) -> String {
        return match self {
            Expression::Binary(ex) => ex.format(depth),
            Expression::Unary(ex) => ex.format(depth),
            Expression::Literal(ex) => ex.format(depth),
            Expression::Grouping(ex) => ex.format(depth),
            Expression::Variable(ex) => ex.format(depth),
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

#[derive(Debug, Clone)]
pub struct VariableExpression {
    pub value: Token,
}

impl VariableExpression {
    pub fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        return format!("{}VAR {}", left_pad, self.value.lexeme);
    }
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub identifier: Token,
    pub initializer: Option<Expression>,
}

impl VariableDeclaration {
    pub fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);
        let children_left_pad = generate_left_pad(depth + 1);

        let initializer_value = match self.initializer {
            Some(ref initializer) => initializer.format(depth + 1),
            None => format!("{}nil", children_left_pad),
        };

        return format!(
            "{0}VAR_DECL\n{1}{2}\n{3}",
            left_pad, children_left_pad, self.identifier.lexeme, initializer_value
        );
    }
}
