use super::{WorthQueryCollectionCapabilityCounters, WorthQueryCollectionWindowCounters};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCollectionCapabilityDenialKind {
    NotCollection,
    StaleInstallationGeneration,
    NativeAccessNotBound,
    UnsupportedGrouping,
    MissingEntityIdentityFacts,
    MissingViewLocalIdentityFacts,
    IdentityFactCardinalityMismatch,
    IdentityFactRelationshipMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryCollectionCapabilityDenial {
    kind: WorthQueryCollectionCapabilityDenialKind,
    counters: WorthQueryCollectionCapabilityCounters,
}

impl WorthQueryCollectionCapabilityDenial {
    pub(super) fn new(
        kind: WorthQueryCollectionCapabilityDenialKind,
        counters: WorthQueryCollectionCapabilityCounters,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(&self) -> WorthQueryCollectionCapabilityDenialKind {
        self.kind
    }

    pub const fn counters(&self) -> WorthQueryCollectionCapabilityCounters {
        self.counters
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCollectionWindowDenialKind {
    StaleInstallationGeneration,
    ForeignCapability,
    CapabilityGenerationMismatch,
    CursorBasisMismatch,
    CursorOrderingMismatch,
    CompleteCollectionRequiresBeginning,
    CompleteCollectionExceedsBreadth,
    CursorPastCollectionEnd,
    ForeignAdmission,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryCollectionWindowDenial {
    kind: WorthQueryCollectionWindowDenialKind,
    counters: WorthQueryCollectionWindowCounters,
}

impl WorthQueryCollectionWindowDenial {
    pub(super) fn new(
        kind: WorthQueryCollectionWindowDenialKind,
        counters: WorthQueryCollectionWindowCounters,
    ) -> Self {
        Self { kind, counters }
    }

    pub const fn kind(&self) -> WorthQueryCollectionWindowDenialKind {
        self.kind
    }

    pub const fn counters(&self) -> WorthQueryCollectionWindowCounters {
        self.counters
    }
}
