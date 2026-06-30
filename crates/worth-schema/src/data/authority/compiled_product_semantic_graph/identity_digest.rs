use sha2::{Digest, Sha256};

pub(super) fn compiled_product_semantic_graph_identity_digest(
    namespace: &'static str,
    canonical_parts: &[String],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(namespace.as_bytes());
    for part in canonical_parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}
