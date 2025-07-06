use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};

trait Store: Debug {}

#[derive(Debug)]
struct StringStore {
    value: String,
}
impl StringStore {
    fn new(value: String) -> Self {
        StringStore { value }
    }
}
impl Store for StringStore {}

#[derive(Debug)]
struct VectorStore<T> {
    value: Vec<T>,
}
impl<T: Debug> Store for VectorStore<T> {}

#[derive(Debug)]
struct DictStore {
    store: HashMap<String, Weak<RefCell<dyn Store>>>,
}
impl DictStore {
    fn new() -> Self {
        DictStore {
            store: HashMap::new(),
        }
    }

    fn insert(&mut self, key: String, value: &Rc<RefCell<dyn Store>>) {
        self.store.insert(key, Rc::downgrade(value));
    }

    /// Removes any entries whose Weak reference is now invalid
    fn cleanup(&mut self) {
        self.store.retain(|_k, weak| weak.upgrade().is_some());
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

    fn insert(&mut self, key: usize, value: Rc<RefCell<dyn Store>>) {
        self.store.insert(key, value);
    }

    fn remove(&mut self, key: &usize) {
        self.store.remove(key);
    }
}

fn main() -> Result<(), SystemTimeError> {
    let mut dict_store = DictStore::new();
    let mut ttl_store = TTLStore::new();

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;

    let rc_store: Rc<RefCell<dyn Store>> =
        Rc::new(RefCell::new(StringStore::new("Madhav".to_string())));

    dict_store.insert("Diya".to_string(), &rc_store);
    ttl_store.insert(now, rc_store.clone());

    println!("[Before drop]");
    println!("Dict: {:?}", dict_store);
    println!("TTL: {:?}", ttl_store);

    // Simulate TTL expiration
    ttl_store.remove(&now);

    // Cleanup invalid Weak refs
    dict_store.cleanup();

    println!("\n[After drop + cleanup]");
    println!("Dict: {:?}", dict_store);
    println!("TTL: {:?}", ttl_store);
    let mut val = dict_store.store.get("Diya");
    println!("{:?}", val.unwrap().upgrade());
    Ok(())
}
