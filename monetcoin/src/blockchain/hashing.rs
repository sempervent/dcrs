// blockchain/src/hashing.rs

use sha2::{Sha256, Digest};
use md5;

pub fn md5_hash(data: &str) -> String {
    format!("{:x}", md5::compute(data))
}

pub fn sha256_hash(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    format!("{:x}", hasher.finalize())
}

// Add additional hashing functions as needed
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_hash() {
        let data = "test data";
        let hash = md5_hash(data);

        assert_eq!(hash, "eb733a00c0c9d336e65691a37ab54293");
    }

    #[test]
    fn test_sha256_hash() {
        let data = "test data";
        let hash = sha256_hash(data);

        assert_eq!(hash, "d5579c46dfcc7d0d08cf5e12f24c027e9431b5e3c1a04c7a8ab44e2ddbe57f6d");
    }
}
