#![feature(get_type_id)]
//! Hierarchical context

use core::any::Any;
use std::collections::HashMap;

#[cfg(test)]
mod lib_tests;

/// Hierarchical context.
pub struct HContext {
    parent: Option<Box<HContext>>,
    props: HashMap<String, Box<dyn Any + Sync + Send + 'static>>,
}

impl HContext {
    /// Created new empty context.
    pub fn new() -> Self {
        HContext {
            parent: None,
            props: HashMap::new(),
        }
    }

    /// Wraps parent context into new child context. Parent context is consumed.
    pub fn from(parent: HContext) -> Self {
        HContext {
            parent: Some(Box::new(parent)),
            props: HashMap::new(),
        }
    }

    /// Sets propery of current context to provided value. If current context has a parent context
    /// and property name is used also in parent context, it will be hidded by new value but not replaced.
    /// When is parent context unvinded again it will return original property value.
    pub fn with(mut self, key: impl Into<String>, val: impl Any + Sync + Send + 'static) -> Self {
        self.props.insert(key.into(), Box::new(val));

        self
    }

    /// Gets value of property. It recursivelly traverses through chain of parents unit values is found or hierarchy ends
    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        if let Some(v) = self.props.get(key) {
            v.downcast_ref::<T>()
        } else {
            if let Some(p) = &self.parent {
                p.get(key)
            } else {
                None
            }
        }
    }

    /// Retreives parent context from current one. Current context is consumed.
    pub fn unwind(self) -> Option<Self> {
        if let Some(p) = self.parent {
            Some(*p)
        } else {
            None
        }
    }
}
