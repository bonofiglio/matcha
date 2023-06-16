use crate::{
    ast::{
        BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression, AST,
    },
    token::TokenType,
    vitus::{Literal, NumberLiteral, Value},
};

const NULLABLE_VALUE_OPERATION_ERROR_MESSAGE: &str =
    "Cannot execute an operation in an optional value. Try unwrapping it first";
const EMPTY_VALUE_OPERATION_ERROR_MESSAGE: &str =
    "Cannot execute a unary operation in an empty value";

#[derive(Debug)]
pub struct InterpreterError {
    pub message: String,
    pub expression: Expression,
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(ast: AST) -> Result<Value, InterpreterError> {
        return Interpreter::evaluate(&ast.root);
    }

    fn evaluate(expression: &Expression) -> Result<Value, InterpreterError> {
        return match expression {
            Expression::Literal(literal) => Interpreter::literal(literal),
            Expression::Unary(unary) => Interpreter::unary(unary),
            Expression::Grouping(grouping) => Interpreter::grouping(grouping),
            Expression::Binary(binary) => Interpreter::binary(binary),
        };
    }

    fn literal(literal: &LiteralExpression) -> Result<Value, InterpreterError> {
        let value = &literal.value.literal;
        assert!(
            value.is_some(),
            "Literal expression value is None. This should never be the case."
        );

        return Ok(Value::Literal(value.clone().unwrap()));
    }

    fn grouping(grouping: &GroupingExpression) -> Result<Value, InterpreterError> {
        return Interpreter::evaluate(&grouping.expression);
    }

    fn unary(unary: &UnaryExpression) -> Result<Value, InterpreterError> {
        let value = match Interpreter::evaluate(&unary.left) {
            Ok(value) => match value {
                Value::Empty => Err(InterpreterError {
                    message: EMPTY_VALUE_OPERATION_ERROR_MESSAGE.to_owned(),
                    expression: Expression::Unary(unary.clone()),
                }),
                Value::Optional(_) => Err(InterpreterError {
                    message: NULLABLE_VALUE_OPERATION_ERROR_MESSAGE.to_owned(),
                    expression: Expression::Unary(unary.clone()),
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
                        expression: Expression::Unary(unary.clone()),
                    })
                }
            },
            TokenType::Bang => match value {
                Literal::Boolean(bool) => return Ok(Value::Literal(Literal::Boolean(!bool))),
                _ => {
                    return Err(InterpreterError {
                        message: "Cannot negate non-boolean value".to_owned(),
                        expression: Expression::Unary(unary.clone()),
                    })
                }
            },
            _ => {
                return Err(InterpreterError {
                    message: format!(
                        "Unexpected unary operator. {} is not a valid unary operator",
                        &unary.operator.lexeme
                    ),
                    expression: Expression::Unary(unary.clone()),
                })
            }
        }
    }

    fn binary(binary: &BinaryExpression) -> Result<Value, InterpreterError> {
        let left_value = Interpreter::evaluate(&binary.left)?;
        let right_value = Interpreter::evaluate(&binary.right)?;

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
                    expression: Expression::Binary(binary.clone()),
                }),
                Literal::Boolean(_) => Err(InterpreterError {
                    message: "Expected number, got boolean".to_owned(),
                    expression: Expression::Binary(binary.clone()),
                }),
            },
            Value::Empty => Err(InterpreterError {
                message: EMPTY_VALUE_OPERATION_ERROR_MESSAGE.to_owned(),
                expression: Expression::Binary(binary.clone()),
            }),
            Value::Optional(_) => Err(InterpreterError {
                message: NULLABLE_VALUE_OPERATION_ERROR_MESSAGE.to_owned(),
                expression: Expression::Binary(binary.clone()),
            }),
        }
    }
}
