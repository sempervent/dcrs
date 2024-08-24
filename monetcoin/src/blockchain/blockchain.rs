// blockchain/src/blockchain.rs

use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
    pub peers: Vec<String>,  // List of peer nodes
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            difficulty,
            peers: Vec::new(),
        };
        blockchain.add_genesis_block();
        blockchain
    }

    fn add_genesis_block(&mut self) {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"), String::from("0"), vec![]);
        self.blocks.push(genesis_block);
    }

    pub fn add_block(&mut self, data: String, content_hash: String, storage_refs: Vec<String>) {
        let previous_hash = self.blocks.last().map_or("0".to_string(), |block| block.current_hash.clone());
        let mut block = Block::new(self.blocks.len() as u64, previous_hash, data, content_hash, storage_refs);

        // Mining the block (proof-of-work)
        block.mine_block(self.difficulty);

        // Check if the hash already exists in the blockchain
        if self.is_hash_unique(&block.current_hash) {
            self.blocks.push(block);
            println!("Block added successfully.");
        } else {
            println!("Block with the same hash already exists. Block not added.");
        }
    }

    fn is_hash_unique(&self, hash: &str) -> bool {
        for block in &self.blocks {
            if block.current_hash == hash {
                return false;
            }
        }
        true
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let previous_block = &self.blocks[i - 1];
            let current_block = &self.blocks[i];

            if current_block.current_hash != Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.previous_hash,
                &current_block.data,
                &current_block.content_hash,
                &current_block.storage_refs,
                current_block.nonce,
            ) {
                return false;
            }

            if current_block.previous_hash != previous_block.current_hash {
                return false;
            }
        }
        true
    }
}
