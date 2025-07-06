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
