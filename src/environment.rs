use std::collections::HashMap;

use crate::matcha::Value;

pub struct Environment {
    pub values: HashMap<String, Value>,
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        return Environment {
            values: HashMap::new(),
            parent: None,
        };
    }

    pub fn with_parent(parent: Box<Environment>) -> Environment {
        return Environment {
            values: HashMap::new(),
            parent: Some(parent),
        };
    }
}
