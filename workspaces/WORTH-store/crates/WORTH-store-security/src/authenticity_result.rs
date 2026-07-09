use crate::{
    StoreAuthenticityCheckCounterSnapshot, StoreAuthenticityPhysicalIdentity,
    StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreSecurityScopeIdentity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreAuthenticityResultKind {
    Verified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreAuthenticityResult {
    kind: StoreAuthenticityResultKind,
    requirement: StoreAuthenticityRequirement,
    scope_identity: StoreSecurityScopeIdentity,
    physical_identity: StoreAuthenticityPhysicalIdentity,
    counters: StoreAuthenticityCheckCounterSnapshot,
}

impl StoreAuthenticityResult {
    pub(crate) const fn verified(
        requirement: StoreAuthenticityRequirement,
        scope_identity: StoreSecurityScopeIdentity,
        physical_identity: StoreAuthenticityPhysicalIdentity,
        counters: StoreAuthenticityCheckCounterSnapshot,
    ) -> Self {
        Self {
            kind: StoreAuthenticityResultKind::Verified,
            requirement,
            scope_identity,
            physical_identity,
            counters,
        }
    }

    pub const fn kind(&self) -> StoreAuthenticityResultKind {
        self.kind
    }

    pub const fn requirement(&self) -> StoreAuthenticityRequirement {
        self.requirement
    }

    pub const fn requirement_class(&self) -> Option<StoreAuthenticityRequirementClass> {
        self.requirement.class()
    }

    pub const fn scope_identity(&self) -> StoreSecurityScopeIdentity {
        self.scope_identity
    }

    pub const fn physical_identity(&self) -> StoreAuthenticityPhysicalIdentity {
        self.physical_identity
    }

    pub const fn counters(&self) -> StoreAuthenticityCheckCounterSnapshot {
        self.counters
    }
}
