// blockchain-cli/src/main.rs

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(4);

    blockchain.add_block("First block after genesis".to_string());
    blockchain.add_block("Second block after genesis".to_string());
    blockchain.add_block("Third block after genesis".to_string());

    for block in blockchain.blocks.iter() {
        println!("{:?}", block);
    }

    println!("Is blockchain valid? {}", blockchain.is_valid());
}
