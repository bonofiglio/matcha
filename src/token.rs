use crate::matcha::Literal;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single character
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u64,
    pub position: u64,
    pub literal: Option<Literal>,
}

impl Token {
    pub const fn new(
        token_type: TokenType,
        lexeme: String,
        line: u64,
        position: u64,
        literal: Option<Literal>,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            line,
            position,
            literal,
        }
    }
}
