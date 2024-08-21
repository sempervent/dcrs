mod block;
mod blockchain;
mod transaction;
mod wallet;

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::wallet::Wallet;

fn main() {
    // Initialize the blockchain
    let mut blockchain = Blockchain::new();

    // Create wallets
    let alice_wallet = Wallet::new();
    let bob_wallet = Wallet::new();

    // Create a transaction
    let mut transaction = Transaction::new(alice_wallet.public_key, bob_wallet.public_key, 50);
    transaction.sign_transaction(&alice_wallet);

    // Add transaction to the blockchain
    match blockchain.add_transaction(transaction) {
        Ok(_) => println!("Transaction added successfully."),
        Err(err) => println!("Failed to add transaction: {}", err),
    }

    // Display the blockchain
    for block in blockchain.chain.iter() {
        println!("{:?}", block);
    }

    // Validate the blockchain
    if blockchain.is_chain_valid() {
        println!("Blockchain is valid!");
    } else {
        println!("Blockchain is not valid!");
    }
}
