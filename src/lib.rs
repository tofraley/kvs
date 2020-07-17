#![allow(unused_variables)]
use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    kv: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore { kv: HashMap::new() }
    }

    pub fn set(self: &mut KvStore, key: String, value: String) -> Option<String> {
        self.kv.insert(key, value)
    }

    pub fn get(self: &KvStore, key: String) -> Option<&String> {
        self.kv.get(&key)
    }

    pub fn remove(self: &mut KvStore, key: &str) -> Option<String> {
        self.kv.remove(key)
    }
}
