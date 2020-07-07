use crate::token::{Literal, Range};

// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::rc::Rc;

pub struct Scope {}

impl Scope {
    pub fn new() -> Self {
        Self {}
    }

    pub fn define(&self, var: Range, value: Option<Literal>) {}

    pub fn assign(&self, var: Range, value: Literal) -> Result<(), UndefinedVariable> {
        Ok(())
    }

    pub fn get(&self, var: Range) -> Option<Literal> {
        None
    }
}

pub struct UndefinedVariable;
