use std::collections::HashMap;

use crate::token::TokenType;

pub struct Vitus {}

impl<'a> Vitus {
    pub fn keywords() -> HashMap<String, TokenType> {
        return HashMap::from([
            ("class".to_owned(), TokenType::Class),
            ("else".to_owned(), TokenType::Else),
            ("false".to_owned(), TokenType::False),
            ("func".to_owned(), TokenType::Func),
            ("for".to_owned(), TokenType::For),
            ("if".to_owned(), TokenType::If),
            ("nil".to_owned(), TokenType::Nil),
            ("print".to_owned(), TokenType::Print),
            ("return".to_owned(), TokenType::Return),
            ("super".to_owned(), TokenType::Super),
            ("this".to_owned(), TokenType::This),
            ("true".to_owned(), TokenType::True),
            ("var".to_owned(), TokenType::Var),
            ("while".to_owned(), TokenType::While),
        ]);
    }
}
