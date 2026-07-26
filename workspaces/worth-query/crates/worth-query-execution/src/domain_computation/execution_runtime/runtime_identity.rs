use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_RUNTIME_AUTHORITY_IDENTITY: AtomicU64 = AtomicU64::new(1);

/// Process-local identity for one concrete Query execution authority owner.
///
/// The identity is minted only while constructing a real execution runtime.
/// Labels, digests, and workspace names cannot reconstruct it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryRuntimeAuthorityIdentity(u64);

impl WorthQueryRuntimeAuthorityIdentity {
    pub(super) fn mint() -> Self {
        Self(NEXT_RUNTIME_AUTHORITY_IDENTITY.fetch_add(1, Ordering::Relaxed))
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}
