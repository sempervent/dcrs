// src/lib.rs

// Blockchain-related modules
pub mod blockchain {
    pub mod block;
    pub mod blockchain;
    pub mod consensus;
    pub mod hashing;
}

// Governance-related module
pub mod governance {
    pub mod governance;
}

// Logging-related module
pub mod logging;

// P2P-related module
pub mod p2p;

// Storage-related module
pub mod storage {
    pub mod storage;
}

// Transaction-related module
pub mod transaction;

// Wallet-related module
pub mod wallet;

// Re-export key structs and functions for easier access
pub use blockchain::block::Block;
pub use blockchain::blockchain::Blockchain;
pub use blockchain::consensus::{proof_of_engagement, proof_of_interaction};
pub use blockchain::hashing::{md5_hash, sha256_hash};
pub use governance::governance::{Governance, Proposal};
pub use storage::storage::StorageNode;
pub use transaction::Transaction; // Assuming Transaction struct exists in transaction.rs
pub use wallet::{Wallet, WalletManager};
