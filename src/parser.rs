use crate::{
    ast::{BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression},
    token::{Token, TokenData},
};

#[derive(Debug)]
pub struct ParserError<'a> {
    pub message: String,
    pub token: &'a Token,
}
impl<'a> ParserError<'a> {
    pub fn new(message: String, token: &'a Token) -> ParserError {
        return ParserError { message, token };
    }
}

pub struct Parser {}

impl<'a> Parser {
    pub fn parse(tokens: &'a Vec<Token>) -> Result<Expression<'a>, ParserError<'a>> {
        let mut current_index = 0;
        return Parser::expression(tokens, &mut current_index);
    }
    pub fn expression(
        tokens: &'a Vec<Token>,
        current_index: &mut usize,
    ) -> Result<Expression<'a>, ParserError<'a>> {
        return Parser::equality(tokens, current_index);
    }

    fn equality(
        tokens: &'a Vec<Token>,
        current_index: &mut usize,
    ) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = Parser::comparison(tokens, current_index)?;
        while Parser::match_token_types(
            tokens,
            current_index,
            &[&TokenData::DoubleEqual, &TokenData::BangEqual],
        ) {
            let operator = Parser::previous(tokens, *current_index);
            let right = Parser::comparison(tokens, current_index)?;
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    fn comparison(
        tokens: &'a Vec<Token>,
        current_index: &mut usize,
    ) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = Parser::term(tokens, current_index)?;

        while Parser::match_token_types(
            tokens,
            current_index,
            &[
                &TokenData::Greater,
                &TokenData::GreaterEqual,
                &TokenData::Less,
                &TokenData::LessEqual,
            ],
        ) {
            let operator = Parser::previous(tokens, *current_index);
            let right = Parser::term(tokens, current_index)?;

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    fn term(
        tokens: &'a Vec<Token>,
        current_index: &mut usize,
    ) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = Parser::factor(tokens, current_index)?;

        while Parser::match_token_types(
            tokens,
            current_index,
            &[&TokenData::Minus, &TokenData::Plus],
        ) {
            let operator = Parser::previous(tokens, *current_index);
            let right = Parser::factor(tokens, current_index)?;

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    fn factor(
        tokens: &'a Vec<Token>,
        current_index: &mut usize,
    ) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = Parser::unary(tokens, current_index)?;

        while Parser::match_token_types(
            tokens,
            current_index,
            &[&TokenData::Slash, &TokenData::Star],
        ) {
            let operator = Parser::previous(tokens, *current_index);
            let right = Parser::unary(tokens, current_index)?;

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }

    fn unary(
        tokens: &'a Vec<Token>,
        current_index: &mut usize,
    ) -> Result<Expression<'a>, ParserError<'a>> {
        if Parser::match_token_types(
            tokens,
            current_index,
            &[&TokenData::Bang, &TokenData::Minus],
        ) {
            let operator = Parser::previous(tokens, *current_index);

            return Ok(Expression::Unary(UnaryExpression {
                operator,
                left: Box::new(Parser::unary(tokens, current_index)?),
            }));
        }

        return Parser::primary(tokens, current_index);
    }

    fn primary(
        tokens: &'a Vec<Token>,
        current_index: &mut usize,
    ) -> Result<Expression<'a>, ParserError<'a>> {
        let current = Parser::next(tokens, *current_index);
        if Parser::match_token_types(
            tokens,
            current_index,
            &[
                &TokenData::False,
                &TokenData::True,
                &TokenData::Nil,
                &TokenData::String(String::new()),
                &TokenData::Integer(0),
                &TokenData::Float(0.0),
            ],
        ) {
            return Ok(Expression::Literal(LiteralExpression { value: current }));
        }

        if Parser::match_token_types(tokens, current_index, &[&TokenData::LeftParen]) {
            let expression = Parser::expression(tokens, current_index)?;
            if !Parser::check(tokens, *current_index, &TokenData::RightParen) {
                let token = Parser::next(tokens, *current_index);
                return Err(ParserError::new(
                    format!("Expected ')' after expression. Got: {}", token.lexeme),
                    token,
                ));
            }
            return Ok(Expression::Grouping(GroupingExpression {
                expression: Box::new(expression),
            }));
        }

        return Err(ParserError::new("Unexpected character".to_owned(), current));
    }

    fn is_end(tokens: &Vec<Token>, current_index: usize) -> bool {
        return std::mem::discriminant(&Parser::next(tokens, current_index).token_data)
            == std::mem::discriminant(&TokenData::Eof);
    }

    fn next(tokens: &Vec<Token>, current_index: usize) -> &Token {
        return &tokens[current_index];
    }

    fn previous(tokens: &Vec<Token>, current_index: usize) -> &Token {
        return &tokens[current_index - 1];
    }

    fn check(tokens: &Vec<Token>, current_index: usize, token_type: &TokenData) -> bool {
        if Parser::is_end(tokens, current_index) {
            return false;
        }

        return std::mem::discriminant(token_type)
            == std::mem::discriminant(&Parser::next(tokens, current_index).token_data);
    }

    fn advance(tokens: &'a Vec<Token>, current_index: &mut usize) -> &'a Token {
        if !Parser::is_end(tokens, *current_index) {
            *current_index += 1;
        }

        return Parser::previous(tokens, *current_index);
    }

    fn match_token_types(
        tokens: &Vec<Token>,
        current_index: &mut usize,
        types: &[&TokenData],
    ) -> bool {
        for token_type in types {
            if Parser::check(tokens, *current_index, token_type) {
                Parser::advance(tokens, current_index);
                return true;
            }
        }

        return false;
    }
}
