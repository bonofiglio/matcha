use crate::{matcha::Value, token::Token};

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
    Block(Vec<Statement>),
    If(IfStatement),
}

impl Statement {
    pub fn format(&self, depth: usize) -> String {
        let result = match self {
            Statement::Expression(ex) => ex.format(depth),
            Statement::VariableDeclaration(declaration) => declaration.format(depth),
            Statement::Block(block) => Statement::format_block(block, depth),
            Statement::If(if_statement) => {
                let left_pad = generate_left_pad(depth);
                let children_left_pad = generate_left_pad(depth + 1);
                let condition = if_statement.condition.format(depth + 2);
                let statements = Statement::format_block(&if_statement.statements, depth + 2);
                let else_block = match if_statement.else_statements {
                    Some(ref block) => format!(
                        "\n{}ELSE\n{}",
                        children_left_pad,
                        Statement::format_block(block, depth + 2)
                    ),
                    None => "".to_owned(),
                };

                format!(
                    "{0}IF_STMT\n{1}CONDITION\n{2}\n{1}THEN\n{3}{4}",
                    left_pad, children_left_pad, condition, statements, else_block
                )
            }
        };

        return format!("{}", result);
    }

    fn format_block(block: &Vec<Statement>, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);
        let mut output: String = block
            .iter()
            .map(|statement| format!("{}\n", statement.format(depth + 1)))
            .collect();

        // Remove trailing '\n' from the last iteration
        output.pop();
        output.pop();

        format!("{}BLOCK\n{}", left_pad, output)
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpression),
    Unary(UnaryExpression),
    Literal(LiteralExpression),
    Grouping(GroupingExpression),
    Variable(VariableExpression),
    Assignment(AssignmentExpression),
}

impl Expression {
    fn format(&self, depth: usize) -> String {
        return match self {
            Expression::Binary(ex) => ex.format(depth),
            Expression::Unary(ex) => ex.format(depth),
            Expression::Literal(ex) => ex.format(depth),
            Expression::Grouping(ex) => ex.format(depth),
            Expression::Variable(ex) => ex.format(depth),
            Expression::Assignment(ex) => ex.format(depth),
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

        return format!("{0}GROUP\n{1}", left_pad, self.expression.format(depth + 1));
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

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    pub name: Token,
    pub value: Box<Expression>,
}

impl AssignmentExpression {
    pub fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);
        let children_left_pad = generate_left_pad(depth + 1);

        return format!(
            "{0}VAR_ASSIGN\n{1}{2}\n{3}",
            left_pad,
            children_left_pad,
            &self.name.lexeme,
            self.value.format(depth + 1)
        );
    }
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub statements: Vec<Statement>,
    pub else_statements: Option<Vec<Statement>>,
}
