#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Colon,
    SemiColon,
    Slash,
    Star,
    BitwiseNot,

    // Multiple characters
    Bang,
    BangEqual,
    Equal,
    DoubleEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    LeftShift,
    RightShift,
    VarDec,

    // Literals
    Identifier,
    String,
    Integer,
    Float,

    // Reserved keywords
    If,
    Else,
    True,
    False,
    For,

    Eof,
}

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: u64,
    pub position: u64,
}

impl<'a> Token<'a> {
    #[inline]
    pub const fn new(
        token_type: TokenType,
        lexeme: &'a str,
        line: u64,
        position: u64,
    ) -> Token<'a> {
        Token {
            token_type,
            lexeme,
            line,
            position,
        }
    }
}
