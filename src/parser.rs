use std::fmt::Display;

use crate::{
    ast::{BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression},
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
        writeln!(
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

    pub fn parse(&mut self) -> Result<Expression, Vec<ParserError>> {
        self.current_index = 0;

        let mut errors = Vec::<ParserError>::new();

        while !self.is_end() {
            let result = self.expression();

            match result {
                Ok(expression) => {
                    if errors.len() == 0 {
                        return Ok(expression);
                    }
                }
                Err(e) => {
                    errors.push(e);
                    self.sync()
                }
            }
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
                | TokenType::Class
                | TokenType::Func
                | TokenType::Var
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

    fn expression(&mut self) -> Result<Expression, ParserError> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.comparison()?;

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
            "Unexpected character".to_owned(),
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
}
