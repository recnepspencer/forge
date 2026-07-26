use sha2::{Digest, Sha256};

pub(crate) fn hash_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.len().to_le_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

pub(crate) fn hash_protocol_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        let byte_count = u64::try_from(part.len()).unwrap_or(u64::MAX);
        hasher.update(byte_count.to_le_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}
