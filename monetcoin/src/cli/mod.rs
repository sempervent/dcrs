// src/cli/mod.rs

use structopt::StructOpt;
use crate::blockchain::Blockchain;
use crate::wallet::WalletManager;

#[derive(StructOpt)]
#[structopt(name = "blockchain-cli", about = "A CLI for interacting with the blockchain.")]
pub enum Cli {
    #[structopt(name = "create-wallet")]
    CreateWallet {
        #[structopt(short, long)]
        address: String,
    },
    #[structopt(name = "view-blockchain")]
    ViewBlockchain,
    #[structopt(name = "add-block")]
    AddBlock {
        #[structopt(short, long)]
        data: String,
    },
}

pub fn run() {
    let args = Cli::from_args();

    match args {
        Cli::CreateWallet { address } => {
            let mut wallet_manager = WalletManager::new();
            wallet_manager.create_wallet(address);
            println!("Wallet created.");
        },
        Cli::ViewBlockchain => {
            let blockchain = Blockchain::new(2); // Replace with actual blockchain instance
            for block in blockchain.blocks {
                println!("{:?}", block);
            }
        },
        Cli::AddBlock { data } => {
            let mut blockchain = Blockchain::new(2); // Replace with actual blockchain instance
            blockchain.add_block(data, "dummy_hash".to_string(), vec![]);
            println!("Block added.");
        },
    }
}
