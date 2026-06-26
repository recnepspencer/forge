use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(crate) fn stable_digest(parts: &[String]) -> String {
    let mut sorted_parts = parts.to_vec();
    sorted_parts.sort();
    let mut hasher = DefaultHasher::new();
    sorted_parts.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
