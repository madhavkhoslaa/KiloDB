use crate::traits::Store::Store;
use std::fmt::Debug;

#[derive(Debug)]
pub struct VectorStore {
    values: Vec<String>,
}

impl VectorStore {
    pub fn new() -> Self {
        VectorStore {
            values: Vec::new(),
        }
    }

    pub fn push_left(&mut self, value: &str) -> usize {
        self.values.insert(0, value.to_string());
        self.values.len()
    }

    pub fn push_right(&mut self, value: &str) -> usize {
        self.values.push(value.to_string());
        self.values.len()
    }

    pub fn pop_left(&mut self) -> Option<String> {
        if !self.values.is_empty() {
            Some(self.values.remove(0))
        } else {
            None
        }
    }

    pub fn pop_right(&mut self) -> Option<String> {
        self.values.pop()
    }

    pub fn get_range(&self, start: isize, stop: isize) -> Vec<String> {
        let len = self.values.len() as isize;
        let start_idx = if start < 0 { (len + start).max(0) as usize } else { start.min(len) as usize };
        let stop_idx = if stop < 0 { (len + stop + 1).max(0) as usize } else { (stop + 1).min(len) as usize };
        
        self.values[start_idx..stop_idx].to_vec()
    }

    pub fn get_index(&self, index: isize) -> Option<&String> {
        let len = self.values.len() as isize;
        let idx = if index < 0 { len + index } else { index };
        
        if idx >= 0 && idx < len {
            self.values.get(idx as usize)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl Store for VectorStore {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_store_new() {
        let store = VectorStore::new();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_push_left() {
        let mut store = VectorStore::new();
        
        assert_eq!(store.push_left("first"), 1);
        assert_eq!(store.push_left("second"), 2);
        assert_eq!(store.push_left("third"), 3);
        
        assert_eq!(store.get_range(0, -1), vec!["third", "second", "first"]);
    }

    #[test]
    fn test_push_right() {
        let mut store = VectorStore::new();
        
        assert_eq!(store.push_right("first"), 1);
        assert_eq!(store.push_right("second"), 2);
        assert_eq!(store.push_right("third"), 3);
        
        assert_eq!(store.get_range(0, -1), vec!["first", "second", "third"]);
    }

    #[test]
    fn test_pop_left() {
        let mut store = VectorStore::new();
        
        store.push_right("first");
        store.push_right("second");
        store.push_right("third");
        
        assert_eq!(store.pop_left(), Some("first".to_string()));
        assert_eq!(store.pop_left(), Some("second".to_string()));
        assert_eq!(store.len(), 1);
        
        assert_eq!(store.pop_left(), Some("third".to_string()));
        assert_eq!(store.pop_left(), None);
    }

    #[test]
    fn test_pop_right() {
        let mut store = VectorStore::new();
        
        store.push_right("first");
        store.push_right("second");
        store.push_right("third");
        
        assert_eq!(store.pop_right(), Some("third".to_string()));
        assert_eq!(store.pop_right(), Some("second".to_string()));
        assert_eq!(store.len(), 1);
        
        assert_eq!(store.pop_right(), Some("first".to_string()));
        assert_eq!(store.pop_right(), None);
    }

    #[test]
    fn test_get_range() {
        let mut store = VectorStore::new();
        
        for i in 0..5 {
            store.push_right(&format!("item{}", i));
        }
        
        // Test positive indices
        assert_eq!(store.get_range(0, 2), vec!["item0", "item1", "item2"]);
        assert_eq!(store.get_range(1, 3), vec!["item1", "item2", "item3"]);
        
        // Test negative indices
        assert_eq!(store.get_range(-2, -1), vec!["item3", "item4"]);
        assert_eq!(store.get_range(0, -1), vec!["item0", "item1", "item2", "item3", "item4"]);
        
        // Test out of bounds
        assert_eq!(store.get_range(10, 20), vec![] as Vec<String>);
        assert_eq!(store.get_range(-10, 2), vec!["item0", "item1", "item2"]);
    }

    #[test]
    fn test_get_index() {
        let mut store = VectorStore::new();
        
        store.push_right("first");
        store.push_right("second");
        store.push_right("third");
        
        // Test positive indices
        assert_eq!(store.get_index(0), Some(&"first".to_string()));
        assert_eq!(store.get_index(1), Some(&"second".to_string()));
        assert_eq!(store.get_index(2), Some(&"third".to_string()));
        
        // Test negative indices
        assert_eq!(store.get_index(-1), Some(&"third".to_string()));
        assert_eq!(store.get_index(-2), Some(&"second".to_string()));
        assert_eq!(store.get_index(-3), Some(&"first".to_string()));
        
        // Test out of bounds
        assert_eq!(store.get_index(3), None);
        assert_eq!(store.get_index(-4), None);
    }

    #[test]
    fn test_mixed_operations() {
        let mut store = VectorStore::new();
        
        store.push_right("middle");
        store.push_left("left");
        store.push_right("right");
        
        assert_eq!(store.get_range(0, -1), vec!["left", "middle", "right"]);
        
        assert_eq!(store.pop_left(), Some("left".to_string()));
        assert_eq!(store.pop_right(), Some("right".to_string()));
        assert_eq!(store.get_range(0, -1), vec!["middle"]);
    }
}
