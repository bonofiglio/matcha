use std::collections::HashMap;

use crate::token::TokenData;

pub struct Vitus {}

impl Vitus {
    pub fn error(line: u64, message: &str) {
        println!("[line: {}] Error: \"{}\"", line, message);
    }

    pub fn keywords() -> HashMap<String, TokenData> {
        return HashMap::from([
            ("class".to_owned(), TokenData::Class),
            ("else".to_owned(), TokenData::Else),
            ("false".to_owned(), TokenData::False),
            ("func".to_owned(), TokenData::Func),
            ("for".to_owned(), TokenData::For),
            ("if".to_owned(), TokenData::If),
            ("nil".to_owned(), TokenData::Nil),
            ("print".to_owned(), TokenData::Print),
            ("return".to_owned(), TokenData::Return),
            ("super".to_owned(), TokenData::Super),
            ("this".to_owned(), TokenData::This),
            ("true".to_owned(), TokenData::True),
            ("var".to_owned(), TokenData::Var),
            ("while".to_owned(), TokenData::While),
        ]);
    }
}
