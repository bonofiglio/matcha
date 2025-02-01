use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
    sync::LazyLock,
};

use crate::token::TokenType;

pub static KEYWORDS: LazyLock<HashMap<&str, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        ("struct", TokenType::Struct),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("func", TokenType::Func),
        ("for", TokenType::For),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("let", TokenType::Let),
        ("while", TokenType::While),
    ])
});

#[derive(Debug, Clone, PartialEq)]
pub enum Literal<'a> {
    String(&'a str),
    Number(NumberLiteral),
    Boolean(bool),
}

impl Literal<'_> {
    pub fn get_type(&self) -> &str {
        match self {
            Literal::String(_) => "String",
            Literal::Number(number) => match number {
                NumberLiteral::Float(_) => "Float",
                NumberLiteral::Integer(_) => "Integer",
            },
            Literal::Boolean(_) => "Boolean",
        }
    }
}

impl Display for Literal<'_> {
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
    Integer(i32),
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

impl PartialEq for NumberLiteral {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NumberLiteral::Integer(left), NumberLiteral::Integer(right)) => left == right,
            (NumberLiteral::Float(left), NumberLiteral::Integer(right)) => {
                *left == f64::from(*right)
            }
            (NumberLiteral::Integer(left), NumberLiteral::Float(right)) => {
                f64::from(*left) == *right
            }
            (NumberLiteral::Float(left), NumberLiteral::Float(right)) => left == right,
        }
    }
}

impl Eq for NumberLiteral {}

impl PartialOrd for NumberLiteral {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (NumberLiteral::Integer(left), NumberLiteral::Integer(right)) => Some(left.cmp(right)),
            (NumberLiteral::Float(left), NumberLiteral::Integer(right)) => {
                Some(left.total_cmp(&f64::from(*right)))
            }
            (NumberLiteral::Integer(left), NumberLiteral::Float(right)) => {
                Some(f64::from(*left).total_cmp(right))
            }
            (NumberLiteral::Float(left), NumberLiteral::Float(right)) => {
                Some(left.total_cmp(right))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Empty,
    Optional(Option<Literal<'a>>),
    Literal(Literal<'a>),
}

impl Display for Value<'_> {
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
