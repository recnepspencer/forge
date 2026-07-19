use sha2::{Digest, Sha256};

pub(super) fn digest_hash_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0x1f]);
    }
    format!("{:x}", hasher.finalize())
}

pub(crate) fn hash_parts(parts: &[String]) -> String {
    digest_hash_parts(parts)
}
