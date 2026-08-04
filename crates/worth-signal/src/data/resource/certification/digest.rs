use serde::Serialize;
use sha2::{Digest, Sha256};

pub(super) fn resource_canonical_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("resource certification serialization");
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}
