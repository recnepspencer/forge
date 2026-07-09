use sha2::{Digest, Sha256};

pub(crate) fn stable_byte_digest(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("sha256:{}:{digest:x}", bytes.len())
}
