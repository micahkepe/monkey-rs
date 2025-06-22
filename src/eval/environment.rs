//! This module defines a programming environment within Monkey.
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::object::Object;

/// Type alias for shared, interior-mutable environment.
pub type Env = Rc<RefCell<Environment>>;

/// A wrapper around the stored values obtained during evaluation.
#[derive(Debug, Default)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
}

impl Environment {
    /// Construct a new blank environment.
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
        }
    }

    /// Retrieves the value associated with a key, if it exists.
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        self.store.get(name).map(Rc::clone)
    }

    /// Sets the value for a given key. If the key is already present in the
    /// environment, its value is updated.
    pub fn set(&mut self, name: &str, val: Rc<Object>) {
        self.store.insert(name.to_string(), val);
    }
}
