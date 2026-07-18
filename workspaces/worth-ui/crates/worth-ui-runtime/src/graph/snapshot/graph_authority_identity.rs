use std::sync::atomic::{AtomicU64, Ordering};

/// Opaque identity for one graph authority lineage.
///
/// This is intentionally distinct from semantic digests: equivalent graph
/// snapshots may compare equal in meaning without sharing authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiGraphAuthorityIdentity(u64);

static NEXT_GRAPH_AUTHORITY_IDENTITY: AtomicU64 = AtomicU64::new(1);

impl UiGraphAuthorityIdentity {
    pub(super) fn mint() -> Self {
        Self(NEXT_GRAPH_AUTHORITY_IDENTITY.fetch_add(1, Ordering::Relaxed))
    }
}
