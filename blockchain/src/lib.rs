use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)] // Add Serialize and Deserialize
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: String,
    pub hash: String,
    pub data: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(index: u64, previous_hash: String, data: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let nonce = 0;
        let hash = Block::calculate_hash(index, timestamp, &previous_hash, &data, nonce);

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
            nonce,
        }
    }

    pub fn calculate_hash(index: u64, timestamp: u128, previous_hash: &str, data: &str, nonce: u64) -> String {
        let content = format!("{}{}{}{}{}", index, timestamp, previous_hash, data, nonce);
        format!("{:x}", md5::compute(content))
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = Block::calculate_hash(self.index, self.timestamp, &self.previous_hash, &self.data, self.nonce);
        }
    }
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: usize,
    pub peers: Vec<String>,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            difficulty,
            peers: Vec::new(),
        };
        blockchain.add_block("Genesis Block".to_string());
        blockchain
    }

    pub async fn add_peer(&mut self, peer: String) {
        self.peers.push(peer);
    }

    pub fn add_block(&mut self, data: String) {
        let previous_hash = self.blocks.last().map_or("0".to_string(), |block| block.hash.clone());
        let mut block = Block::new(self.blocks.len() as u64, previous_hash, data);

        // Mining the block (proof-of-work)
        block.mine_block(self.difficulty);

        // Check if the hash already exists in the blockchain
        if self.is_hash_unique(&block.hash) {
            self.blocks.push(block);
            println!("Block added successfully.");
        } else {
            println!("Block with the same hash already exists. Block not added.");
        }
    }

    // New method to check hash uniqueness
    fn is_hash_unique(&self, hash: &str) -> bool {
        for block in &self.blocks {
            if block.hash == hash {
                return false;
            }
        }
        true
    }

    pub async fn broadcast_new_block(&self, block: &Block) {
        for peer in &self.peers {
            let client = reqwest::Client::new();
            let res = client.post(peer)
                .json(block)
                .send()
                .await;
              match res {
                  Ok(_) => println!("Block sent to peer {}", peer),
                  Err(e) => println!("Error sending block to peer {}: {}", peer, e),
              }
           }
       }
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let previous_block = &self.blocks[i - 1];
            let current_block = &self.blocks[i];

            if current_block.hash != Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.previous_hash,
                &current_block.data,
                current_block.nonce,
            ) {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

   pub async fn receive_block(&mut self, block: Block) {
        // Verify block and add to the chain
        if block.previous_hash == self.blocks.last().unwrap().hash {
            self.blocks.push(block);
        } else {
            eprintln!("Received block is invalid and was not added to the chain");
        }
   }
