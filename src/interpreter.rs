use crate::{
    environment::Environment,
    matcha::{Literal, NumberLiteral, Value},
    statement::{
        BinaryExpression, Expression, GroupingExpression, LiteralExpression, Statement,
        UnaryExpression, VariableDeclaration, VariableExpression,
    },
    token::TokenType,
};

const NULLABLE_VALUE_OPERATION_ERROR_MESSAGE: &str =
    "Cannot execute an operation in an optional value. Try unwrapping it first";
const EMPTY_VALUE_OPERATION_ERROR_MESSAGE: &str =
    "Cannot execute a unary operation in an empty value";

#[derive(Debug)]
pub struct InterpreterError {
    pub message: String,
    pub statement: Statement,
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(
        environment: &mut Environment,
        statements: &Vec<Statement>,
    ) -> Result<Value, InterpreterError> {
        for i in 0..statements.len() {
            // Return last value
            if i == statements.len() - 1 {
                return Ok(Interpreter::evaluate(environment, &statements[i])?);
            }

            Interpreter::evaluate(environment, &statements[i])?;
        }

        return Ok(Value::Empty);
    }

    fn evaluate(
        environment: &mut Environment,
        statement: &Statement,
    ) -> Result<Value, InterpreterError> {
        return match statement {
            Statement::VariableDeclaration(decl) => {
                let _ = Interpreter::variable_declaration(environment, decl)?;
                return Ok(Value::Empty);
            }
            Statement::Expression(expression) => Interpreter::expression(environment, expression),
            Statement::Block(block) => Interpreter::block(environment, block),
        };
    }

    fn expression(
        environment: &Environment,
        expression: &Expression,
    ) -> Result<Value, InterpreterError> {
        return match expression {
            Expression::Literal(literal) => Interpreter::literal(literal),
            Expression::Unary(unary) => Interpreter::unary(environment, unary),
            Expression::Grouping(grouping) => Interpreter::grouping(environment, grouping),
            Expression::Binary(binary) => Interpreter::binary(environment, binary),
            Expression::Variable(variable) => {
                Interpreter::variable_expression(environment, variable)
            }
        };
    }

    fn literal(literal: &LiteralExpression) -> Result<Value, InterpreterError> {
        let value = &literal.value.literal;
        return match value {
            Some(value) => Ok(Value::Literal(value.clone())),
            None => Err(InterpreterError {
                message: "Literal expression value is None. This should never be the case."
                    .to_owned(),
                statement: Statement::Expression(Expression::Literal(literal.clone())),
            }),
        };
    }

    fn grouping(
        environment: &Environment,
        grouping: &GroupingExpression,
    ) -> Result<Value, InterpreterError> {
        return Interpreter::expression(environment, &grouping.expression);
    }

    fn unary(
        environment: &Environment,
        unary: &UnaryExpression,
    ) -> Result<Value, InterpreterError> {
        let value = match Interpreter::expression(environment, &unary.left) {
            Ok(value) => match value {
                Value::Empty => Err(InterpreterError {
                    message: EMPTY_VALUE_OPERATION_ERROR_MESSAGE.to_owned(),
                    statement: Statement::Expression(Expression::Unary(unary.clone())),
                }),
                Value::Optional(_) => Err(InterpreterError {
                    message: NULLABLE_VALUE_OPERATION_ERROR_MESSAGE.to_owned(),
                    statement: Statement::Expression(Expression::Unary(unary.clone())),
                }),
                Value::Literal(literal) => Ok(literal),
            },
            Err(e) => Err(e),
        }?;

        match unary.operator.token_type {
            TokenType::Minus => match value {
                Literal::Number(number) => match number {
                    NumberLiteral::Integer(integer) => {
                        return Ok(Value::Literal(Literal::Number(NumberLiteral::Integer(
                            -integer,
                        ))))
                    }
                    NumberLiteral::Float(float) => {
                        return Ok(Value::Literal(Literal::Number(NumberLiteral::Float(
                            -float,
                        ))))
                    }
                },
                _ => {
                    return Err(InterpreterError {
                        message: "Cannot use operator \"-\" on non-numeric value".to_owned(),
                        statement: Statement::Expression(Expression::Unary(unary.clone())),
                    })
                }
            },
            TokenType::Bang => match value {
                Literal::Boolean(bool) => return Ok(Value::Literal(Literal::Boolean(!bool))),
                _ => {
                    return Err(InterpreterError {
                        message: "Cannot negate non-boolean value".to_owned(),
                        statement: Statement::Expression(Expression::Unary(unary.clone())),
                    })
                }
            },
            _ => {
                return Err(InterpreterError {
                    message: format!(
                        "Unexpected unary operator. {} is not a valid unary operator",
                        &unary.operator.lexeme
                    ),
                    statement: Statement::Expression(Expression::Unary(unary.clone())),
                })
            }
        }
    }

    fn binary(
        environment: &Environment,
        binary: &BinaryExpression,
    ) -> Result<Value, InterpreterError> {
        let left_value = Interpreter::expression(environment, &binary.left)?;
        let right_value = Interpreter::expression(environment, &binary.right)?;

        match binary.operator.token_type {
            TokenType::Plus => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                return Ok(Value::Literal(Literal::Number(left + right)));
            }
            TokenType::Minus => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                return Ok(Value::Literal(Literal::Number(left - right)));
            }
            TokenType::Star => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                return Ok(Value::Literal(Literal::Number(left * right)));
            }
            TokenType::Slash => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                return Ok(Value::Literal(Literal::Number(left / right)));
            }
            _ => todo!(),
        }
    }

    fn unwrap_number(
        value: Value,
        binary: &BinaryExpression,
    ) -> Result<NumberLiteral, InterpreterError> {
        match value {
            Value::Literal(literal) => match literal {
                Literal::Number(number) => Ok(number),
                Literal::String(_) => Err(InterpreterError {
                    message: "Expected number, got string".to_owned(),
                    statement: Statement::Expression(Expression::Binary(binary.clone())),
                }),
                Literal::Boolean(_) => Err(InterpreterError {
                    message: "Expected number, got boolean".to_owned(),
                    statement: Statement::Expression(Expression::Binary(binary.clone())),
                }),
            },
            Value::Empty => Err(InterpreterError {
                message: EMPTY_VALUE_OPERATION_ERROR_MESSAGE.to_owned(),
                statement: Statement::Expression(Expression::Binary(binary.clone())),
            }),
            Value::Optional(_) => Err(InterpreterError {
                message: NULLABLE_VALUE_OPERATION_ERROR_MESSAGE.to_owned(),
                statement: Statement::Expression(Expression::Binary(binary.clone())),
            }),
        }
    }

    fn variable_declaration(
        environment: &mut Environment,
        decl: &VariableDeclaration,
    ) -> Result<(), InterpreterError> {
        let result = environment.values.insert(
            decl.identifier.lexeme.to_owned(),
            match decl.initializer {
                Some(ref initializer) => match initializer {
                    Expression::Literal(literal_expression) => {
                        match literal_expression.value.literal {
                            Some(ref literal) => Value::Literal(literal.clone()),
                            None => Value::Optional(None),
                        }
                    }
                    _ => Interpreter::expression(environment, &initializer)?,
                },
                None => Value::Empty,
            },
        );

        if result.is_some() {
            return Err(InterpreterError {
                statement: Statement::VariableDeclaration(decl.to_owned()),
                message: format!(
                    "Variable '{}' already declared in this scope",
                    decl.identifier.lexeme
                ),
            });
        }

        return Ok(());
    }

    fn variable_expression(
        environment: &Environment,
        variable: &VariableExpression,
    ) -> Result<Value, InterpreterError> {
        return match environment.values.get(&variable.value.lexeme) {
            Some(value) => Ok(value.clone()),
            None => match environment.parent {
                Some(ref parent) => Interpreter::variable_expression(parent, variable),
                None => Err(InterpreterError {
                    statement: Statement::Expression(Expression::Variable(variable.clone())),
                    message: format!(
                        "Variable '{}' not found in the current scope",
                        variable.value.lexeme
                    ),
                }),
            },
        };
    }

    fn block(
        environment: &mut Environment,
        statements: &Vec<Statement>,
    ) -> Result<Value, InterpreterError> {
        let mut inner_environment = Environment::with_parent(environment);

        return Interpreter::interpret(&mut inner_environment, statements);
    }
}
