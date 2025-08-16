use crate::traits::Store::Store;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HashStore {
    fields: HashMap<String, String>,
}

impl HashStore {
    pub fn new() -> Self {
        HashStore {
            fields: HashMap::new(),
        }
    }

    pub fn set_field(&mut self, field: &str, value: &str) -> bool {
        self.fields.insert(field.to_string(), value.to_string()).is_none()
    }

    pub fn get_field(&self, field: &str) -> Option<&String> {
        self.fields.get(field)
    }

    pub fn get_all_fields(&self) -> &HashMap<String, String> {
        &self.fields
    }

    pub fn delete_field(&mut self, field: &str) -> bool {
        self.fields.remove(field).is_some()
    }

    pub fn exists_field(&self, field: &str) -> bool {
        self.fields.contains_key(field)
    }

    pub fn get_keys(&self) -> Vec<String> {
        self.fields.keys().cloned().collect()
    }

    pub fn get_values(&self) -> Vec<String> {
        self.fields.values().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }
}

impl Store for HashStore {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_store_new() {
        let store = HashStore::new();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_set_and_get_field() {
        let mut store = HashStore::new();
        
        // Test setting a new field
        assert!(store.set_field("name", "John"));
        assert_eq!(store.get_field("name"), Some(&"John".to_string()));
        assert_eq!(store.len(), 1);
        
        // Test updating existing field
        assert!(!store.set_field("name", "Jane"));
        assert_eq!(store.get_field("name"), Some(&"Jane".to_string()));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_multiple_fields() {
        let mut store = HashStore::new();
        
        store.set_field("name", "John");
        store.set_field("age", "30");
        store.set_field("city", "New York");
        
        assert_eq!(store.len(), 3);
        assert_eq!(store.get_field("name"), Some(&"John".to_string()));
        assert_eq!(store.get_field("age"), Some(&"30".to_string()));
        assert_eq!(store.get_field("city"), Some(&"New York".to_string()));
    }

    #[test]
    fn test_field_exists() {
        let mut store = HashStore::new();
        
        assert!(!store.exists_field("name"));
        store.set_field("name", "John");
        assert!(store.exists_field("name"));
        assert!(!store.exists_field("age"));
    }

    #[test]
    fn test_delete_field() {
        let mut store = HashStore::new();
        
        store.set_field("name", "John");
        store.set_field("age", "30");
        
        assert!(store.delete_field("name"));
        assert!(!store.exists_field("name"));
        assert_eq!(store.len(), 1);
        
        // Test deleting non-existent field
        assert!(!store.delete_field("nonexistent"));
    }

    #[test]
    fn test_get_keys() {
        let mut store = HashStore::new();
        
        store.set_field("name", "John");
        store.set_field("age", "30");
        
        let mut keys = store.get_keys();
        keys.sort();
        assert_eq!(keys, vec!["age", "name"]);
    }

    #[test]
    fn test_get_values() {
        let mut store = HashStore::new();
        
        store.set_field("name", "John");
        store.set_field("age", "30");
        
        let mut values = store.get_values();
        values.sort();
        assert_eq!(values, vec!["30", "John"]);
    }

    #[test]
    fn test_get_all_fields() {
        let mut store = HashStore::new();
        
        store.set_field("name", "John");
        store.set_field("age", "30");
        
        let all_fields = store.get_all_fields();
        assert_eq!(all_fields.len(), 2);
        assert_eq!(all_fields.get("name"), Some(&"John".to_string()));
        assert_eq!(all_fields.get("age"), Some(&"30".to_string()));
    }
} 