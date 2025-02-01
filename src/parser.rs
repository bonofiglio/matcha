use std::fmt::Display;

use crate::{
    statement::{
        AssignmentExpression, BinaryExpression, Expression, ForStatement, GroupingExpression,
        IfStatement, LiteralExpression, Statement, UnaryExpression, VariableDeclaration,
        VariableExpression,
    },
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct ParserError<'a> {
    pub message: String,
    pub token: Token<'a>,
}

impl ParserError<'_> {
    pub fn new(message: String, token: Token) -> ParserError {
        ParserError { message, token }
    }
}

impl Display for ParserError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Parser error at {}:{}. {}",
            self.token.line, self.token.position, self.message
        )
    }
}

pub struct Parser<'a> {
    current_index: usize,
    tokens: Vec<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            current_index: 0,
            tokens,
        }
    }

    pub fn parse(mut self) -> Result<Vec<Statement<'a>>, Vec<ParserError<'a>>> {
        self.current_index = 0;

        let mut statements = Vec::<Statement>::new();
        let mut errors = Vec::<ParserError<'a>>::new();

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

        if errors.is_empty() {
            return Ok(statements);
        }

        Err(errors)
    }

    #[inline]
    fn sync(&mut self) {
        self.advance();

        while !self.is_end() {
            // Skip any tokens that are not one of the specified
            match self.previous().token_type {
                TokenType::SemiColon | TokenType::For | TokenType::If => {
                    return;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    #[inline]
    fn statement<'b>(&'b mut self) -> Result<Statement<'a>, ParserError<'a>> {
        if self.consumed_one_of([TokenType::If]) {
            return self.if_statement();
        }

        if self.consumed_one_of([TokenType::For]) {
            return self.while_statement();
        }

        match self.lookahead_many::<4>().map(|t| t.map(|t| t.token_type)) {
            [Some(TokenType::Identifier), Some(TokenType::Colon), Some(TokenType::Identifier), Some(TokenType::Equal)]
            | [Some(TokenType::Identifier), Some(TokenType::VarDec), ..] => {
                return self.variable_declaration()
            }
            _ => {}
        }

        if self.consumed_one_of([TokenType::LeftBrace]) {
            return Ok(Statement::Block(self.block()?));
        }

        self.expression_statement()
    }

    #[inline]
    fn expression_statement<'b>(&'b mut self) -> Result<Statement<'a>, ParserError<'a>> {
        let expr = self.expression()?;

        let _ = self.consume_and_expect(TokenType::SemiColon, "Expected ';'".to_owned())?;

        Ok(Statement::Expression(expr))
    }

    #[inline]
    fn expression<'b>(&'b mut self) -> Result<Expression<'a>, ParserError<'a>> {
        self.assignment()
    }

    #[inline]
    fn variable_declaration<'b>(&'b mut self) -> Result<Statement<'a>, ParserError<'a>> {
        let identifier = self
            .consume_and_expect(TokenType::Identifier, "Expected identifier".to_owned())?
            .clone();

        let r#type = if self.consumed_one_of([TokenType::Colon]) {
            Some(self.variable_declaration_type()?)
        } else {
            None
        };

        let initializer = if self.consumed_one_of([TokenType::VarDec, TokenType::Equal]) {
            self.expression()?
        } else {
            unreachable!()
        };

        let declaration = Statement::VariableDeclaration(VariableDeclaration {
            identifier,
            initializer,
            r#type,
        });

        let _ = self.consume_and_expect(TokenType::SemiColon, "Expected ';'".to_owned())?;

        Ok(declaration)
    }

    #[inline(always)]
    fn variable_declaration_type(&mut self) -> Result<Token<'a>, ParserError<'a>> {
        let identifier = self
            .consume_and_expect(TokenType::Identifier, "Expected type identifier".to_owned())?
            .clone();

        Ok(identifier)
    }

    #[inline]
    fn assignment<'b>(&'b mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let expr = self.or()?;

        if self.consumed_one_of([TokenType::Equal]) {
            let equals = self.previous();

            match expr {
                Expression::Variable(variable) => {
                    return Ok(Expression::Assignment(AssignmentExpression {
                        value: Box::new(self.assignment()?),
                        identifier: variable.value,
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

        Ok(expr)
    }

    #[inline]
    fn or<'b>(&'b mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.and()?;

        while self.consumed_one_of([TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            expr = Expression::Logical(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    #[inline]
    fn and<'b>(&'b mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.equality()?;

        while self.consumed_one_of([TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            expr = Expression::Logical(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    #[inline]
    fn equality(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.comparison()?;

        self.next_matches(TokenType::Equal);

        while self.consumed_one_of([TokenType::DoubleEqual, TokenType::BangEqual]) {
            let operator = self.previous().clone();
            let right = Box::new(self.comparison()?);

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right,
            });
        }

        Ok(expr)
    }

    #[inline]
    fn comparison(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.term()?;

        while self.consumed_one_of([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    #[inline]
    fn term(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.factor()?;

        while self.consumed_one_of([TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    #[inline]
    fn factor(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        let mut expr = self.unary()?;

        while self.consumed_one_of([TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    #[inline]
    fn unary(&mut self) -> Result<Expression<'a>, ParserError<'a>> {
        if self.consumed_one_of([TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();

            return Ok(Expression::Unary(UnaryExpression {
                operator,
                left: Box::new(self.unary()?),
            }));
        }

        self.primary()
    }

    #[inline]
    fn primary<'b>(&'b mut self) -> Result<Expression<'a>, ParserError<'a>> {
        if self.consumed_one_of([
            TokenType::False,
            TokenType::True,
            TokenType::String,
            TokenType::Integer,
            TokenType::Float,
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

        if self.consumed_one_of([TokenType::LeftParen]) {
            let expression = self.expression()?;
            if !self.next_matches(TokenType::RightParen) {
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

        Err(ParserError::new(
            format!("Unexpected token '{:#?}'", current),
            current.clone(),
        ))
    }

    #[inline]
    fn block<'b>(&'b mut self) -> Result<Vec<Statement<'a>>, ParserError<'a>> {
        let mut statements = Vec::<Statement>::new();

        while !self.next_matches(TokenType::RightBrace) && !self.is_end() {
            statements.push(self.statement()?);
        }

        let _ =
            self.consume_and_expect(TokenType::RightBrace, "Expected '}' after block".to_owned())?;

        Ok(statements)
    }

    #[inline]
    fn if_statement<'b>(&'b mut self) -> Result<Statement<'a>, ParserError<'a>> {
        let condition = self.expression()?;

        let _ = self.consume_and_expect(
            TokenType::LeftBrace,
            "Expected '{{' after condition".to_owned(),
        )?;

        let statements = self.block()?;

        let else_statements = if self.consumed_one_of([TokenType::Else]) {
            let _ = self.consume_and_expect(
                TokenType::LeftBrace,
                "Expected '{{' after condition".to_owned(),
            )?;

            Some(self.block()?)
        } else {
            None
        };

        Ok(Statement::If(IfStatement {
            condition,
            statements,
            else_statements,
        }))
    }

    #[inline]
    fn while_statement<'b>(&'b mut self) -> Result<Statement<'a>, ParserError<'a>> {
        let condition = self.expression()?;

        let _ = self.consume_and_expect(
            TokenType::LeftBrace,
            "Expected '{{' after condition".to_owned(),
        )?;

        let statements = self.block()?;

        Ok(Statement::For(ForStatement {
            condition,
            statements,
        }))
    }

    #[inline]
    fn is_end(&self) -> bool {
        self.next().token_type == TokenType::Eof
    }

    #[inline]
    fn next<'b>(&'b self) -> &'b Token<'a> {
        &self.tokens[self.current_index]
    }

    #[inline]
    fn previous<'b>(&'b self) -> &'b Token<'a> {
        &self.tokens[self.current_index - 1]
    }

    #[inline]
    fn next_matches(&self, token_type: TokenType) -> bool {
        if self.is_end() {
            return false;
        }

        token_type == self.next().token_type
    }

    #[inline]
    fn advance<'b>(&'b mut self) -> &'b Token<'a> {
        if !self.is_end() {
            self.current_index += 1;
        }

        self.previous()
    }

    #[inline]
    fn consumed_one_of<const SIZE: usize>(&mut self, types: [TokenType; SIZE]) -> bool {
        for token_type in types {
            if self.next_matches(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    #[inline]
    fn consume_and_expect<'b>(
        &'b mut self,
        token_type: TokenType,
        error_message: String,
    ) -> Result<&'b Token<'a>, ParserError<'a>> {
        self.advance();
        let previous = self.previous();

        if previous.token_type == token_type {
            return Ok(previous);
        }

        Err(ParserError::new(error_message, previous.clone()))
    }

    #[inline(always)]
    fn lookahead(&self, amount: usize) -> Option<&Token<'a>> {
        self.tokens.get(self.current_index + amount)
    }

    #[inline(always)]
    fn lookahead_many<const AMOUNT: usize>(&self) -> [Option<&Token>; AMOUNT] {
        let mut tokens = [None; AMOUNT];

        for (i, token) in tokens.iter_mut().enumerate().take(AMOUNT) {
            *token = self.lookahead(i);
        }

        tokens
    }

    #[inline(always)]
    fn lookahead_matches_one_of<const SIZE: usize>(
        &self,
        amount: usize,
        types: [TokenType; SIZE],
    ) -> bool {
        let Some(token) = self.lookahead(amount) else {
            return false;
        };

        for token_type in types {
            if token_type == token.token_type {
                return true;
            }
        }

        false
    }
}
