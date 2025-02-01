use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::matcha::Value;

#[derive(Debug)]
pub struct Environment<'a> {
    pub values: HashMap<String, Value<'a>>,
    pub parent: Option<Rc<RefCell<Environment<'a>>>>,
}

impl Default for Environment<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
        Environment {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Environment {
        Environment {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }
}
