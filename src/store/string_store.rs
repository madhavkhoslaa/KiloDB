use crate::traits::Store::Store;
#[derive(Debug)]
pub struct StringStore {
    value: String,
}
impl StringStore {
    pub fn new(value: String) -> Self {
        StringStore { value: value }
    }
    pub fn get_value(&self) -> &String {
        &self.value
    }
}

impl Store for StringStore {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_store_new() {
        let store = StringStore::new("test_value".to_string());
        assert_eq!(store.get_value(), "test_value");
    }

    #[test]
    fn test_string_store_empty() {
        let store = StringStore::new("".to_string());
        assert_eq!(store.get_value(), "");
    }

    #[test]
    fn test_string_store_unicode() {
        let store = StringStore::new("hello 世界".to_string());
        assert_eq!(store.get_value(), "hello 世界");
    }
}
