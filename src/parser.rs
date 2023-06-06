use crate::{
    ast::{BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression},
    token::{Token, TokenData},
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

pub struct Parser {
    current_index: usize,
    error_mode: bool,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        return Parser {
            current_index: 0,
            error_mode: false,
            tokens,
        };
    }
    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        return self.expression();
    }
    pub fn expression(&mut self) -> Result<Expression, ParserError> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expression, ParserError> {
        let mut expr = self.comparison()?;

        while self.match_token_types(&[&TokenData::DoubleEqual, &TokenData::BangEqual]) {
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
            &TokenData::Greater,
            &TokenData::GreaterEqual,
            &TokenData::Less,
            &TokenData::LessEqual,
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

        while self.match_token_types(&[&TokenData::Minus, &TokenData::Plus]) {
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

        while self.match_token_types(&[&TokenData::Slash, &TokenData::Star]) {
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
        if self.match_token_types(&[&TokenData::Bang, &TokenData::Minus]) {
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
            &TokenData::False,
            &TokenData::True,
            &TokenData::Nil,
            &TokenData::String(String::new()),
            &TokenData::Integer(0),
            &TokenData::Float(0.0),
        ]) {
            let value = self.previous();
            return Ok(Expression::Literal(LiteralExpression {
                value: value.clone(),
            }));
        }

        if self.match_token_types(&[&TokenData::LeftParen]) {
            let expression = self.expression()?;
            if !self.check(&TokenData::RightParen) {
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
        return std::mem::discriminant(&self.next().token_data)
            == std::mem::discriminant(&TokenData::Eof);
    }

    fn next(&self) -> &Token {
        return &self.tokens[self.current_index];
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current_index - 1];
    }

    fn check(&self, token_type: &TokenData) -> bool {
        if self.is_end() {
            return false;
        }

        return std::mem::discriminant(token_type)
            == std::mem::discriminant(&self.next().token_data);
    }

    fn advance(&mut self) -> &Token {
        if !self.is_end() {
            self.current_index += 1;
        }

        return self.previous();
    }

    fn match_token_types(&mut self, types: &[&TokenData]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }
}
