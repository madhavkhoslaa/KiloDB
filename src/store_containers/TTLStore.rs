use crate::traits::Store::Store;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
// keyed by data key now, not TTL seconds. used to be keyed by the TTL
// value itself, so any two keys with the same default TTL (86400) stomped
// on each other's only strong ref and the older one silently went nil
#[derive(Debug)]
pub struct TTLStore {
    pub store: HashMap<String, Arc<RwLock<dyn Store>>>,
}

impl TTLStore {
    pub fn new() -> Self {
        TTLStore {
            store: HashMap::new(),
        }
    }
}
