use crate::PhysicalReference;

use super::{AccessPolicySecurityScope, StoreAccessMode};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MixedAccessTransition {
    previous: StoreAccessMode,
    requested: StoreAccessMode,
}

impl MixedAccessTransition {
    pub const fn new(previous: StoreAccessMode, requested: StoreAccessMode) -> Self {
        Self {
            previous,
            requested,
        }
    }

    pub const fn previous(self) -> StoreAccessMode {
        self.previous
    }

    pub const fn requested(self) -> StoreAccessMode {
        self.requested
    }

    pub const fn involves(self, mode: StoreAccessMode) -> bool {
        matches!(
            (self.previous, self.requested, mode),
            (StoreAccessMode::Buffered, _, StoreAccessMode::Buffered)
                | (_, StoreAccessMode::Buffered, StoreAccessMode::Buffered)
                | (StoreAccessMode::Mmap, _, StoreAccessMode::Mmap)
                | (_, StoreAccessMode::Mmap, StoreAccessMode::Mmap)
                | (StoreAccessMode::DirectIo, _, StoreAccessMode::DirectIo)
                | (_, StoreAccessMode::DirectIo, StoreAccessMode::DirectIo)
        )
    }

    pub const fn has_only_physical_participants(self) -> bool {
        is_physical_participant(self.previous) && is_physical_participant(self.requested)
    }
}

const fn is_physical_participant(mode: StoreAccessMode) -> bool {
    matches!(
        mode,
        StoreAccessMode::Buffered | StoreAccessMode::Mmap | StoreAccessMode::DirectIo
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MixedInvalidationPosture {
    PageCacheInvalidated,
    MmapCleanSharedVisibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum MixedWritebackPosture {
    NoDirtyWritebackRace,
    DirtyWritebackSequencedByStore,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MixedAccessCoherenceBasis {
    transition: MixedAccessTransition,
    reference: PhysicalReference,
    security_scope: AccessPolicySecurityScope,
    invalidation: MixedInvalidationPosture,
    writeback: MixedWritebackPosture,
    _seal: MixedAccessCoherenceBasisSeal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MixedAccessCoherenceBasisSeal;

impl MixedAccessCoherenceBasis {
    pub(crate) const fn new(
        transition: MixedAccessTransition,
        reference: PhysicalReference,
        security_scope: AccessPolicySecurityScope,
        invalidation: MixedInvalidationPosture,
        writeback: MixedWritebackPosture,
    ) -> Self {
        Self {
            transition,
            reference,
            security_scope,
            invalidation,
            writeback,
            _seal: MixedAccessCoherenceBasisSeal,
        }
    }

    pub fn matches_request(
        self,
        transition: MixedAccessTransition,
        reference: PhysicalReference,
        security_scope: AccessPolicySecurityScope,
    ) -> bool {
        self.transition == transition
            && self.reference == reference
            && self.security_scope == security_scope
    }

    pub const fn transition(self) -> MixedAccessTransition {
        self.transition
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn security_scope(self) -> AccessPolicySecurityScope {
        self.security_scope
    }
}
