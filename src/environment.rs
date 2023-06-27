use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::matcha::Value;

pub struct Environment {
    pub values: HashMap<String, Value>,
    pub parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        return Environment {
            values: HashMap::new(),
            parent: None,
        };
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        return Environment {
            values: HashMap::new(),
            parent: Some(parent),
        };
    }
}
