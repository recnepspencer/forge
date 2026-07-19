use sha2::{Digest, Sha256};

/// Adds one tagged text field to a canonical identity without reserving any
/// characters from the caller's vocabulary.
pub(crate) fn hash_text_field(hasher: &mut Sha256, tag: &str, value: &str) {
    hash_bytes(hasher, tag.as_bytes());
    hash_bytes(hasher, value.as_bytes());
}

fn hash_bytes(hasher: &mut Sha256, value: &[u8]) {
    let length = u64::try_from(value.len()).expect("canonical identity field length fits u64");
    hasher.update(length.to_le_bytes());
    hasher.update(value);
}
