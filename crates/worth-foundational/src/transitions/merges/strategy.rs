use crate::identities::{BoundaryHandle, CanonicalDigestId, EquivalenceBasisId};
use crate::transitions::FoundationalBranchId;

macro_rules! nonempty_string_wrapper {
    ($name:ident, $empty_variant:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(String);

        impl $name {
            pub fn new(
                value: impl Into<String>,
            ) -> Result<Self, super::vocabulary::FoundationalMergeConstructionDenial> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(
                        super::vocabulary::FoundationalMergeConstructionDenial::$empty_variant,
                    );
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalTransitionStrategyId(BoundaryHandle);

impl FoundationalTransitionStrategyId {
    pub const fn new(handle: BoundaryHandle) -> Self {
        Self(handle)
    }

    pub const fn handle(&self) -> BoundaryHandle {
        self.0
    }
}

nonempty_string_wrapper!(FoundationalTransitionStrategyFamily, EmptyStrategyFamily);
nonempty_string_wrapper!(
    FoundationalTransitionStrategySemanticName,
    EmptyStrategySemanticName
);
nonempty_string_wrapper!(FoundationalTransitionStrategyVersion, EmptyStrategyVersion);
nonempty_string_wrapper!(FoundationalTransitionBasisFamily, EmptyBasisFamily);
nonempty_string_wrapper!(FoundationalTransitionBasisVersion, EmptyBasisVersion);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalTransitionStrategyOwnershipClass {
    RuntimeBuiltIn,
    CustomRegistered,
    CompatibilityLowered,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalTransitionStrategyIdentity {
    id: FoundationalTransitionStrategyId,
    family: FoundationalTransitionStrategyFamily,
    semantic_name: FoundationalTransitionStrategySemanticName,
    version: FoundationalTransitionStrategyVersion,
    ownership: FoundationalTransitionStrategyOwnershipClass,
}

impl FoundationalTransitionStrategyIdentity {
    pub fn new(
        id: FoundationalTransitionStrategyId,
        family: FoundationalTransitionStrategyFamily,
        semantic_name: FoundationalTransitionStrategySemanticName,
        version: FoundationalTransitionStrategyVersion,
        ownership: FoundationalTransitionStrategyOwnershipClass,
    ) -> Self {
        Self {
            id,
            family,
            semantic_name,
            version,
            ownership,
        }
    }

    pub const fn id(&self) -> FoundationalTransitionStrategyId {
        self.id
    }

    pub fn family(&self) -> &FoundationalTransitionStrategyFamily {
        &self.family
    }

    pub fn semantic_name(&self) -> &FoundationalTransitionStrategySemanticName {
        &self.semantic_name
    }

    pub fn version(&self) -> &FoundationalTransitionStrategyVersion {
        &self.version
    }

    pub const fn ownership(&self) -> FoundationalTransitionStrategyOwnershipClass {
        self.ownership
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalTransitionStrategyDescriptorDigest(CanonicalDigestId);

impl FoundationalTransitionStrategyDescriptorDigest {
    pub const fn new(digest_id: CanonicalDigestId) -> Self {
        Self(digest_id)
    }

    pub const fn digest_id(&self) -> CanonicalDigestId {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalTransitionStrategyContractBasis(EquivalenceBasisId);

impl FoundationalTransitionStrategyContractBasis {
    pub const fn new(basis_id: EquivalenceBasisId) -> Self {
        Self(basis_id)
    }

    pub const fn basis_id(&self) -> EquivalenceBasisId {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalTransitionBasisIdentity(EquivalenceBasisId);

impl FoundationalTransitionBasisIdentity {
    pub const fn new(basis_id: EquivalenceBasisId) -> Self {
        Self(basis_id)
    }

    pub const fn basis_id(&self) -> EquivalenceBasisId {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalMergeBaseSelectionBasis(EquivalenceBasisId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalStrategyBasis(EquivalenceBasisId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalTransitionCorrespondenceBasis(EquivalenceBasisId);
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FoundationalTransitionRemapBasis(EquivalenceBasisId);

macro_rules! basis_wrapper_impl {
    ($name:ident) => {
        impl $name {
            pub const fn new(basis_id: EquivalenceBasisId) -> Self {
                Self(basis_id)
            }

            pub const fn basis_id(&self) -> EquivalenceBasisId {
                self.0
            }
        }
    };
}

basis_wrapper_impl!(FoundationalMergeBaseSelectionBasis);
basis_wrapper_impl!(FoundationalStrategyBasis);
basis_wrapper_impl!(FoundationalTransitionCorrespondenceBasis);
basis_wrapper_impl!(FoundationalTransitionRemapBasis);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalMergeBasis {
    identity: FoundationalTransitionBasisIdentity,
    family: FoundationalTransitionBasisFamily,
    version: FoundationalTransitionBasisVersion,
    source_branch: FoundationalBranchId,
    target_branch: FoundationalBranchId,
}

impl FoundationalMergeBasis {
    pub fn new(
        identity: FoundationalTransitionBasisIdentity,
        family: FoundationalTransitionBasisFamily,
        version: FoundationalTransitionBasisVersion,
        source_branch: FoundationalBranchId,
        target_branch: FoundationalBranchId,
    ) -> Self {
        Self {
            identity,
            family,
            version,
            source_branch,
            target_branch,
        }
    }

    pub const fn identity(&self) -> FoundationalTransitionBasisIdentity {
        self.identity
    }

    pub fn family(&self) -> &FoundationalTransitionBasisFamily {
        &self.family
    }

    pub fn version(&self) -> &FoundationalTransitionBasisVersion {
        &self.version
    }

    pub fn source_branch(&self) -> &FoundationalBranchId {
        &self.source_branch
    }

    pub fn target_branch(&self) -> &FoundationalBranchId {
        &self.target_branch
    }
}
