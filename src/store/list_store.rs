use crate::traits::Store::Store;
use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ListStore {
    pub data: VecDeque<String>,
}

impl ListStore {
    pub fn new() -> Self {
        ListStore {
            data: VecDeque::new(),
        }
    }

    pub fn new_with_data(data: VecDeque<String>) -> Self {
        ListStore { data }
    }

    pub fn push_front(&mut self, value: String) {
        self.data.push_front(value);
    }

    pub fn push_back(&mut self, value: String) {
        self.data.push_back(value);
    }

    pub fn pop_front(&mut self) -> Option<String> {
        self.data.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<String> {
        self.data.pop_back()
    }

    pub fn get(&self, index: isize) -> Option<&String> {
        let len = self.data.len() as isize;
        if index >= 0 && index < len {
            self.data.get(index as usize)
        } else if index < 0 && index >= -len {
            self.data.get((len + index) as usize)
        } else {
            None
        }
    }

    pub fn range(&self, start: isize, stop: isize) -> Vec<&String> {
        let len = self.data.len() as isize;
        let start = if start < 0 { len + start } else { start };
        let stop = if stop < 0 { len + stop } else { stop };
        
        let start = start.max(0).min(len) as usize;
        let stop = stop.max(0).min(len) as usize;
        
        if start >= stop {
            return Vec::new();
        }
        
        self.data.range(start..stop).collect()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Store for ListStore {} 