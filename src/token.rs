use crate::matcha::Literal;

#[derive(Debug, Clone, PartialEq)]
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

    // Literals
    Identifier,
    String,
    Integer,
    Float,

    // Reserved keywords
    Struct,
    Else,
    False,
    Func,
    For,
    If,
    Nil,
    Return,
    Super,
    This,
    True,
    Let,
    While,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub line: u64,
    pub position: u64,
    pub literal: Option<Literal<'a>>,
}

impl<'a> Token<'a> {
    #[inline]
    pub const fn new(
        token_type: TokenType,
        lexeme: &'a str,
        line: u64,
        position: u64,
        literal: Option<Literal<'a>>,
    ) -> Token<'a> {
        Token {
            token_type,
            lexeme,
            line,
            position,
            literal,
        }
    }
}
