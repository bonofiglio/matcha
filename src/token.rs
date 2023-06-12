#[derive(Debug, Clone)]
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
    Class,
    Else,
    False,
    Func,
    For,
    If,
    Nil,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub enum TokenLiteral {
    String(String),
    Float(f64),
    Integer(i64),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u64,
    pub position: u64,
    pub literal: Option<TokenLiteral>,
}

impl Token {
    pub const fn new(
        token_type: TokenType,
        lexeme: String,
        line: u64,
        position: u64,
        literal: Option<TokenLiteral>,
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
