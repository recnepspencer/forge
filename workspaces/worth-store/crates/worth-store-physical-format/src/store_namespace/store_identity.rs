pub(super) const STORE_IDENTITY_BYTES: usize = 16;

/// Candidate identity bytes used before durable namespace publication.
///
/// This value validates representation only. It does not claim that its bytes
/// came from an approved entropy source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProposedStoreIdentity([u8; STORE_IDENTITY_BYTES]);

impl ProposedStoreIdentity {
    pub fn from_nonzero_bytes(bytes: [u8; STORE_IDENTITY_BYTES]) -> Option<Self> {
        (bytes != [0; STORE_IDENTITY_BYTES]).then_some(Self(bytes))
    }

    pub const fn bytes(self) -> [u8; STORE_IDENTITY_BYTES] {
        self.0
    }
}

/// Path-independent identity decoded from a valid published record.
///
/// The identity is canonical namespace meaning, not permission to open or
/// mutate a Store runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StableStoreIdentity([u8; STORE_IDENTITY_BYTES]);

impl StableStoreIdentity {
    pub const fn bytes(self) -> [u8; STORE_IDENTITY_BYTES] {
        self.0
    }

    pub(crate) const fn from_published_record(identity: ProposedStoreIdentity) -> Self {
        Self(identity.bytes())
    }
}
