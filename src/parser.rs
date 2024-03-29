use std::fmt::Display;

use crate::{
    statement::{
        AssignmentExpression, BinaryExpression, Expression, GroupingExpression, IfStatement,
        LiteralExpression, Statement, UnaryExpression, VariableDeclaration, VariableExpression,
        WhileStatement,
    },
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub token: Token,
}

impl ParserError {
    pub fn new(message: String, token: Token) -> ParserError {
        return ParserError { message, token };
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parser error at {}:{}. {}",
            self.token.line, self.token.position, self.message
        )
    }
}

pub struct Parser {
    current_index: usize,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        return Parser {
            current_index: 0,
            tokens,
        };
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, Vec<ParserError>> {
        self.current_index = 0;

        let mut statements = Vec::<Statement>::new();
        let mut errors = Vec::<ParserError>::new();

        while !self.is_end() {
            let result = self.statement();

            match result {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(e) => {
                    errors.push(e);
                    self.sync()
                }
            }
        }

        if errors.len() == 0 {
            return Ok(statements);
        }

        return Err(errors);
    }

    fn sync(&mut self) {
        self.advance();

        while !self.is_end() {
            // Skip any tokens that are not one of the specified
            match self.previous().token_type {
                TokenType::SemiColon
                | TokenType::For
                | TokenType::While
                | TokenType::Struct
                | TokenType::Func
                | TokenType::Let
                | TokenType::If
                | TokenType::Return => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn statement(&mut self) -> Result<Statement, ParserError> {
        if self.match_token_types(&[&TokenType::If]) {
            return self.if_statement();
        }

        if self.match_token_types(&[&TokenType::While]) {
            return self.while_statement();
        }

        if self.match_token_types(&[&TokenType::Let]) {
            return self.variable_declaration();
        }

        if self.match_token_types(&[&TokenType::LeftBrace]) {
            return Ok(Statement::Block(self.block()?));
        }

        return self.expression_statement();
    }

    fn variable_declaration(&mut self) -> Result<Statement, ParserError> {
        let identifier = self
            .consume_token(TokenType::Identifier, "Expected identifier".to_owned())?
            .clone();

        let initializer = if self.match_token_types(&[&TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };

        let declaration = Statement::VariableDeclaration(VariableDeclaration {
            identifier,
            initializer,
        });

        let _ = self.consume_token(TokenType::SemiColon, "Expected ';'".to_owned())?;

        return Ok(declaration);
    }

    fn expression_statement(&mut self) -> Result<Statement, ParserError> {
        let expr = self.expression()?;

        let _ = self.consume_token(TokenType::SemiColon, "Expected ';'".to_owned())?;

        return Ok(Statement::Expression(expr));
    }

    fn expression(&mut self) -> Result<Expression, ParserError> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expression, ParserError> {
        let expr = self.or()?;

        if self.match_token_types(&[&TokenType::Equal]) {
            let equals = self.previous();

            match expr {
                Expression::Variable(variable) => {
                    return Ok(Expression::Assignment(AssignmentExpression {
                        value: Box::new(self.assignment()?),
                        name: variable.value,
                    }))
                }
                _ => {
                    return Err(ParserError {
                        message: "Invalid assignment target".to_owned(),
                        token: equals.clone(),
                    })
                }
            }
        };

        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.and()?;

        while self.match_token_types(&[&TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            expr = Expression::Logical(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    fn and(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.equality()?;

        while self.match_token_types(&[&TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            expr = Expression::Logical(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.comparison()?;

        if self.check(&TokenType::Equal) {}

        while self.match_token_types(&[&TokenType::DoubleEqual, &TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = Box::new(self.comparison()?);

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.term()?;

        while self.match_token_types(&[
            &TokenType::Greater,
            &TokenType::GreaterEqual,
            &TokenType::Less,
            &TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.factor()?;

        while self.match_token_types(&[&TokenType::Minus, &TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.unary()?;

        while self.match_token_types(&[&TokenType::Slash, &TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expression, ParserError> {
        if self.match_token_types(&[&TokenType::Bang, &TokenType::Minus]) {
            let operator = self.previous().clone();

            return Ok(Expression::Unary(UnaryExpression {
                operator,
                left: Box::new(self.unary()?),
            }));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expression, ParserError> {
        if self.match_token_types(&[
            &TokenType::False,
            &TokenType::True,
            &TokenType::Nil,
            &TokenType::String,
            &TokenType::Integer,
            &TokenType::Float,
        ]) {
            let value = self.previous();
            return Ok(Expression::Literal(LiteralExpression {
                value: value.clone(),
            }));
        }

        if self.next().token_type == TokenType::Identifier {
            self.advance();
            return Ok(Expression::Variable(VariableExpression {
                value: self.previous().clone(),
            }));
        }

        if self.match_token_types(&[&TokenType::LeftParen]) {
            let expression = self.expression()?;
            if !self.check(&TokenType::RightParen) {
                let token = self.next();
                return Err(ParserError::new(
                    format!("Expected ')' after expression. Got: {}", token.lexeme),
                    token.clone(),
                ));
            }

            self.advance();

            return Ok(Expression::Grouping(GroupingExpression {
                expression: Box::new(expression),
            }));
        }

        let current = self.next();

        return Err(ParserError::new(
            format!("Unexpected token '{:#?}'", current),
            current.clone(),
        ));
    }

    fn is_end(&self) -> bool {
        return std::mem::discriminant(&self.next().token_type)
            == std::mem::discriminant(&TokenType::Eof);
    }

    fn next(&self) -> &Token {
        return &self.tokens[self.current_index];
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current_index - 1];
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_end() {
            return false;
        }

        return std::mem::discriminant(token_type)
            == std::mem::discriminant(&self.next().token_type);
    }

    fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.current_index += 1;
        }

        return self.previous();
    }

    fn match_token_types(&mut self, types: &[&TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn consume_token(
        &mut self,
        token_type: TokenType,
        error_message: String,
    ) -> Result<&Token, ParserError> {
        self.advance();
        let previous = self.previous();

        if previous.token_type == token_type {
            return Ok(previous);
        }

        return Err(ParserError::new(error_message, previous.clone()));
    }

    fn block(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::<Statement>::new();

        while !self.check(&TokenType::RightBrace) && !self.is_end() {
            statements.push(self.statement()?);
        }

        let _ = self.consume_token(TokenType::RightBrace, "Expected '}' after block".to_owned())?;

        return Ok(statements);
    }

    fn if_statement(&mut self) -> Result<Statement, ParserError> {
        let condition = self.expression()?;

        let _ = self.consume_token(
            TokenType::LeftBrace,
            "Expected '{{' after condition".to_owned(),
        )?;

        let statements = self.block()?;

        let else_statements = if self.match_token_types(&[&TokenType::Else]) {
            let _ = self.consume_token(
                TokenType::LeftBrace,
                "Expected '{{' after condition".to_owned(),
            )?;

            Some(self.block()?)
        } else {
            None
        };

        return Ok(Statement::If(IfStatement {
            condition,
            statements,
            else_statements,
        }));
    }

    fn while_statement(&mut self) -> Result<Statement, ParserError> {
        let condition = self.expression()?;

        let _ = self.consume_token(
            TokenType::LeftBrace,
            "Expected '{{' after condition".to_owned(),
        )?;

        let statements = self.block()?;

        return Ok(Statement::While(WhileStatement {
            condition,
            statements,
        }));
    }
}
