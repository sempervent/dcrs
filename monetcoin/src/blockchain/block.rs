// blockchain/src/block.rs

use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub current_hash: String,
    pub data: String,          // Metadata about the content
    pub content_hash: String,  // Hash of the actual content
    pub storage_refs: Vec<String>, // IPv6 URLs pointing to content storage
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: String, content_hash: String, storage_refs: Vec<String>) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let nonce = 0;
        let current_hash = Block::calculate_hash(index, timestamp, &previous_hash, &data, &content_hash, &storage_refs, nonce);

        Block {
            index,
            timestamp,
            previous_hash,
            current_hash,
            data,
            content_hash,
            storage_refs,
            nonce,
        }
    }

    pub fn calculate_hash(index: u64, timestamp: u128, previous_hash: &str, data: &str, content_hash: &str, storage_refs: &Vec<String>, nonce: u64) -> String {
        let content = format!("{}{}{}{}{}{:?}{}", index, timestamp, previous_hash, data, content_hash, storage_refs, nonce);
        format!("{:x}", md5::compute(content)) // Quick MD5 hash; we'll also add SHA-256 and more later
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.current_hash[..difficulty] != target {
            self.nonce += 1;
            self.current_hash = Block::calculate_hash(self.index, self.timestamp, &self.previous_hash, &self.data, &self.content_hash, &self.storage_refs, self.nonce);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let index = 1;
        let previous_hash = String::from("0000");
        let data = String::from("Test Block");
        let content_hash = String::from("abc123");
        let storage_refs = vec![String::from("::1")];

        let block = Block::new(index, previous_hash.clone(), data.clone(), content_hash.clone(), storage_refs.clone());

        assert_eq!(block.index, index);
        assert_eq!(block.previous_hash, previous_hash);
        assert_eq!(block.data, data);
        assert_eq!(block.content_hash, content_hash);
        assert_eq!(block.storage_refs, storage_refs);
    }

    #[test]
    fn test_block_hashing() {
        let index = 1;
        let previous_hash = String::from("0000");
        let data = String::from("Test Block");
        let content_hash = String::from("abc123");
        let storage_refs = vec![String::from("::1")];

        let block = Block::new(index, previous_hash, data, content_hash, storage_refs);
        let expected_hash = Block::calculate_hash(block.index, block.timestamp, &block.previous_hash, &block.data, &block.content_hash, &block.storage_refs, block.nonce);

        assert_eq!(block.current_hash, expected_hash);
    }

    #[test]
    fn test_block_mining() {
        let index = 1;
        let previous_hash = String::from("0000");
        let data = String::from("Test Block");
        let content_hash = String::from("abc123");
        let storage_refs = vec![String::from("::1")];

        let mut block = Block::new(index, previous_hash, data, content_hash, storage_refs);
        block.mine_block(2); // Mine with difficulty 2

        assert!(block.current_hash.starts_with("00"));
    }
}