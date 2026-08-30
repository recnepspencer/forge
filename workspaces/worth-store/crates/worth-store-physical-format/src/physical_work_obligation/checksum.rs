use sha2::{Digest, Sha256};

pub(super) fn calculate(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}
