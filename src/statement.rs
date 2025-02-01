use crate::token::Token;

fn generate_left_pad(depth: usize) -> String {
    if depth > 0 {
        "│  ".repeat(depth - 1) + "├─ "
    } else {
        "".to_owned()
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum Statement<'a> {
    Expression(Expression<'a>),
    VariableDeclaration(VariableDeclaration<'a>),
    Block(Vec<Statement<'a>>),
    If(IfStatement<'a>),
    While(WhileStatement<'a>),
}

impl Statement<'_> {
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
            Statement::While(while_statement) => {
                let left_pad = generate_left_pad(depth);
                let children_left_pad = generate_left_pad(depth + 1);
                let condition = while_statement.condition.format(depth + 2);
                let statements = Statement::format_block(&while_statement.statements, depth + 2);

                format!(
                    "{0}WHILE_STMT\n{1}CONDITION\n{2}\n{1}THEN\n{3}",
                    left_pad, children_left_pad, condition, statements
                )
            }
        };

        result.to_string()
    }

    fn format_block(block: &Vec<Statement>, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);
        let mut output: String = block
            .iter()
            .map(|statement| statement.format(depth + 1))
            .collect();

        // Remove trailing '\n' from the last iteration
        output.pop();
        output.pop();

        format!("{}BLOCK\n{}", left_pad, output)
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Binary(BinaryExpression<'a>),
    Unary(UnaryExpression<'a>),
    Literal(LiteralExpression<'a>),
    Grouping(GroupingExpression<'a>),
    Variable(VariableExpression<'a>),
    Assignment(AssignmentExpression<'a>),
    Logical(BinaryExpression<'a>),
}

impl Expression<'_> {
    fn format(&self, depth: usize) -> String {
        match self {
            Expression::Binary(ex) => ex.format(depth),
            Expression::Unary(ex) => ex.format(depth),
            Expression::Literal(ex) => ex.format(depth),
            Expression::Grouping(ex) => ex.format(depth),
            Expression::Variable(ex) => ex.format(depth),
            Expression::Assignment(ex) => ex.format(depth),
            Expression::Logical(ex) => ex.format(depth),
        }
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct BinaryExpression<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: Token<'a>,
    pub right: Box<Expression<'a>>,
}

impl BinaryExpression<'_> {
    fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        format!(
            "{0}{1}\n{2}\n{3}",
            left_pad,
            self.operator.lexeme,
            self.left.format(depth + 1),
            self.right.format(depth + 1)
        )
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct UnaryExpression<'a> {
    pub left: Box<Expression<'a>>,
    pub operator: Token<'a>,
}

impl UnaryExpression<'_> {
    pub fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        format!(
            "{}{}\n{}",
            left_pad,
            self.operator.lexeme,
            self.left.format(depth + 1),
        )
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct LiteralExpression<'a> {
    pub value: Token<'a>,
}

impl LiteralExpression<'_> {
    fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        format!("{}{}", left_pad, self.value.lexeme)
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct GroupingExpression<'a> {
    pub expression: Box<Expression<'a>>,
}

impl GroupingExpression<'_> {
    fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        format!("{0}GROUP\n{1}", left_pad, self.expression.format(depth + 1))
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct VariableExpression<'a> {
    pub value: Token<'a>,
}

impl VariableExpression<'_> {
    pub fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);

        format!("{}VAR {}", left_pad, self.value.lexeme)
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct VariableDeclaration<'a> {
    pub identifier: Token<'a>,
    pub initializer: Option<Expression<'a>>,
}

impl VariableDeclaration<'_> {
    pub fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);
        let children_left_pad = generate_left_pad(depth + 1);

        let initializer_value = match self.initializer {
            Some(ref initializer) => initializer.format(depth + 1),
            None => format!("{}nil", children_left_pad),
        };

        format!(
            "{0}VAR_DECL\n{1}{2}\n{3}",
            left_pad, children_left_pad, self.identifier.lexeme, initializer_value
        )
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct AssignmentExpression<'a> {
    pub name: Token<'a>,
    pub value: Box<Expression<'a>>,
}

impl AssignmentExpression<'_> {
    pub fn format(&self, depth: usize) -> String {
        let left_pad = generate_left_pad(depth);
        let children_left_pad = generate_left_pad(depth + 1);

        format!(
            "{0}VAR_ASSIGN\n{1}{2}\n{3}",
            left_pad,
            children_left_pad,
            &self.name.lexeme,
            self.value.format(depth + 1)
        )
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct IfStatement<'a> {
    pub condition: Expression<'a>,
    pub statements: Vec<Statement<'a>>,
    pub else_statements: Option<Vec<Statement<'a>>>,
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Clone)]
pub struct WhileStatement<'a> {
    pub condition: Expression<'a>,
    pub statements: Vec<Statement<'a>>,
}
