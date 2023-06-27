use std::collections::HashMap;

use crate::matcha::Value;

pub struct Environment<'a> {
    pub values: HashMap<String, Value>,
    pub parent: Option<&'a Environment<'a>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        return Environment {
            values: HashMap::new(),
            parent: None,
        };
    }

    pub fn with_parent(parent: &'a Environment) -> Environment<'a> {
        return Environment {
            values: HashMap::new(),
            parent: Some(parent),
        };
    }
}
