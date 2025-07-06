use crate::traits::Store::Store;
use std::cell::RefCell;
use std::fmt::Debug;
use std::{collections::HashMap, rc::Rc};
#[derive(Debug)]
pub struct TTLStore {
    pub store: HashMap<usize, Rc<RefCell<dyn Store>>>,
}

impl TTLStore {
    pub fn new() -> Self {
        TTLStore {
            store: HashMap::new(),
        }
    }
}
