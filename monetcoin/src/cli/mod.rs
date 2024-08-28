// src/cli/mod.rs

use structopt::StructOpt;
use crate::blockchain::Blockchain;
use crate::wallet::WalletManager;
use crate::governance::Governance;
use crate::p2p::P2PNode;

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
    #[structopt(name = "propose")]
    Propose {
        #[structopt(short, long)]
        description: String,
    },
    #[structopt(name = "vote")]
    Vote {
        #[structopt(short, long)]
        proposal_id: u64,
        #[structopt(short, long)]
        for_proposal: bool,
        #[structopt(short, long)]
        coins: u64,
    },
    #[structopt(name = "start-node")]
    StartNode {
        #[structopt(short, long)]
        address: String,
        #[structopt(short, long)]
        port: u16,
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
        Cli::Propose { description } => {
            let mut governance = Governance::new();
            governance.propose(description);
            println!("Proposal added.");
        },
        Cli::Vote { proposal_id, for_proposal, coins } => {
            let mut governance = Governance::new();
            governance.vote(proposal_id, for_proposal, coins);
            println!("Vote cast.");
        },
        Cli::StartNode { address, port } => {
            let node_address = format!("{}:{}", address, port).parse().unwrap();
            let node = P2PNode::new(node_address);
            node.listen();
        },
    }
}
