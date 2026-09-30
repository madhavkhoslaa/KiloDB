use crate::traits::Store::Store;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{RwLock, Weak};

#[derive(Debug)]
pub struct DictStore {
    pub store: HashMap<String, Option<Weak<RwLock<dyn Store>>>>,
}
impl DictStore {
    pub fn new() -> Self {
        DictStore {
            store: HashMap::new(),
        }
    }
}
