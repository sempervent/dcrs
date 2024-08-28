// storage/src/storage.rs

use std::collections::HashMap;

pub struct StorageNode {
    pub storage_limit: usize,
    pub content_store: HashMap<String, String>, // Maps content hashes to their storage paths
}

impl StorageNode {
    pub fn new(storage_limit: usize) -> Self {
        StorageNode {
            storage_limit,
            content_store: HashMap::new(),
        }
    }

    pub fn add_content(&mut self, content_hash: String, content_path: String) -> Result<(), String> {
        if self.content_store.len() < self.storage_limit {
            self.content_store.insert(content_hash, content_path);
            Ok(())
        } else {
            Err("Storage limit reached".to_string())
        }
    }

    pub fn get_content(&self, content_hash: &str) -> Option<&String> {
        self.content_store.get(content_hash)
    }

    pub fn sync_content(&mut self, other_node: &StorageNode) {
        for (hash, path) in &other_node.content_store {
            if !self.content_store.contains_key(hash) {
                self.add_content(hash.clone(), path.clone()).ok();
            }
        }
    }
}
