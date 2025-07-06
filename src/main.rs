use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
mod store;
mod store_containers;
mod traits;
use std::time::{SystemTime, SystemTimeError, UNIX_EPOCH};
use store::string_store::StringStore;
use store_containers::DictStore::DictStore;
use store_containers::TTLStore::TTLStore;
use traits::Store::Store; // create a LRU/LFU store like this

fn main() -> Result<(), Box<dyn Error>> {
    let mut data = DictStore::new();
    let mut ttl_store = TTLStore::new();
    {
        let shared_store: Rc<RefCell<dyn Store>> =
            Rc::new(RefCell::new(StringStore::new("Madhav".to_owned())));
        data.store
            .insert("Diya".to_owned(), Some(Rc::downgrade(&shared_store)));
        ttl_store.store.insert(
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize,
            shared_store.clone(),
        );
    }
    println!("{:?}", ttl_store);
    println!("{:?}", data);
    ttl_store.store.clear();
    println!("{:?}", ttl_store);
    println!(
        "{:?}",
        data.store.get("Diya").unwrap().as_ref().unwrap().upgrade()
    );
    Ok(())
}
