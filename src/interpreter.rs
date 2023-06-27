use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    matcha::{Literal, NumberLiteral, Value},
    statement::{
        AssignmentExpression, BinaryExpression, Expression, GroupingExpression, IfStatement,
        LiteralExpression, Statement, UnaryExpression, VariableDeclaration, VariableExpression,
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
        environment: Rc<RefCell<Environment>>,
        statements: &Vec<Statement>,
    ) -> Result<Value, InterpreterError> {
        for i in 0..statements.len() {
            // Return last value
            if i == statements.len() - 1 {
                return Ok(Interpreter::evaluate(environment, &statements[i])?);
            }

            Interpreter::evaluate(Rc::clone(&environment), &statements[i])?;
        }

        return Ok(Value::Empty);
    }

    fn evaluate(
        environment: Rc<RefCell<Environment>>,
        statement: &Statement,
    ) -> Result<Value, InterpreterError> {
        return match statement {
            Statement::VariableDeclaration(decl) => {
                let _ = Interpreter::variable_declaration(environment, decl)?;
                return Ok(Value::Empty);
            }
            Statement::Expression(expression) => Interpreter::expression(environment, expression),
            Statement::Block(block) => Interpreter::block(environment, block),
            Statement::If(if_statement) => Interpreter::if_statement(environment, if_statement),
        };
    }

    fn expression(
        environment: Rc<RefCell<Environment>>,
        expression: &Expression,
    ) -> Result<Value, InterpreterError> {
        return match expression {
            Expression::Literal(literal) => Interpreter::literal(literal),
            Expression::Unary(unary) => Interpreter::unary(environment, unary),
            Expression::Grouping(grouping) => Interpreter::grouping(environment, grouping),
            Expression::Binary(binary) => Interpreter::binary(environment, binary),
            Expression::Variable(variable) => {
                Interpreter::variable_expression(&environment.borrow(), variable)
            }
            Expression::Assignment(assignment) => Interpreter::assign(environment, assignment),
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
        environment: Rc<RefCell<Environment>>,
        grouping: &GroupingExpression,
    ) -> Result<Value, InterpreterError> {
        return Interpreter::expression(environment, &grouping.expression);
    }

    fn unary(
        environment: Rc<RefCell<Environment>>,
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
        environment: Rc<RefCell<Environment>>,
        binary: &BinaryExpression,
    ) -> Result<Value, InterpreterError> {
        let left_value = Interpreter::expression(Rc::clone(&environment), &binary.left)?;
        let right_value = Interpreter::expression(Rc::clone(&environment), &binary.right)?;

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
            TokenType::Greater => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                return Ok(Value::Literal(Literal::Boolean(left > right)));
            }
            TokenType::GreaterEqual => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                return Ok(Value::Literal(Literal::Boolean(left >= right)));
            }
            TokenType::Less => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                return Ok(Value::Literal(Literal::Boolean(left < right)));
            }
            TokenType::LessEqual => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                return Ok(Value::Literal(Literal::Boolean(left <= right)));
            }
            TokenType::DoubleEqual => {
                return match (left_value, right_value) {
                    (Value::Literal(ref left_literal), Value::Literal(ref right_literal)) => {
                        return match (left_literal, right_literal) {
                            (Literal::Number(left_number), Literal::Number(right_number)) => Ok(
                                Value::Literal(Literal::Boolean(left_number == right_number)),
                            ),
                            (Literal::String(left_string), Literal::String(right_string)) => Ok(
                                Value::Literal(Literal::Boolean(left_string == right_string)),
                            ),
                            (Literal::Boolean(left_bool), Literal::Boolean(right_bool)) => {
                                Ok(Value::Literal(Literal::Boolean(left_bool == right_bool)))
                            }
                            _ => Err(InterpreterError {
                                message: format!(
                                    "Can't compare {} with {}",
                                    left_literal.get_type(),
                                    right_literal.get_type()
                                ),
                                statement: Statement::Expression(Expression::Binary(
                                    binary.clone(),
                                )),
                            }),
                        }
                    }
                    _ => Err(InterpreterError {
                        message: "Can't compare non-literal values".to_owned(),
                        statement: Statement::Expression(Expression::Binary(binary.clone())),
                    }),
                };
            }
            TokenType::BangEqual => {
                return match (left_value, right_value) {
                    (Value::Literal(ref left_literal), Value::Literal(ref right_literal)) => {
                        return match (left_literal, right_literal) {
                            (Literal::Number(left_number), Literal::Number(right_number)) => Ok(
                                Value::Literal(Literal::Boolean(left_number != right_number)),
                            ),
                            (Literal::String(left_string), Literal::String(right_string)) => Ok(
                                Value::Literal(Literal::Boolean(left_string != right_string)),
                            ),
                            (Literal::Boolean(left_bool), Literal::Boolean(right_bool)) => {
                                Ok(Value::Literal(Literal::Boolean(left_bool != right_bool)))
                            }
                            _ => Err(InterpreterError {
                                message: format!(
                                    "Can't compare {} with {}",
                                    left_literal.get_type(),
                                    right_literal.get_type()
                                ),
                                statement: Statement::Expression(Expression::Binary(
                                    binary.clone(),
                                )),
                            }),
                        }
                    }
                    _ => Err(InterpreterError {
                        message: "Can't compare non-literal values".to_owned(),
                        statement: Statement::Expression(Expression::Binary(binary.clone())),
                    }),
                };
            }
            _ => Err(InterpreterError {
                message: format!("Invalid operator '{}'", binary.operator.lexeme),
                statement: Statement::Expression(Expression::Binary(binary.clone())),
            }),
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
        environment: Rc<RefCell<Environment>>,
        decl: &VariableDeclaration,
    ) -> Result<(), InterpreterError> {
        let value = match decl.initializer {
            Some(ref initializer) => match initializer {
                Expression::Literal(literal_expression) => match literal_expression.value.literal {
                    Some(ref literal) => Value::Literal(literal.clone()),
                    None => Value::Optional(None),
                },
                _ => Interpreter::expression(Rc::clone(&environment), &initializer)?,
            },
            None => Value::Empty,
        };

        let result = environment
            .borrow_mut()
            .values
            .insert(decl.identifier.lexeme.to_owned(), value);

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
                Some(ref parent) => Interpreter::variable_expression(&parent.borrow(), variable),
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
        environment: Rc<RefCell<Environment>>,
        statements: &Vec<Statement>,
    ) -> Result<Value, InterpreterError> {
        let mut inner_environment = Rc::new(RefCell::new(Environment::with_parent(environment)));

        return Interpreter::interpret(inner_environment, statements);
    }

    fn if_statement(
        environment: Rc<RefCell<Environment>>,
        if_statement: &IfStatement,
    ) -> Result<Value, InterpreterError> {
        let condition_result =
            Interpreter::expression(Rc::clone(&environment), &if_statement.condition)?;

        let statements_to_execute = match condition_result {
            Value::Literal(literal) => match literal {
                Literal::Boolean(bool_literal) => {
                    if bool_literal {
                        Some(&if_statement.statements)
                    } else {
                        match if_statement.else_statements {
                            Some(ref statements) => Some(statements),
                            None => None,
                        }
                    }
                }
                _ => {
                    return Err(InterpreterError {
                        message: "Expected boolean condition".to_owned(),
                        statement: Statement::If(if_statement.clone()),
                    })
                }
            },
            _ => {
                return Err(InterpreterError {
                    message: "Expected boolean condition".to_owned(),
                    statement: Statement::If(if_statement.clone()),
                })
            }
        };

        return match statements_to_execute {
            Some(ref statements) => Interpreter::block(environment, statements),
            None => Ok(Value::Empty),
        };
    }

    fn assign(
        environment: Rc<RefCell<Environment>>,
        assignment: &AssignmentExpression,
    ) -> Result<Value, InterpreterError> {
        let mut env_borrow = environment.borrow_mut();
        let current_value = env_borrow.values.get(&assignment.name.lexeme);

        match current_value {
            Some(_) => {
                let new_value =
                    Interpreter::expression(Rc::clone(&environment), &assignment.value)?;
                let prev = env_borrow.values.get_mut(&assignment.name.lexeme).unwrap();

                *prev = new_value;

                return Ok(Value::Empty);
            }
            None => match &env_borrow.parent {
                Some(parent) => return Ok(Interpreter::assign(Rc::clone(parent), assignment)?),
                None => {
                    return Err(InterpreterError {
                        message: format!(
                            "Cannot assign a value to undeclared variable '{}'",
                            assignment.name.lexeme
                        ),
                        statement: Statement::Expression(Expression::Assignment(
                            assignment.clone(),
                        )),
                    })
                }
            },
        }
    }
}
