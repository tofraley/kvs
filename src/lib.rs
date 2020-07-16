#![allow(unused_variables)]

pub struct KvStore;

impl KvStore {
  pub fn new() -> KvStore {
    KvStore {}
  }

  pub fn set(self: &KvStore, key: String, value: String) {
    eprintln!("unimplemented");
  }

  pub fn get(self: &KvStore, key: String) -> Option<String> {
    eprintln!("unimplemented");
    None
  }

  pub fn remove(self: &KvStore, key: String) {
    eprintln!("unimplemented");
  }
}