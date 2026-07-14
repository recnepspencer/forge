use sha2::{Digest, Sha256};
use worth_store_aspect_native::StoreAspectIdentity;

/// Comparison-only identity retained by admitted capabilities.
///
/// This value cannot issue Store authority; it only proves that two opaque
/// capabilities descended from the same current-authority identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StoreCurrentAuthorityIdentity([u8; 32]);

impl StoreCurrentAuthorityIdentity {
    pub fn from_aspect_identity(identity: &StoreAspectIdentity) -> Self {
        let raw = identity.aspect_key().as_str();
        let mut digest = Sha256::new();
        digest.update((raw.len() as u64).to_be_bytes());
        digest.update(raw.as_bytes());
        Self(digest.finalize().into())
    }

    pub const fn fingerprint(self) -> [u8; 32] {
        self.0
    }
}
