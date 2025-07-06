use std::cell::RefCell;
use std::error::Error;
use std::fmt::Debug;
use std::rc::Weak;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use std::{collections::HashMap, rc::Rc};
trait Store: Debug {}

#[derive(Debug)]
struct StringStore {
    value: String,
}
impl StringStore {
    fn new(value: String) -> Self {
        StringStore { value: value }
    }
}

#[derive(Debug)]
struct VectorStore<T> {
    value: Vec<T>,
}

impl<T: Debug> Store for VectorStore<T> {}
impl Store for StringStore {}

#[derive(Debug)]
struct DictStore {
    store: HashMap<String, Option<Weak<RefCell<dyn Store>>>>,
}
impl DictStore {
    fn new() -> Self {
        DictStore {
            store: HashMap::new(),
        }
    }
}

#[derive(Debug)]
struct TTLStore {
    store: HashMap<usize, Rc<RefCell<dyn Store>>>,
}

impl TTLStore {
    fn new() -> Self {
        TTLStore {
            store: HashMap::new(),
        }
    }
}
fn main() -> Result<(), SystemTimeError> {
    let mut data = DictStore::new();
    let mut ttl_store = TTLStore::new();
    let shared_store: Rc<RefCell<dyn Store>> =
        Rc::new(RefCell::new(StringStore::new("Madhav".to_owned())));
    data.store
        .insert("Diya".to_owned(), Some(Rc::downgrade(&shared_store)));
    ttl_store.store.insert(
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize,
        shared_store.clone(),
    );
    println!("{:?}", ttl_store);
    println!("{:?}", data);
    Ok(())
}
