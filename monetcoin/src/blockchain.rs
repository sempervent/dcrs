use crate::block::Block;
use crate::transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u64,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4,  // Difficulty for proof-of-work, if implemented
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, vec![], "0".to_string());
        self.chain.push(genesis_block);
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, new_block: Block) {
        let previous_hash = self.latest_block().hash.clone();
        let mut block = new_block;
        block.previous_hash = previous_hash;
        block.hash = block.calculate_hash();
        self.chain.push(block);
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), &'static str> {
        if transaction.is_valid() {
            let latest_block = self.latest_block();
            let new_block = Block::new(latest_block.index + 1, vec![transaction], latest_block.hash.clone());
            self.add_block(new_block);
            Ok(())
        } else {
            Err("Invalid Transaction")
        }
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}
