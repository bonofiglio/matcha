use std::fmt::Display;

use crate::{
    matcha::{Literal, NumberLiteral, KEYWORDS},
    source::Source,
    token::{Token, TokenType},
};

const UNKNOWN_TOKEN_MESSAGE: &str = "Unknown token";
const UNTERMINATED_STRING_MESSAGE: &str = "Unterminated string";
const INVALID_NUMBER_MESSAGE: &str = "Invalid number";

pub enum ScannerErrorType {
    UnknownToken,
    UnterminatedString,
    InvalidNumber,
}

#[derive(Debug)]
pub struct ScannerError {
    pub message: &'static str,
    pub line: u64,
    pub position: u64,
}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Scanner error at {}:{}. {}",
            self.line, self.position, self.message
        )
    }
}

impl ScannerError {
    pub fn new(error_type: ScannerErrorType, line: u64, position: u64) -> ScannerError {
        match error_type {
            ScannerErrorType::UnknownToken => ScannerError {
                message: UNKNOWN_TOKEN_MESSAGE,
                line,
                position,
            },
            ScannerErrorType::UnterminatedString => ScannerError {
                message: UNTERMINATED_STRING_MESSAGE,
                line,
                position,
            },
            ScannerErrorType::InvalidNumber => ScannerError {
                message: INVALID_NUMBER_MESSAGE,
                line,
                position,
            },
        }
    }
}

pub struct Scanner<'a> {
    pub source: Source<'a>,
}

impl<'a> Scanner<'a> {
    pub fn scan(&mut self) -> Result<Vec<Token<'a>>, ScannerError> {
        let mut line: u64 = 1;
        let mut position: u64 = 0;
        let mut tokens = Vec::<Token<'a>>::new();

        while (Scanner::scan_token(&mut self.source, &mut line, &mut position, &mut tokens)?)
            .is_some()
        {}

        Scanner::add_token("", line, position, &mut tokens, TokenType::Eof, None);

        Ok(tokens)
    }

    // Helpers:

    #[inline]
    fn add_token(
        lexeme: &'a str,
        line: u64,
        position: u64,
        tokens: &mut Vec<Token<'a>>,
        token_type: TokenType,
        literal: Option<Literal<'a>>,
    ) {
        tokens.push(Token::new(token_type, lexeme, line, position, literal));
    }

    #[inline]
    fn scan_token<'b>(
        source: &'b mut Source<'a>,
        line: &mut u64,
        position: &mut u64,
        tokens: &mut Vec<Token<'a>>,
    ) -> Result<Option<()>, ScannerError> {
        let Some(c) = Scanner::advance(source, position) else {
            return Ok(None);
        };

        match c {
            // Single characters
            '(' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::LeftParen,
                None,
            ),
            ')' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::RightParen,
                None,
            ),
            '{' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::LeftBrace,
                None,
            ),
            '}' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::RightBrace,
                None,
            ),
            '[' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::LeftBracket,
                None,
            ),
            ']' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::RightBracket,
                None,
            ),
            ',' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::Comma,
                None,
            ),
            '.' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::Dot,
                None,
            ),
            '-' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::Minus,
                None,
            ),
            '+' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::Plus,
                None,
            ),
            ';' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::SemiColon,
                None,
            ),
            '*' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::Star,
                None,
            ),

            // Operators
            '&' => {
                if Scanner::matches_next(source, position, '&') {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::And,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::BitwiseAnd,
                        None,
                    )
                }
            }
            '|' => {
                if Scanner::matches_next(source, position, '|') {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::Or,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::BitwiseOr,
                        None,
                    )
                }
            }
            '!' => {
                if Scanner::matches_next(source, position, '=') {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::BangEqual,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::Bang,
                        None,
                    )
                }
            }
            '=' => {
                if Scanner::matches_next(source, position, '=') {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::DoubleEqual,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::Equal,
                        None,
                    )
                }
            }
            '>' => {
                if Scanner::matches_next(source, position, '=') {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::GreaterEqual,
                        None,
                    )
                } else if Scanner::matches_next(source, position, '>') {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::RightShift,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::Greater,
                        None,
                    )
                }
            }
            '<' => {
                if Scanner::matches_next(source, position, '=') {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::LessEqual,
                        None,
                    )
                } else if Scanner::matches_next(source, position, '<') {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::LeftShift,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::Less,
                        None,
                    )
                }
            }
            '^' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::BitwiseXor,
                None,
            ),
            '~' => Scanner::add_token(
                source.pop_lexeme(),
                *line,
                *position,
                tokens,
                TokenType::BitwiseNot,
                None,
            ),
            // Division operator and comments
            '/' => {
                if Scanner::matches_next(source, position, '/') {
                    // A comment goes until the end of the line.
                    while let Some(next) = source.peek() {
                        if next == '\n' {
                            break;
                        }

                        source.next();
                        *position += 1;
                    }
                    source.pop_lexeme();
                } else {
                    Scanner::add_token(
                        source.pop_lexeme(),
                        *line,
                        *position,
                        tokens,
                        TokenType::Slash,
                        None,
                    );
                };
            }

            // Ignore characters without semantic meaning
            ' ' | '\r' | '\t' => {
                source.pop_lexeme();
            }
            '\n' => {
                *line += 1;
                *position = 0;
                source.pop_lexeme();
            }

            // String literals
            '"' => {
                return Ok(Some(Scanner::string_literal(
                    source, line, position, tokens,
                )?))
            }

            // Number literals
            '0'..='9' => {
                return Ok(Some(Scanner::number_literal(
                    source, line, position, tokens,
                )?))
            }

            // Identifier
            'A'..='Z' | 'a'..='z' => Scanner::identifier_or_keyword(source, line, position, tokens),
            _ => {
                return Err(ScannerError::new(
                    ScannerErrorType::UnknownToken,
                    *line,
                    *position,
                ))
            }
        };

        Ok(Some(()))
    }

    #[inline]
    fn advance(source: &mut Source, position: &mut u64) -> Option<char> {
        let current = source.next()?;

        *position += 1;

        Some(current)
    }

    #[inline]
    fn matches_next(source: &mut Source, position: &mut u64, expected: char) -> bool {
        match source.peek() {
            Some(current) => {
                if current != expected {
                    false
                } else {
                    *position += 1;
                    source.next();
                    true
                }
            }
            None => false,
        }
    }

    // Handlers:

    #[inline]
    fn string_literal<'b>(
        source: &'b mut Source<'a>,
        line: &mut u64,
        position: &mut u64,
        tokens: &mut Vec<Token<'a>>,
    ) -> Result<(), ScannerError> {
        while let Some(next) = source.peek() {
            if next == '"' {
                break;
            }

            let next = Scanner::advance(source, position).expect("Next must exist");

            if next == '\n' {
                *line += 1;
                *position = 1;
            }
        }

        if source.peek().is_none() {
            return Err(ScannerError::new(
                ScannerErrorType::UnterminatedString,
                *line,
                *position,
            ));
        }

        // Consume the closing quote
        let closing_quote = Scanner::advance(source, position);
        debug_assert_eq!(closing_quote, Some('"'));

        let lexeme = source.pop_lexeme();

        // Must at least include the two quotes
        debug_assert!(lexeme.len() >= 2);

        let value = &lexeme[1..(lexeme.len() - 1)];

        Scanner::add_token(
            lexeme,
            *line,
            *position,
            tokens,
            TokenType::String,
            Some(Literal::String(value)),
        );
        Ok(())
    }

    #[inline]
    fn number_literal<'b>(
        source: &'b mut Source<'a>,
        line: &mut u64,
        position: &mut u64,
        tokens: &mut Vec<Token<'a>>,
    ) -> Result<(), ScannerError> {
        let mut is_float = false;

        while let Some(next) = source.peek() {
            if !next.is_ascii_digit() {
                break;
            }

            Scanner::advance(source, position);
        }

        if source.peek() == Some('.') {
            is_float = true;
            Scanner::advance(source, position);

            // Expect next character to be a digit after the dot
            if !source.peek().is_some_and(|c| c.is_ascii_digit()) {
                return Err(ScannerError::new(
                    ScannerErrorType::InvalidNumber,
                    *line,
                    *position,
                ));
            }

            while source.peek().is_some_and(|c| c.is_ascii_digit()) {
                Scanner::advance(source, position);
            }
        }

        let lexeme = source.pop_lexeme();

        if is_float {
            match lexeme.parse::<f64>() {
                Err(_) => {
                    return Err(ScannerError::new(
                        ScannerErrorType::InvalidNumber,
                        *line,
                        *position,
                    ));
                }
                Ok(value) => Scanner::add_token(
                    lexeme,
                    *line,
                    *position,
                    tokens,
                    TokenType::Float,
                    Some(Literal::Number(NumberLiteral::Float(value))),
                ),
            }
        } else {
            match lexeme.parse::<i32>() {
                Err(_) => {
                    return Err(ScannerError::new(
                        ScannerErrorType::InvalidNumber,
                        *line,
                        *position,
                    ));
                }
                Ok(value) => Scanner::add_token(
                    lexeme,
                    *line,
                    *position,
                    tokens,
                    TokenType::Integer,
                    Some(Literal::Number(NumberLiteral::Integer(value))),
                ),
            }
        }
        Ok(())
    }

    #[inline]
    fn identifier_or_keyword(
        source: &mut Source<'a>,
        line: &mut u64,
        position: &mut u64,
        tokens: &mut Vec<Token<'a>>,
    ) {
        while source.peek().is_some_and(|c| c.is_ascii_alphanumeric()) {
            Scanner::advance(source, position);
        }

        let value = source.pop_lexeme();

        // If the value is a known keyword, add the token and return early
        if let Some(keyword) = KEYWORDS.get(value) {
            let literal = match keyword {
                TokenType::True => Some(Literal::Boolean(true)),
                TokenType::False => Some(Literal::Boolean(false)),
                _ => None,
            };

            return Scanner::add_token(value, *line, *position, tokens, keyword.clone(), literal);
        }

        Scanner::add_token(
            value,
            *line,
            *position,
            tokens,
            TokenType::Identifier,
            Some(Literal::String(value)),
        );
    }
}
