use forge_store_contracts::StableDigest;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CheckpointId {
    digest: StableDigest,
}

impl CheckpointId {
    pub(crate) fn from_basis(value: impl Into<String>) -> Self {
        Self {
            digest: StableDigest::new(value)
                .expect("S.4 checkpoint identity digest basis is non-empty"),
        }
    }

    pub fn digest(&self) -> &StableDigest {
        &self.digest
    }
}
