use super::{RuntimeWorldIdentityExhaustion, RuntimeWorldOwnerIdentity};

/// Owner-issued identity of one product reference cell.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductBranchIdentity {
    owner: RuntimeWorldOwnerIdentity,
    ordinal: u64,
}

impl ProductBranchIdentity {
    pub(crate) const fn issued(owner: RuntimeWorldOwnerIdentity, ordinal: u64) -> Self {
        Self { owner, ordinal }
    }

    pub const fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }
}

/// Lifecycle incarnation of a product reference. Retirement never permits
/// the same branch name or cell identity to reuse this value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductBranchLifecycleIncarnation {
    owner: RuntimeWorldOwnerIdentity,
    ordinal: u64,
}

impl ProductBranchLifecycleIncarnation {
    pub(crate) const fn issued(owner: RuntimeWorldOwnerIdentity, ordinal: u64) -> Self {
        Self { owner, ordinal }
    }

    pub const fn owner_identity(self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }
}

/// Generation of one mutable product reference cell. It advances only after
/// a successful product-reference movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductBranchReferenceGeneration(u64);

impl ProductBranchReferenceGeneration {
    pub(crate) const fn initial() -> Self {
        Self(0)
    }

    pub(crate) const fn get(self) -> u64 {
        self.0
    }

    pub(crate) fn advance(self) -> Result<Self, RuntimeWorldIdentityExhaustion> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(RuntimeWorldIdentityExhaustion::new(
                super::RuntimeWorldIdentityFamily::ProductBranchReferenceGeneration,
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::ProductBranchReferenceGeneration;
    use crate::identity::RuntimeWorldIdentityFamily;

    #[test]
    fn generation_advances_only_as_a_checked_transition() {
        let initial = ProductBranchReferenceGeneration::initial();
        assert_eq!(initial.get(), 0);
        assert_eq!(initial.advance().unwrap().get(), 1);

        let terminal = ProductBranchReferenceGeneration(u64::MAX);
        let denial = terminal.advance().expect_err("generation must not wrap");
        assert_eq!(
            denial.family(),
            RuntimeWorldIdentityFamily::ProductBranchReferenceGeneration
        );
    }
}
