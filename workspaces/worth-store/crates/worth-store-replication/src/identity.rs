#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReplicationCapsuleId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReplicationPeerId(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicationSourceEpoch(u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplicationLineageIdentity(String);

impl ReplicationPeerId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn admit(raw: String) -> Option<Self> {
        (!raw.is_empty()).then_some(Self(raw))
    }

    pub fn from_declared_peer(raw: impl Into<String>) -> Option<Self> {
        Self::admit(raw.into())
    }
}

impl ReplicationSourceEpoch {
    pub const fn get(self) -> u64 {
        self.0
    }

    pub(crate) const fn admit(raw: u64) -> Option<Self> {
        if raw == 0 {
            None
        } else {
            Some(Self(raw))
        }
    }
}

impl ReplicationLineageIdentity {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn admit(raw: String) -> Option<Self> {
        (!raw.is_empty()).then_some(Self(raw))
    }

    pub fn from_declared_lineage(raw: impl Into<String>) -> Option<Self> {
        Self::admit(raw.into())
    }

    pub fn stable_fingerprint(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};

        let mut digest = Sha256::new();
        digest.update(b"worth-store-replication-lineage-v1");
        digest.update((self.0.len() as u64).to_be_bytes());
        digest.update(self.0.as_bytes());
        digest.finalize().into()
    }
}
