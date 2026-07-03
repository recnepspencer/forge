use crate::{
    StoreAuthenticityCheckCounterSnapshot, StoreAuthenticityRequirement, StoreSecurityScopeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreAuthenticityCheckDenialKind {
    ResultNotRequired,
    MissingWitness,
    StaleWitness,
    WrongScope,
    WrongPhysicalIdentity,
    Unavailable,
    Unsupported,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreAuthenticityCheckDenial {
    kind: StoreAuthenticityCheckDenialKind,
    requirement: StoreAuthenticityRequirement,
    scope_identity: StoreSecurityScopeIdentity,
    counters: StoreAuthenticityCheckCounterSnapshot,
}

impl StoreAuthenticityCheckDenial {
    pub(crate) const fn new(
        kind: StoreAuthenticityCheckDenialKind,
        requirement: StoreAuthenticityRequirement,
        scope_identity: StoreSecurityScopeIdentity,
        counters: StoreAuthenticityCheckCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            requirement,
            scope_identity,
            counters,
        }
    }

    pub const fn kind(&self) -> StoreAuthenticityCheckDenialKind {
        self.kind
    }

    pub const fn requirement(&self) -> StoreAuthenticityRequirement {
        self.requirement
    }

    pub const fn scope_identity(&self) -> StoreSecurityScopeIdentity {
        self.scope_identity
    }

    pub const fn counters(&self) -> StoreAuthenticityCheckCounterSnapshot {
        self.counters
    }

    pub const fn is_checksum_valid_authenticity_failed(&self) -> bool {
        matches!(self.kind, StoreAuthenticityCheckDenialKind::Failed)
    }

    pub const fn is_checksum_valid_authenticity_unavailable(&self) -> bool {
        matches!(self.kind, StoreAuthenticityCheckDenialKind::Unavailable)
    }

    pub const fn is_checksum_valid_authenticity_unsupported(&self) -> bool {
        matches!(self.kind, StoreAuthenticityCheckDenialKind::Unsupported)
    }
}
