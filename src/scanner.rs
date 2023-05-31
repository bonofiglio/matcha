use std::collections::HashMap;

use crate::{
    token::{Token, TokenData},
    vitus::Vitus,
};

pub struct Scanner {}

impl Scanner {
    pub fn scan(source: &str, keywords: &HashMap<String, TokenData>) -> Vec<Token> {
        let mut current_index: usize = 0;
        let mut start_index: usize = 0;
        let mut line: u64 = 0;
        let mut tokens = Vec::<Token>::new();

        while !Scanner::eof(source, current_index) {
            start_index = current_index;
            Scanner::scan_token(
                source,
                start_index,
                &mut current_index,
                &mut line,
                &mut tokens,
                keywords,
            );
        }

        Scanner::add_token(
            source,
            start_index,
            current_index,
            line,
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
        tokens: &mut Vec<Token>,
        token_data: TokenData,
    ) {
        let lexeme: String = source
            .chars()
            .skip(start_index)
            .take(current_index - start_index)
            .collect();

        tokens.push(Token::new(token_data, lexeme, line));
    }

    fn scan_token(
        source: &str,
        start_index: usize,
        current_index: &mut usize,
        line: &mut u64,
        tokens: &mut Vec<Token>,
        keywords: &HashMap<String, TokenData>,
    ) {
        let c = Scanner::advance(source, current_index);

        match c {
            // Single characters
            '(' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::LeftParen,
            ),
            ')' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::RightParen,
            ),
            '{' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::LeftBrace,
            ),
            '}' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::RightBrace,
            ),
            ',' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::Comma,
            ),
            '.' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::Dot,
            ),
            '-' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::Minus,
            ),
            '+' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::Plus,
            ),
            ';' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::SemiColon,
            ),
            '*' => Scanner::add_token(
                source,
                start_index,
                *current_index,
                *line,
                tokens,
                TokenData::Star,
            ),

            // Operators
            '&' => {
                if Scanner::matches_next(source, current_index, '&') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::And,
                    )
                } else {
                    Vitus::error(*line, &format!("Unexpected character: {}", c))
                }
            }
            '|' => {
                if Scanner::matches_next(source, current_index, '|') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::Or,
                    )
                } else {
                    Vitus::error(*line, &format!("Unexpected character: {}", c))
                }
            }
            '!' => {
                if Scanner::matches_next(source, current_index, '=') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::BangEqual,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::Bang,
                    )
                }
            }
            '=' => {
                if Scanner::matches_next(source, current_index, '=') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::DoubleEqual,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::Equal,
                    )
                }
            }
            '>' => {
                if Scanner::matches_next(source, current_index, '=') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::GreaterEqual,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::Greater,
                    )
                }
            }
            '<' => {
                if Scanner::matches_next(source, current_index, '=') {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::LessEqual,
                    )
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::Less,
                    )
                }
            }
            // Division operator and comments
            '/' => {
                return if Scanner::matches_next(source, current_index, '/') {
                    // A comment goes until the end of the line.
                    while Scanner::lookahead(source, *current_index) != '\n'
                        && !Scanner::eof(source, *current_index)
                    {
                        // We don't need to know the contents of the comment, so we ignore the value
                        *current_index += 1;
                    }
                } else {
                    Scanner::add_token(
                        source,
                        start_index,
                        *current_index,
                        *line,
                        tokens,
                        TokenData::Slash,
                    );
                };
            }

            // Ignore characters without semantic meaning
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => {
                *line += 1;
            }

            // String literals
            '"' => Scanner::string_literal(source, start_index, current_index, line, tokens),

            // Number literals
            '0'..='9' => Scanner::number_literal(source, start_index, current_index, line, tokens),

            // Identifier
            'A'..='Z' | 'a'..='z' => Scanner::identifier_or_keyword(
                source,
                start_index,
                current_index,
                line,
                tokens,
                keywords,
            ),
            _ => Vitus::error(*line, &format!("Unexpected character: {}", c)),
        };
    }

    fn advance(source: &str, current_index: &mut usize) -> char {
        let current = source.chars().nth(*current_index).unwrap();

        *current_index += 1;

        return current;
    }

    fn matches_next(source: &str, current_index: &mut usize, char: char) -> bool {
        if Scanner::eof(source, *current_index) {
            return false;
        }

        let current = source.chars().nth(*current_index).unwrap();

        if current != char {
            return false;
        }

        // Consume only if it matched the char
        *current_index += 1;

        return true;
    }

    // Handlers:

    fn string_literal(
        source: &str,
        start_index: usize,
        current_index: &mut usize,
        line: &mut u64,
        tokens: &mut Vec<Token>,
    ) {
        while Scanner::lookahead(source, *current_index) != '"'
            && !Scanner::eof(source, *current_index)
        {
            if Scanner::lookahead(source, *current_index) == '\n' {
                *line += 1;
            }
            Scanner::advance(source, current_index);
        }

        if Scanner::eof(source, *current_index) {
            Vitus::error(*line, "Unterminated string");
            return;
        }

        // Consume the closing quote
        Scanner::advance(source, current_index);

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
            tokens,
            TokenData::String(value.to_owned()),
        );
    }

    fn number_literal(
        source: &str,
        start_index: usize,
        current_index: &mut usize,
        line: &mut u64,
        tokens: &mut Vec<Token>,
    ) {
        let mut is_float = false;

        while Scanner::lookahead(source, *current_index).is_ascii_digit() {
            Scanner::advance(source, current_index);
        }

        if Scanner::lookahead(source, *current_index) == '.' {
            is_float = true;
            Scanner::advance(source, current_index);

            // Expect next character to be a digit after the dot
            if !Scanner::lookahead(source, *current_index).is_ascii_digit() {
                Vitus::error(*line, "Invalid number");
                return;
            }

            while Scanner::lookahead(source, *current_index).is_ascii_digit() {
                Scanner::advance(source, current_index);
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
                    return Vitus::error(*line, "Invalid number (parsing failed)");
                }
                Ok(value) => Scanner::add_token(
                    source,
                    start_index,
                    *current_index,
                    *line,
                    tokens,
                    TokenData::Float(value),
                ),
            }
        } else {
            match lexeme.parse::<i64>() {
                Err(_) => {
                    return Vitus::error(*line, "Invalid number (parsing failed)");
                }
                Ok(value) => Scanner::add_token(
                    source,
                    start_index,
                    *current_index,
                    *line,
                    tokens,
                    TokenData::Integer(value),
                ),
            }
        }
    }

    fn identifier_or_keyword(
        source: &str,
        start_index: usize,
        current_index: &mut usize,
        line: &mut u64,
        tokens: &mut Vec<Token>,
        keywords: &HashMap<String, TokenData>,
    ) {
        while Scanner::lookahead(source, *current_index).is_ascii_alphanumeric() {
            Scanner::advance(source, current_index);
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
                tokens,
                keyword.clone(),
            );
        }

        Scanner::add_token(
            source,
            start_index,
            *current_index,
            *line,
            tokens,
            TokenData::Identifier(value),
        );
    }
}
