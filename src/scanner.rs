use std::collections::HashMap;

use crate::{
    token::{Token, TokenData},
    vitus::Vitus,
};

pub struct Scanner<'a> {
    source: String,
    keywords: &'a HashMap<String, TokenData>,
    current: usize,
    start: usize,
    line: u64,
    pub tokens: Vec<Token>,
}

impl Scanner<'_> {
    pub fn new(source: String, keywords: &HashMap<String, TokenData>) -> Scanner {
        return Scanner {
            source,
            keywords,
            current: 0,
            start: 0,
            line: 0,
            tokens: vec![],
        };
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenData::Eof);

        // Restart the scanner to its original state
        self.start = 0;
        self.current = 0;
        self.line = 0;

        return &self.tokens;
    }

    // Helpers:

    fn eof(&self) -> bool {
        return self.source.len() <= self.current;
    }

    pub fn lookahead(&self) -> char {
        if self.eof() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap();
    }

    fn add_token(&mut self, token_data: TokenData) {
        let lexeme: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        self.tokens.push(Token::new(token_data, lexeme, self.line));
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            // Single characters
            '(' => self.add_token(TokenData::LeftParen),
            ')' => self.add_token(TokenData::RightParen),
            '{' => self.add_token(TokenData::LeftBrace),
            '}' => self.add_token(TokenData::RightBrace),
            ',' => self.add_token(TokenData::Comma),
            '.' => self.add_token(TokenData::Dot),
            '-' => self.add_token(TokenData::Minus),
            '+' => self.add_token(TokenData::Plus),
            ';' => self.add_token(TokenData::SemiColon),
            '*' => self.add_token(TokenData::Star),

            // Operators
            '&' => {
                if self.matches_next('&') {
                    self.add_token(TokenData::And)
                } else {
                    Vitus::error(self.line, &format!("Unexpected character: {}", c))
                }
            }
            '|' => {
                if self.matches_next('|') {
                    self.add_token(TokenData::Or)
                } else {
                    Vitus::error(self.line, &format!("Unexpected character: {}", c))
                }
            }
            '!' => {
                if self.matches_next('=') {
                    self.add_token(TokenData::BangEqual)
                } else {
                    self.add_token(TokenData::Bang)
                }
            }
            '=' => {
                if self.matches_next('=') {
                    self.add_token(TokenData::DoubleEqual)
                } else {
                    self.add_token(TokenData::Equal)
                }
            }
            '>' => {
                if self.matches_next('=') {
                    self.add_token(TokenData::GreaterEqual)
                } else {
                    self.add_token(TokenData::Greater)
                }
            }
            '<' => {
                if self.matches_next('=') {
                    self.add_token(TokenData::LessEqual)
                } else {
                    self.add_token(TokenData::Less)
                }
            }
            // Division operator and comments
            '/' => {
                return if self.matches_next('/') {
                    // A comment goes until the end of the line.
                    while self.lookahead() != '\n' && !self.eof() {
                        // We don't need to know the contents of the comment, so we ignore the value
                        self.current += 1;
                    }
                } else {
                    self.add_token(TokenData::Slash);
                };
            }

            // Ignore characters without semantic meaning
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => {
                self.line += 1;
            }

            // String literals
            '"' => self.string_literal(),

            // Number literals
            '0'..='9' => self.number_literal(),

            // Identifier
            'A'..='Z' | 'a'..='z' => self.identifier_or_keyword(),
            _ => Vitus::error(self.line, &format!("Unexpected character: {}", c)),
        };
    }

    fn advance(&mut self) -> char {
        let current = self.source.chars().nth(self.current).unwrap();

        self.current += 1;

        return current;
    }

    fn matches_next(&mut self, char: char) -> bool {
        if self.eof() {
            return false;
        }

        let current = self.source.chars().nth(self.current).unwrap();

        if current != char {
            return false;
        }

        // Consume only if it matched the char
        self.current += 1;

        return true;
    }

    // Handlers:

    fn string_literal(&mut self) {
        while self.lookahead() != '"' && !self.eof() {
            if self.lookahead() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.eof() {
            Vitus::error(self.line, "Unterminated string");
            return;
        }

        // Consume the closing quote
        self.advance();

        let value: String = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - self.start - 2)
            .collect();

        self.add_token(TokenData::String(value.to_owned()));
    }

    fn number_literal(&mut self) {
        let mut is_float = false;

        while self.lookahead().is_ascii_digit() {
            self.advance();
        }

        if self.lookahead() == '.' {
            is_float = true;
            self.advance();

            // Expect next character to be a digit after the dot
            if !self.lookahead().is_ascii_digit() {
                Vitus::error(self.line, "Invalid number");
                return;
            }

            while self.lookahead().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        if is_float {
            match lexeme.parse::<f64>() {
                Err(_) => {
                    return Vitus::error(self.line, "Invalid number (parsing failed)");
                }
                Ok(value) => self.add_token(TokenData::Float(value)),
            }
        } else {
            match lexeme.parse::<i64>() {
                Err(_) => {
                    return Vitus::error(self.line, "Invalid number (parsing failed)");
                }
                Ok(value) => self.add_token(TokenData::Integer(value)),
            }
        }
    }

    fn identifier_or_keyword(&mut self) {
        while self.lookahead().is_ascii_alphanumeric() {
            self.advance();
        }

        let value: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        // If the value is a known keyword, add the token and return early
        if let Some(keyword) = self.keywords.get(&value) {
            return self.add_token(keyword.clone());
        }

        self.add_token(TokenData::Identifier(value));
    }
}
