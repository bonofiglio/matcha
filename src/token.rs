#[derive(Debug, Clone)]
pub enum TokenData {
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

    // Literals
    Identifier(String),
    String(String),
    Integer(i64),
    Float(f64),

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
pub struct Token {
    pub token_data: TokenData,
    pub lexeme: String,
    pub line: u64,
}

impl Token {
    pub fn new(token_data: TokenData, lexeme: String, line: u64) -> Token {
        Token {
            token_data,
            lexeme,
            line,
        }
    }
}
