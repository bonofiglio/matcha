use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use once_cell::sync::Lazy;

use crate::token::TokenType;

pub static KEYWORDS: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    HashMap::from([
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
    ])
});

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(NumberLiteral),
    Boolean(bool),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Boolean(bool) => write!(f, "{}", if *bool { "true" } else { "false" }),
            Literal::Number(num) => write!(f, "{}", num),
        }
    }
}

#[derive(Debug, Clone)]
pub enum NumberLiteral {
    Float(f64),
    Integer(i64),
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberLiteral::Float(float) => write!(f, "{}", float),
            NumberLiteral::Integer(integer) => write!(f, "{}", integer),
        }
    }
}

impl Add for NumberLiteral {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (NumberLiteral::Integer(left), NumberLiteral::Integer(right)) => {
                NumberLiteral::Integer(left + right)
            }
            (NumberLiteral::Float(left), NumberLiteral::Integer(right)) => {
                NumberLiteral::Float(left + (right as f64))
            }
            (NumberLiteral::Integer(left), NumberLiteral::Float(right)) => {
                NumberLiteral::Float((left as f64) + right)
            }
            (NumberLiteral::Float(left), NumberLiteral::Float(right)) => {
                NumberLiteral::Float(left + right)
            }
        }
    }
}

impl Sub for NumberLiteral {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (NumberLiteral::Integer(left), NumberLiteral::Integer(right)) => {
                NumberLiteral::Integer(left - right)
            }
            (NumberLiteral::Float(left), NumberLiteral::Integer(right)) => {
                NumberLiteral::Float(left - (right as f64))
            }
            (NumberLiteral::Integer(left), NumberLiteral::Float(right)) => {
                NumberLiteral::Float((left as f64) - right)
            }
            (NumberLiteral::Float(left), NumberLiteral::Float(right)) => {
                NumberLiteral::Float(left - right)
            }
        }
    }
}

impl Mul for NumberLiteral {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (NumberLiteral::Integer(left), NumberLiteral::Integer(right)) => {
                NumberLiteral::Integer(left * right)
            }
            (NumberLiteral::Float(left), NumberLiteral::Integer(right)) => {
                NumberLiteral::Float(left * (right as f64))
            }
            (NumberLiteral::Integer(left), NumberLiteral::Float(right)) => {
                NumberLiteral::Float((left as f64) * right)
            }
            (NumberLiteral::Float(left), NumberLiteral::Float(right)) => {
                NumberLiteral::Float(left * right)
            }
        }
    }
}

impl Div for NumberLiteral {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (NumberLiteral::Integer(left), NumberLiteral::Integer(right)) => {
                NumberLiteral::Integer(left / right)
            }
            (NumberLiteral::Float(left), NumberLiteral::Integer(right)) => {
                NumberLiteral::Float(left / (right as f64))
            }
            (NumberLiteral::Integer(left), NumberLiteral::Float(right)) => {
                NumberLiteral::Float((left as f64) / right)
            }
            (NumberLiteral::Float(left), NumberLiteral::Float(right)) => {
                NumberLiteral::Float(left / right)
            }
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Empty,
    Optional(Option<Literal>),
    Literal(Literal),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Empty => write!(f, "<empty>"),
            Value::Optional(optional_literal) => match optional_literal {
                None => write!(f, "None"),
                Some(literal) => write!(f, "{}", literal),
            },
            Value::Literal(literal) => write!(f, "{}", literal),
        }
    }
}
