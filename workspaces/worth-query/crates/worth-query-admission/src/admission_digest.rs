use sha2::{Digest, Sha256};
use worth_foundational::facade::CanonicalDigestId;

pub(crate) fn hash_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

pub(crate) fn hash_parts_with_digests(parts: &[String], digests: &[&CanonicalDigestId]) -> String {
    let mut hasher = Sha256::new();
    for digest in digests {
        hasher.update(b"digest");
        hasher.update(digest.bytes());
    }
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}
