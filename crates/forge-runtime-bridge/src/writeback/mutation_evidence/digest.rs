use std::sync::Arc;

pub(super) fn aggregate_digest(label: &str, entries: impl IntoIterator<Item = String>) -> Arc<str> {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(label.as_bytes());
    for entry in entries {
        hasher.update(entry.as_bytes());
    }
    let digest = hasher.finalize();
    Arc::from(format!("{label}:sha256:{digest:x}"))
}

pub(super) fn aggregate_optional_digest(
    label: &str,
    entries: impl IntoIterator<Item = String>,
) -> Option<Arc<str>> {
    let entries = entries.into_iter().collect::<Vec<_>>();
    if entries.is_empty() {
        return None;
    }
    Some(aggregate_digest(label, entries))
}
