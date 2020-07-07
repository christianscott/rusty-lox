use crate::token::Literal;

use std::collections::HashMap;

pub struct Environment {
    scopes: Vec<HashMap<String, Option<Literal>>>,
}

impl Environment {
    pub fn new() -> Self {
        let top_level_scope = HashMap::new();
        Self {
            scopes: vec![top_level_scope],
        }
    }

    pub fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, value: Option<Literal>) {
        self.scopes
            .last_mut()
            .expect("always at least one scope")
            .insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Literal) -> Result<(), ()> {
        debug_assert!(
            self.scopes.len() >= 1,
            "there should always be at least one scope"
        );

        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name, Some(value));
                return Ok(());
            }
        }

        Err(())
    }

    pub fn get(&mut self, name: &str) -> Option<Literal> {
        debug_assert!(
            self.scopes.len() >= 1,
            "there should always be at least one scope"
        );

        for scope in self.scopes.iter_mut().rev() {
            let val = scope.get(name);
            if let Some(val) = val {
                // unwrap twice since it may exist but not have been initialized
                return val.clone();
            }
        }

        None
    }
}
