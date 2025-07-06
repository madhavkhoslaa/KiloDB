use crate::store_containers::{DictStore::DictStore, TTLStore::TTLStore};

pub struct context {
    pub DataBase: DictStore,
    pub TTLStore: TTLStore,
    pub EvictionStore: String, // TODO
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
