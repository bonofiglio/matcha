use std::{collections::HashMap, fmt::Display};

use crate::token::{Token, TokenLiteral, TokenType};

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
        writeln!(
            f,
            "Scanner error at {}:{}. {}",
            self.line, self.position, self.message
        )
    }
}

impl ScannerError {
    pub fn new(error_type: ScannerErrorType, line: u64, position: u64) -> ScannerError {
        return match error_type {
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
        };
    }
}

pub struct Scanner {}

impl<'a> Scanner {
    pub fn scan(
        source: &'a str,
        keywords: &HashMap<String, TokenType>,
    ) -> Result<Vec<Token>, ScannerError> {
        let mut current_index: usize = 0;
        let mut start_index: usize = 0;
        let mut line: u64 = 0;
        let mut position: u64 = 0;
        let mut tokens = Vec::<Token>::new();

        while !Scanner::eof(source, current_index) {
            start_index = current_index;
            Scanner::scan_token(
                source,
                start_index,
                &mut current_index,
                &mut line,
                &mut position,
                &mut tokens,
                keywords,
            )?;
        }

        Scanner::add_token(
            source,
            start_index,
            current_index,
            line,
            position,
            &mut tokens,
            TokenType::Eof,
            None,
        );

        return Ok(tokens);
    }

    // Helpers:

    fn eof(source: &str, current: usize) -> bool {
        return source.len() <= current;
    }

    fn lookahead(source: &str, current_index: usize) -> char {
        if Scanner::eof(source, current_index) {
            return '\0';
        }

        return source.chars().nth(current_index).unwrap();
    }

    fn add_token(
        source: &str,
        start_index: usize,
        current_index: usize,
        line: u64,
        position: u64,
        tokens: &mut Vec<Token>,
        token_type: TokenType,
        literal: Option<TokenLiteral>,
    ) {
        let lexeme: String = source
            .chars()
            .skip(start_index)
            .take(current_index - start_index)
            .collect();

        tokens.push(Token::new(token_type, lexeme, line, position, literal));
    }

    fn scan_token(
        source: &'a str,
        start_index: usize,
        current_index: &mut usize,
        line: &mut u64,
        position: &mut u64,
        tokens: &mut Vec<Token>,
        keywords: &HashMap<String, TokenType>,
    ) -> Result<(), ScannerError> {
        let c = Scanner::advance(source, current_index, position);

        match c {
            // Single characters
            '(' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::LeftParen,
                None,
            ),
            ')' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::RightParen,
                None,
            ),
            '{' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::LeftBrace,
                None,
            ),
            '}' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::RightBrace,
                None,
            ),
            ',' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::Comma,
                None,
            ),
            '.' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::Dot,
                None,
            ),
            '-' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::Minus,
                None,
            ),
            '+' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::Plus,
                None,
            ),
            ';' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::SemiColon,
                None,
            ),
            '*' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::Star,
                None,
            ),

            // Operators
            '&' => {
                if Scanner::matches_next(source, current_index, position, '&') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::And,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::BitwiseAnd,
                        None,
                    )
                }
            }
            '|' => {
                if Scanner::matches_next(source, current_index, position, '|') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::Or,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::BitwiseOr,
                        None,
                    )
                }
            }
            '!' => {
                if Scanner::matches_next(source, current_index, position, '=') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::BangEqual,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::Bang,
                        None,
                    )
                }
            }
            '=' => {
                if Scanner::matches_next(source, current_index, position, '=') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::DoubleEqual,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::Equal,
                        None,
                    )
                }
            }
            '>' => {
                if Scanner::matches_next(source, current_index, position, '=') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::GreaterEqual,
                        None,
                    )
                } else if Scanner::matches_next(source, current_index, position, '>') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::RightShift,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::Greater,
                        None,
                    )
                }
            }
            '<' => {
                if Scanner::matches_next(source, current_index, position, '=') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::LessEqual,
                        None,
                    )
                } else if Scanner::matches_next(source, current_index, position, '<') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::LeftShift,
                        None,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::Less,
                        None,
                    )
                }
            }
            '^' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::BitwiseXor,
                None,
            ),
            '~' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenType::BitwiseNot,
                None,
            ),
            // Division operator and comments
            '/' => {
                if Scanner::matches_next(source, current_index, position, '/') {
                    // A comment goes until the end of the line.
                    while Scanner::lookahead(source, *current_index) != '\n'
                        && !Scanner::eof(source, *current_index)
                    {
                        // We don't need to know the contents of the comment, so we ignore the value
                        *current_index += 1;
                        *position += 1;
                    }
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenType::Slash,
                        None,
                    );
                };
                return Ok(());
            }

            // Ignore characters without semantic meaning
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => {
                *line += 1;
                *position = 0;
            }

            // String literals
            '"' => {
                return Scanner::string_literal(
                    source,
                    start_index,
                    current_index,
                    line,
                    position,
                    tokens,
                )
            }

            // Number literals
            '0'..='9' => {
                return Scanner::number_literal(
                    source,
                    start_index,
                    current_index,
                    line,
                    position,
                    tokens,
                )
            }

            // Identifier
            'A'..='Z' | 'a'..='z' => Scanner::identifier_or_keyword(
                source,
                start_index,
                current_index,
                line,
                position,
                tokens,
                keywords,
            ),
            _ => {
                return Err(ScannerError::new(
                    ScannerErrorType::UnknownToken,
                    *line,
                    *position,
                ))
            }
        };

        return Ok(());
    }

    fn advance(source: &str, current_index: &mut usize, position: &mut u64) -> char {
        let current = source.chars().nth(*current_index).unwrap();

        *current_index += 1;
        *position += 1;

        return current;
    }

    fn matches_next(
        source: &str,
        current_index: &mut usize,
        position: &mut u64,
        char: char,
    ) -> bool {
        if Scanner::eof(source, *current_index) {
            return false;
        }

        let current = source.chars().nth(*current_index).unwrap();

        if current != char {
            return false;
        }

        // Consume only if it matched the char
        *current_index += 1;
        *position += 1;

        return true;
    }

    // Handlers:

    fn string_literal(
        source: &str,
        start_index: usize,
        current_index: &mut usize,
        line: &mut u64,
        position: &mut u64,
        tokens: &mut Vec<Token>,
    ) -> Result<(), ScannerError> {
        while Scanner::lookahead(source, *current_index) != '"'
            && !Scanner::eof(source, *current_index)
        {
            if Scanner::lookahead(source, *current_index) == '\n' {
                *line += 1;
                *position = 0;
            }
            Scanner::advance(source, current_index, position);
        }

        if Scanner::eof(source, *current_index) {
            return Err(ScannerError::new(
                ScannerErrorType::UnterminatedString,
                *line,
                *position,
            ));
        }

        // Consume the closing quote
        Scanner::advance(source, current_index, position);

        let value: String = source
            .chars()
            .skip(start_index + 1)
            .take(*current_index - start_index - 2)
            .collect();

        Scanner::add_token(
            source,
            start_index,
            *current_index,
            *line,
            *position,
            tokens,
            TokenType::String,
            Some(TokenLiteral::String(value.to_owned())),
        );
        return Ok(());
    }

    fn number_literal(
        source: &str,
        start_index: usize,
        current_index: &mut usize,
        line: &mut u64,
        position: &mut u64,
        tokens: &mut Vec<Token>,
    ) -> Result<(), ScannerError> {
        let mut is_float = false;

        while Scanner::lookahead(source, *current_index).is_ascii_digit() {
            Scanner::advance(source, current_index, position);
        }

        if Scanner::lookahead(source, *current_index) == '.' {
            is_float = true;
            Scanner::advance(source, current_index, position);

            // Expect next character to be a digit after the dot
            if !Scanner::lookahead(source, *current_index).is_ascii_digit() {
                return Err(ScannerError::new(
                    ScannerErrorType::InvalidNumber,
                    *line,
                    *position,
                ));
            }

            while Scanner::lookahead(source, *current_index).is_ascii_digit() {
                Scanner::advance(source, current_index, position);
            }
        }

        let lexeme: String = source
            .chars()
            .skip(start_index)
            .take(*current_index - start_index)
            .collect();

        if is_float {
            match lexeme.parse::<f64>() {
                Err(_) => {
                    return Err(ScannerError::new(
                        ScannerErrorType::InvalidNumber,
                        *line,
                        *position,
                    ))
                }
                Ok(value) => Scanner::add_token(
                    source,
                    start_index,
                    *current_index,
                    *line,
                    *position,
                    tokens,
                    TokenType::Float,
                    Some(TokenLiteral::Float(value)),
                ),
            }
        } else {
            match lexeme.parse::<i64>() {
                Err(_) => {
                    return Err(ScannerError::new(
                        ScannerErrorType::InvalidNumber,
                        *line,
                        *position,
                    ));
                }
                Ok(value) => Scanner::add_token(
                    source,
                    start_index,
                    *current_index,
                    *line,
                    *position,
                    tokens,
                    TokenType::Integer,
                    Some(TokenLiteral::Integer(value)),
                ),
            }
        }
        return Ok(());
    }

    fn identifier_or_keyword(
        source: &str,
        start_index: usize,
        current_index: &mut usize,
        line: &mut u64,
        position: &mut u64,
        tokens: &mut Vec<Token>,
        keywords: &HashMap<String, TokenType>,
    ) {
        while Scanner::lookahead(source, *current_index).is_ascii_alphanumeric() {
            Scanner::advance(source, current_index, position);
        }

        let value: String = source
            .chars()
            .skip(start_index)
            .take(*current_index - start_index)
            .collect();

        // If the value is a known keyword, add the token and return early
        if let Some(keyword) = keywords.get(&value) {
            return Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                keyword.clone(),
                None,
            );
        }

        Scanner::add_token(
            source,
            start_index,
            *current_index,
            *line,
            *position,
            tokens,
            TokenType::Identifier,
            Some(TokenLiteral::String(value)),
        );
    }
}
