//! This module defines a programming environment within Monkey.
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::eval::object::Object;

/// Type alias for shared, interior-mutable environment.
pub type Env = Rc<RefCell<Environment>>;

/// A wrapper around the stored values obtained during evaluation.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Environment {
    store: HashMap<String, Rc<Object>>,
    /// Outer/ enclosing environment that is being extended by the Environment
    /// instance.
    outer: Option<Env>,
}

impl Environment {
    /// Construct a new blank environment.
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    /// Constructs a new enclosed environment within the outer environment.
    pub fn new_enclosed_environment(outer: &Env) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(outer)),
        }
    }

    /// Retrieves the value associated with a key, if it exists.
    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        match self.store.get(name) {
            Some(obj) => Some(Rc::clone(obj)),
            None => {
                // Check the enclosing environment as well, if it exists.
                if let Some(outer) = &self.outer {
                    outer.borrow().get(name)
                } else {
                    None
                }
            }
        }
    }

    /// Sets the value for a given key. If the key is already present in the
    /// environment, its value is updated.
    pub fn set(&mut self, name: &str, val: Rc<Object>) {
        self.store.insert(name.to_string(), val);
    }
}
