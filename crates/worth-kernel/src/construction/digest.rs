use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(super) fn digest_owned_parts(parts: &[String]) -> String {
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}
