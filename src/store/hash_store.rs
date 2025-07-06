use crate::traits::Store::Store;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct HashStore {
    pub data: HashMap<String, String>,
}

impl HashStore {
    pub fn new() -> Self {
        HashStore {
            data: HashMap::new(),
        }
    }

    pub fn new_with_data(data: HashMap<String, String>) -> Self {
        HashStore { data }
    }

    pub fn get(&self, field: &str) -> Option<&String> {
        self.data.get(field)
    }

    pub fn set(&mut self, field: String, value: String) {
        self.data.insert(field, value);
    }

    pub fn delete(&mut self, field: &str) -> bool {
        self.data.remove(field).is_some()
    }

    pub fn exists(&self, field: &str) -> bool {
        self.data.contains_key(field)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn get_all(&self) -> &HashMap<String, String> {
        &self.data
    }
}

impl Store for HashStore {} 