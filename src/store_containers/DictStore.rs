use crate::traits::Store::Store;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Mutex, Weak};

#[derive(Debug)]
pub struct DictStore {
    pub store: HashMap<String, Option<Weak<Mutex<dyn Store>>>>,
}
impl DictStore {
    pub fn new() -> Self {
        DictStore {
            store: HashMap::new(),
        }
    }
}
