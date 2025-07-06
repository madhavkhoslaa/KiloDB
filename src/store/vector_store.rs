use crate::traits::Store::Store;
use std::fmt::Debug;

#[derive(Debug)]
struct VectorStore<T> {
    value: Vec<T>,
}

impl<T: Debug> Store for VectorStore<T> {}
