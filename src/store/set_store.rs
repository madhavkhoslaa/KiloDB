use crate::traits::Store::Store;
use std::collections::HashSet;

#[derive(Debug)]
pub struct SetStore {
    members: HashSet<String>,
}

impl SetStore {
    pub fn new() -> Self {
        SetStore {
            members: HashSet::new(),
        }
    }

    pub fn add_member(&mut self, member: &str) -> bool {
        self.members.insert(member.to_string())
    }

    pub fn remove_member(&mut self, member: &str) -> bool {
        self.members.remove(member)
    }

    pub fn is_member(&self, member: &str) -> bool {
        self.members.contains(member)
    }

    pub fn get_members(&self) -> Vec<String> {
        self.members.iter().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.members.len()
    }
}

impl Store for SetStore {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_store_new() {
        let store = SetStore::new();
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_add_member() {
        let mut store = SetStore::new();
        
        // Test adding new member
        assert!(store.add_member("member1"));
        assert_eq!(store.len(), 1);
        assert!(store.is_member("member1"));
        
        // Test adding duplicate member
        assert!(!store.add_member("member1"));
        assert_eq!(store.len(), 1);
        
        // Test adding another member
        assert!(store.add_member("member2"));
        assert_eq!(store.len(), 2);
    }

    #[test]
    fn test_remove_member() {
        let mut store = SetStore::new();
        
        store.add_member("member1");
        store.add_member("member2");
        
        // Test removing existing member
        assert!(store.remove_member("member1"));
        assert_eq!(store.len(), 1);
        assert!(!store.is_member("member1"));
        assert!(store.is_member("member2"));
        
        // Test removing non-existent member
        assert!(!store.remove_member("member3"));
        assert_eq!(store.len(), 1);
    }

    #[test]
    fn test_is_member() {
        let mut store = SetStore::new();
        
        assert!(!store.is_member("member1"));
        
        store.add_member("member1");
        assert!(store.is_member("member1"));
        assert!(!store.is_member("member2"));
        
        store.remove_member("member1");
        assert!(!store.is_member("member1"));
    }

    #[test]
    fn test_get_members() {
        let mut store = SetStore::new();
        
        assert_eq!(store.get_members().len(), 0);
        
        store.add_member("member1");
        store.add_member("member2");
        store.add_member("member3");
        
        let mut members = store.get_members();
        members.sort();
        assert_eq!(members, vec!["member1", "member2", "member3"]);
    }

    #[test]
    fn test_set_uniqueness() {
        let mut store = SetStore::new();
        
        store.add_member("duplicate");
        store.add_member("duplicate");
        store.add_member("duplicate");
        
        assert_eq!(store.len(), 1);
        assert_eq!(store.get_members(), vec!["duplicate"]);
    }

    #[test]
    fn test_empty_string_member() {
        let mut store = SetStore::new();
        
        assert!(store.add_member(""));
        assert!(store.is_member(""));
        assert_eq!(store.len(), 1);
        
        assert!(store.remove_member(""));
        assert!(!store.is_member(""));
        assert_eq!(store.len(), 0);
    }

    #[test]
    fn test_unicode_members() {
        let mut store = SetStore::new();
        
        store.add_member("hello");
        store.add_member("世界");
        store.add_member("🦀");
        
        assert_eq!(store.len(), 3);
        assert!(store.is_member("世界"));
        assert!(store.is_member("🦀"));
    }
} 