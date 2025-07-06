use crate::store_containers::{DictStore::DictStore, TTLStore::TTLStore};

pub struct context {
    DataBase: DictStore,
    TTLStore: TTLStore,
    EvictionStore: String, // TODO
}

impl context {
    pub fn new() -> Self {
        context {
            DataBase: DictStore::new(),
            TTLStore: TTLStore::new(),
            EvictionStore: "TODO".to_owned(),
        }
    }
}
