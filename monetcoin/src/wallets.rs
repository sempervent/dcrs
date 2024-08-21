use ed25519_dalek::{Keypair, PublicKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;

pub struct Wallet {
    pub public_key: PublicKey,
    private_key: Keypair,
}

impl Wallet {
    pub fn new() -> Self {
        let mut csprng = OsRng{};
        let keypair = Keypair::generate(&mut csprng);
        Wallet {
            public_key: keypair.public,
            private_key: keypair,
        }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.private_key.sign(message)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.public_key.verify(message, signature).is_ok()
    }
}
