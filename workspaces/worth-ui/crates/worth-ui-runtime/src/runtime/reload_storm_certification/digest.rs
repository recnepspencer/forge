use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub(crate) fn fold_texts(entries: impl IntoIterator<Item = String>) -> u64 {
    let mut entries = entries.into_iter().collect::<Vec<_>>();
    entries.sort();
    let mut hasher = DefaultHasher::new();
    for entry in entries {
        entry.hash(&mut hasher);
    }
    hasher.finish()
}
