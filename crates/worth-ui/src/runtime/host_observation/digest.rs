use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(super) fn digest_parts(parts: impl IntoIterator<Item = impl AsRef<str>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.as_ref().hash(&mut hasher);
    }
    hasher.finish()
}
