#![deny(missing_docs)]
//! A simple key/value store

use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/Value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val,Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    inner: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.inner.insert(key, value);
    }
    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Option<String> {
        self.inner.get(&key).cloned()
    }
    /// Remove a given key
    pub fn remove(&mut self, key: String) {
        self.inner.remove(&key);
    }
}
