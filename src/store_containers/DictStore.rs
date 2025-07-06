use crate::traits::Store::Store;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Weak;

#[derive(Debug)]
pub struct DictStore {
    pub store: HashMap<String, Option<Weak<RefCell<dyn Store>>>>,
}
impl DictStore {
    pub fn new() -> Self {
        DictStore {
            store: HashMap::new(),
        }
    }
}
