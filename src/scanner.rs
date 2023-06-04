use std::collections::HashMap;

use crate::{
    token::{Token, TokenData},
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
    pub fn scan(source: &'a str, keywords: &HashMap<String, TokenData>) -> Vec<Token> {
        let mut current_index: usize = 0;
        let mut start_index: usize = 0;
        let mut line: u64 = 0;
        let mut position: u64 = 0;
        let mut tokens = Vec::<Token>::new();

        while !Scanner::eof(source, current_index) {
            start_index = current_index;
            let result = Scanner::scan_token(
                source,
                start_index,
                &mut current_index,
                &mut line,
                &mut position,
                &mut tokens,
                keywords,
            );
        }

        Scanner::add_token(
            source,
            start_index,
            current_index,
            line,
            0,
            &mut tokens,
            TokenData::Eof,
        );

        return tokens;
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
        token_data: TokenData,
    ) {
        let lexeme: String = source
            .chars()
            .skip(start_index)
            .take(current_index - start_index)
            .collect();

        tokens.push(Token::new(token_data, lexeme, line, position));
    }

    fn scan_token(
        source: &'a str,
        start_index: usize,
        current_index: &mut usize,
        line: &mut u64,
        position: &mut u64,
        tokens: &mut Vec<Token>,
        keywords: &HashMap<String, TokenData>,
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
                TokenData::LeftParen,
            ),
            ')' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::RightParen,
            ),
            '{' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::LeftBrace,
            ),
            '}' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::RightBrace,
            ),
            ',' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::Comma,
            ),
            '.' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::Dot,
            ),
            '-' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::Minus,
            ),
            '+' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::Plus,
            ),
            ';' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::SemiColon,
            ),
            '*' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::Star,
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
                        TokenData::And,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenData::BitwiseAnd,
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
                        TokenData::Or,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenData::BitwiseOr,
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
                        TokenData::BangEqual,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenData::Bang,
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
                        TokenData::DoubleEqual,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenData::Equal,
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
                        TokenData::GreaterEqual,
                    )
                } else if Scanner::matches_next(source, current_index, position, '>') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenData::RightShift,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenData::Greater,
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
                        TokenData::LessEqual,
                    )
                } else if Scanner::matches_next(source, current_index, position, '<') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenData::LeftShift,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        *position,
                        tokens,
                        TokenData::Less,
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
                TokenData::BitwiseXor,
            ),
            '~' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                *position,
                tokens,
                TokenData::BitwiseNot,
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
                        TokenData::Slash,
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
            TokenData::String(value.to_owned()),
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
                    TokenData::Float(value),
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
                    TokenData::Integer(value),
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
        keywords: &HashMap<String, TokenData>,
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
            );
        }

        Scanner::add_token(
            source,
            start_index,
            *current_index,
            *line,
            *position,
            tokens,
            TokenData::Identifier(value),
        );
    }
}
