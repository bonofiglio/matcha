use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    environment::Environment,
    matcha::{Literal, NumberLiteral, Value},
    statement::{
        AssignmentExpression, BinaryExpression, Expression, GroupingExpression, IfStatement,
        LiteralExpression, Statement, UnaryExpression, VariableDeclaration, VariableExpression,
        WhileStatement,
    },
    token::TokenType,
};

const NULLABLE_VALUE_OPERATION_ERROR_MESSAGE: &str =
    "Cannot execute an operation in an optional value. Try unwrapping it first";
const EMPTY_VALUE_OPERATION_ERROR_MESSAGE: &str =
    "Cannot execute a unary operation in an empty value";

#[derive(Debug)]
pub struct InterpreterError<'a> {
    pub message: String,
    pub statement: Statement<'a>,
}

impl Display for InterpreterError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime error: {}")
    }
}

pub struct Interpreter {}

impl<'a> Interpreter {
    pub fn interpret<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        statements: &'b [Statement<'a>],
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        for i in 0..statements.len() {
            // Return last value
            if i == statements.len() - 1 {
                return Interpreter::evaluate(environment, &statements[i]);
            }

            Interpreter::evaluate(Rc::clone(&environment), &statements[i])?;
        }

        Ok(Value::Empty)
    }

    fn evaluate<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        statement: &'b Statement<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        match statement {
            Statement::VariableDeclaration(decl) => {
                Interpreter::variable_declaration(environment, decl)?;
                Ok(Value::Empty)
            }
            Statement::Expression(expression) => Interpreter::expression(environment, expression),
            Statement::Block(block) => Interpreter::block(environment, block),
            Statement::If(if_statement) => Interpreter::if_statement(environment, if_statement),
            Statement::While(while_statement) => {
                Interpreter::while_statement(environment, while_statement)
            }
        }
    }

    fn expression<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        expression: &'b Expression<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        match expression {
            Expression::Literal(literal) => Interpreter::literal(literal),
            Expression::Unary(unary) => Interpreter::unary(environment, unary),
            Expression::Grouping(grouping) => Interpreter::grouping(environment, grouping),
            Expression::Binary(binary) => Interpreter::binary(environment, binary),
            Expression::Variable(variable) => {
                let borrow = environment.borrow();
                let result = Interpreter::variable_expression(&borrow, variable)?;

                Ok(result)
            }
            Expression::Assignment(assignment) => Interpreter::assign(environment, assignment),
            Expression::Logical(logical) => Interpreter::logical(environment, logical),
        }
    }

    fn literal(literal: &LiteralExpression<'a>) -> Result<Value<'a>, InterpreterError<'a>> {
        let value = &literal.value.literal;
        match value {
            Some(value) => Ok(Value::Literal(value.clone())),
            None => Err(InterpreterError {
                message: "Literal expression value is None. This should never be the case."
                    .to_owned(),
                statement: Statement::Expression(Expression::Literal(literal.clone())),
            }),
        }
    }

    fn grouping<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        grouping: &'b GroupingExpression<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        Interpreter::expression(environment, &grouping.expression)
    }

    fn unary<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        unary: &'b UnaryExpression<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
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
                    NumberLiteral::Integer(integer) => Ok(Value::Literal(Literal::Number(
                        NumberLiteral::Integer(-integer),
                    ))),
                    NumberLiteral::Float(float) => Ok(Value::Literal(Literal::Number(
                        NumberLiteral::Float(-float),
                    ))),
                },
                _ => Err(InterpreterError {
                    message: "Cannot use operator \"-\" on non-numeric value".to_owned(),
                    statement: Statement::Expression(Expression::Unary(unary.clone())),
                }),
            },
            TokenType::Bang => match value {
                Literal::Boolean(bool) => Ok(Value::Literal(Literal::Boolean(!bool))),
                _ => Err(InterpreterError {
                    message: "Cannot negate non-boolean value".to_owned(),
                    statement: Statement::Expression(Expression::Unary(unary.clone())),
                }),
            },
            _ => Err(InterpreterError {
                message: format!(
                    "Unexpected unary operator. {} is not a valid unary operator",
                    &unary.operator.lexeme
                ),
                statement: Statement::Expression(Expression::Unary(unary.clone())),
            }),
        }
    }

    fn binary<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        binary: &'b BinaryExpression<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        let left_value = Interpreter::expression(Rc::clone(&environment), &binary.left)?;
        let right_value = Interpreter::expression(Rc::clone(&environment), &binary.right)?;

        match binary.operator.token_type {
            TokenType::Plus => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                Ok(Value::Literal(Literal::Number(left + right)))
            }
            TokenType::Minus => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                Ok(Value::Literal(Literal::Number(left - right)))
            }
            TokenType::Star => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                Ok(Value::Literal(Literal::Number(left * right)))
            }
            TokenType::Slash => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                Ok(Value::Literal(Literal::Number(left / right)))
            }
            TokenType::Greater => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                Ok(Value::Literal(Literal::Boolean(left > right)))
            }
            TokenType::GreaterEqual => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                Ok(Value::Literal(Literal::Boolean(left >= right)))
            }
            TokenType::Less => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                Ok(Value::Literal(Literal::Boolean(left < right)))
            }
            TokenType::LessEqual => {
                let left = Interpreter::unwrap_number(left_value, binary)?;
                let right = Interpreter::unwrap_number(right_value, binary)?;

                Ok(Value::Literal(Literal::Boolean(left <= right)))
            }
            TokenType::DoubleEqual => match (left_value, right_value) {
                (Value::Literal(ref left_literal), Value::Literal(ref right_literal)) => {
                    match (left_literal, right_literal) {
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
                            statement: Statement::Expression(Expression::Binary(binary.clone())),
                        }),
                    }
                }
                _ => Err(InterpreterError {
                    message: "Can't compare non-literal values".to_owned(),
                    statement: Statement::Expression(Expression::Binary(binary.clone())),
                }),
            },
            TokenType::BangEqual => match (left_value, right_value) {
                (Value::Literal(ref left_literal), Value::Literal(ref right_literal)) => {
                    match (left_literal, right_literal) {
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
                            statement: Statement::Expression(Expression::Binary(binary.clone())),
                        }),
                    }
                }
                _ => Err(InterpreterError {
                    message: "Can't compare non-literal values".to_owned(),
                    statement: Statement::Expression(Expression::Binary(binary.clone())),
                }),
            },
            _ => Err(InterpreterError {
                message: format!("Invalid operator '{}'", binary.operator.lexeme),
                statement: Statement::Expression(Expression::Binary(binary.clone())),
            }),
        }
    }

    fn unwrap_number(
        value: Value<'a>,
        binary: &BinaryExpression<'a>,
    ) -> Result<NumberLiteral, InterpreterError<'a>> {
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

    fn variable_declaration<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        decl: &'b VariableDeclaration<'a>,
    ) -> Result<(), InterpreterError<'a>> {
        let value = match decl.initializer {
            Some(ref initializer) => match initializer {
                Expression::Literal(literal_expression) => match literal_expression.value.literal {
                    Some(ref literal) => Value::Literal(literal.clone()),
                    None => Value::Optional(None),
                },
                _ => Interpreter::expression(Rc::clone(&environment), initializer)?,
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
        Ok(())
    }

    fn variable_expression<'b>(
        environment: &Environment<'a>,
        variable: &'b VariableExpression<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        match environment.values.get(variable.value.lexeme) {
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
        }
    }

    fn block<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        statements: &'b Vec<Statement<'a>>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        let inner_environment = Rc::new(RefCell::new(Environment::with_parent(environment)));

        Interpreter::interpret(inner_environment, statements)
    }

    fn if_statement<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        if_statement: &'b IfStatement<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        let condition_result =
            Interpreter::expression(Rc::clone(&environment), &if_statement.condition)?;

        let statements_to_execute = match condition_result {
            Value::Literal(Literal::Boolean(bool_literal)) => {
                if bool_literal {
                    Some(&if_statement.statements)
                } else {
                    if_statement.else_statements.as_ref()
                }
            }
            _ => {
                return Err(InterpreterError {
                    message: "Expected boolean condition".to_owned(),
                    statement: Statement::If(if_statement.clone()),
                })
            }
        };

        match statements_to_execute {
            Some(statements) => Interpreter::block(environment, statements),
            None => Ok(Value::Empty),
        }
    }

    fn assign<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        assignment: &'b AssignmentExpression<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        let env_borrow = environment.borrow();
        let current_value = env_borrow.values.get(assignment.name.lexeme);

        match current_value {
            Some(_) => {
                drop(env_borrow);

                let new_value =
                    Interpreter::expression(Rc::clone(&environment), &assignment.value)?;
                let mut env_borrow_mut = environment.borrow_mut();
                let prev = env_borrow_mut
                    .values
                    .get_mut(assignment.name.lexeme)
                    .unwrap();

                *prev = new_value;

                Ok(Value::Empty)
            }
            None => match &env_borrow.parent {
                Some(parent) => Interpreter::assign(Rc::clone(parent), assignment),
                None => Err(InterpreterError {
                    message: format!(
                        "Cannot assign a value to undeclared variable '{}'",
                        assignment.name.lexeme
                    ),
                    statement: Statement::Expression(Expression::Assignment(assignment.clone())),
                }),
            },
        }
    }

    fn while_statement<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        while_statement: &'b WhileStatement<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        while match Interpreter::unwrap_bool(Interpreter::expression(
            Rc::clone(&environment),
            &while_statement.condition,
        )?) {
            Ok(boolean) => Ok(boolean),
            Err(message) => Err(InterpreterError {
                message,
                statement: Statement::While(while_statement.clone()),
            }),
        }? {
            Interpreter::block(Rc::clone(&environment), &while_statement.statements)?;
        }

        Ok(Value::Empty)
    }

    fn unwrap_bool(value: Value) -> Result<bool, String> {
        match value {
            Value::Literal(literal) => match literal {
                Literal::Boolean(boolean) => Ok(boolean),
                Literal::Number(_) => Err("Expected boolean, got number".to_owned()),
                Literal::String(_) => Err("Expected number, got string".to_owned()),
            },
            Value::Empty => Err(EMPTY_VALUE_OPERATION_ERROR_MESSAGE.to_owned()),
            Value::Optional(_) => Err(NULLABLE_VALUE_OPERATION_ERROR_MESSAGE.to_owned()),
        }
    }

    fn logical<'b>(
        environment: Rc<RefCell<Environment<'a>>>,
        logical: &'b BinaryExpression<'a>,
    ) -> Result<Value<'a>, InterpreterError<'a>> {
        match logical.operator.token_type {
            TokenType::Or => {
                let left_result = Interpreter::unwrap_bool(Interpreter::expression(
                    Rc::clone(&environment),
                    &logical.left,
                )?);

                let left_value = match left_result {
                    Ok(boolean) => Ok(boolean),
                    Err(message) => Err(InterpreterError {
                        message,
                        statement: Statement::Expression(Expression::Logical(logical.clone())),
                    }),
                }?;

                if left_value {
                    Ok(Value::Literal(Literal::Boolean(true)))
                } else {
                    let right_result = Interpreter::unwrap_bool(Interpreter::expression(
                        Rc::clone(&environment),
                        &logical.right,
                    )?);

                    let right_value = match right_result {
                        Ok(boolean) => Ok(boolean),
                        Err(message) => Err(InterpreterError {
                            message,
                            statement: Statement::Expression(Expression::Logical(logical.clone())),
                        }),
                    }?;

                    Ok(Value::Literal(Literal::Boolean(right_value)))
                }
            }
            TokenType::And => {
                let left_result = Interpreter::unwrap_bool(Interpreter::expression(
                    Rc::clone(&environment),
                    &logical.left,
                )?);

                let left_value = match left_result {
                    Ok(boolean) => Ok(boolean),
                    Err(message) => Err(InterpreterError {
                        message,
                        statement: Statement::Expression(Expression::Logical(logical.clone())),
                    }),
                }?;

                if !left_value {
                    Ok(Value::Literal(Literal::Boolean(false)))
                } else {
                    let right_result = Interpreter::unwrap_bool(Interpreter::expression(
                        Rc::clone(&environment),
                        &logical.right,
                    )?);

                    let right_value = match right_result {
                        Ok(boolean) => Ok(boolean),
                        Err(message) => Err(InterpreterError {
                            message,
                            statement: Statement::Expression(Expression::Logical(logical.clone())),
                        }),
                    }?;

                    Ok(Value::Literal(Literal::Boolean(left_value && right_value)))
                }
            }
            _ => todo!(),
        }
    }
}
