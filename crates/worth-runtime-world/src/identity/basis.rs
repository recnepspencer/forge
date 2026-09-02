use super::RuntimeWorldOwnerIdentity;

/// Owner-issued identity of one exact admitted component/correspondence
/// tuple. It is not a content digest and does not collapse commit history.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompositeBasisIdentity {
    owner: RuntimeWorldOwnerIdentity,
    ordinal: u64,
}

impl CompositeBasisIdentity {
    pub(crate) const fn issued(owner: RuntimeWorldOwnerIdentity, ordinal: u64) -> Self {
        Self { owner, ordinal }
    }

    pub const fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }
}
