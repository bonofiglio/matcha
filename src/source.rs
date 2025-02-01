use std::str::Chars;

#[derive(Debug)]
pub struct Source<'a> {
    source: &'a str,
    chars: Chars<'a>,
    current_index: usize,
    lexeme_start: usize,
}

impl<'a> Source<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars(),
            current_index: 0,
            lexeme_start: 0,
        }
    }

    pub fn pop_lexeme(&mut self) -> &'a str {
        debug_assert!(self.current_index >= self.lexeme_start);
        debug_assert!(self.lexeme_start <= self.source.len());

        let lexeme = if self.current_index == self.source.len() {
            &self.source[self.lexeme_start - 1..]
        } else if self.lexeme_start == self.current_index {
            &self.source[self.lexeme_start - 1..self.current_index]
        } else {
            &self.source[self.lexeme_start..self.current_index]
        };

        self.lexeme_start = self.current_index;

        lexeme
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
}

impl Iterator for Source<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;
        self.chars.next()
    }
}
