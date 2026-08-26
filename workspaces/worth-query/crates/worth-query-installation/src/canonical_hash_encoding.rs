use sha2::{Digest, Sha256};

pub(crate) trait CanonicalHashSink {
    fn write(&mut self, value: &[u8]);
}

impl CanonicalHashSink for Sha256 {
    fn write(&mut self, value: &[u8]) {
        Digest::update(self, value);
    }
}

#[derive(Default)]
pub(crate) struct CanonicalHashByteCounter(u64);

impl CanonicalHashByteCounter {
    pub const fn bytes(&self) -> u64 {
        self.0
    }
}

impl CanonicalHashSink for CanonicalHashByteCounter {
    fn write(&mut self, value: &[u8]) {
        self.0 = self
            .0
            .saturating_add(u64::try_from(value.len()).unwrap_or(u64::MAX));
    }
}

/// Adds one tagged text field to a canonical identity without reserving any
/// characters from the caller's vocabulary.
pub(crate) fn hash_text_field(hasher: &mut impl CanonicalHashSink, tag: &str, value: &str) {
    hash_bytes(hasher, tag.as_bytes());
    hash_bytes(hasher, value.as_bytes());
}

fn hash_bytes(hasher: &mut impl CanonicalHashSink, value: &[u8]) {
    let length = u64::try_from(value.len()).expect("canonical identity field length fits u64");
    hasher.write(&length.to_le_bytes());
    hasher.write(value);
}
