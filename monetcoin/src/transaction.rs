use ed25519_dalek::{PublicKey, Signature};
use ed25519_dalek::Verifier;
use serde::{Serialize, Deserialize};
use crate::wallet::Wallet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: PublicKey,
    pub receiver: PublicKey,
    pub amount: u64,
    pub signature: Option<Signature>,
}

impl Transaction {
    pub fn new(sender: PublicKey, receiver: PublicKey, amount: u64) -> Self {
        Transaction {
            sender,
            receiver,
            amount,
            signature: None,
        }
    }

    pub fn sign_transaction(&mut self, wallet: &Wallet) {
        let message = self.get_message();
        let signature = wallet.sign(&message);
        self.signature = Some(signature);
    }

    pub fn is_valid(&self) -> bool {
        if let Some(signature) = &self.signature {
            let message = self.get_message();
            self.sender.verify(&message, signature).is_ok()
        } else {
            false
        }
    }

    fn get_message(&self) -> Vec<u8> {
        [self.sender.as_bytes(), self.receiver.as_bytes(), &self.amount.to_le_bytes()].concat()
    }
}
