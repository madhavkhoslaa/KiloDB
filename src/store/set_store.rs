use crate::traits::Store::Store;
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct SetStore {
    pub data: HashSet<String>,
}

impl SetStore {
    pub fn new() -> Self {
        SetStore {
            data: HashSet::new(),
        }
    }

    pub fn new_with_data(data: HashSet<String>) -> Self {
        SetStore { data }
    }

    pub fn add(&mut self, member: String) -> bool {
        self.data.insert(member)
    }

    pub fn remove(&mut self, member: &str) -> bool {
        self.data.remove(member)
    }

    pub fn contains(&self, member: &str) -> bool {
        self.data.contains(member)
    }

    pub fn members(&self) -> Vec<&String> {
        self.data.iter().collect()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Store for SetStore {} 