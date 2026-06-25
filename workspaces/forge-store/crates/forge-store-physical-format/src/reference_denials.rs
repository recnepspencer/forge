use crate::{PhysicalReference, PhysicalReferenceValidationCounterSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalReferenceDenialKind {
    WrongReferenceKind,
    PlacementMismatch,
    StaleSlotGeneration,
    StaleExtentGeneration,
    StaleFreeSpaceReuseGeneration,
    StaleRootPublicationGeneration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalReferenceValidationDenial {
    kind: PhysicalReferenceDenialKind,
    reference: PhysicalReference,
    counters: PhysicalReferenceValidationCounterSnapshot,
    stale_reference: Option<StalePhysicalReference>,
}

impl PhysicalReferenceValidationDenial {
    pub(crate) const fn wrong_kind(
        reference: PhysicalReference,
        counters: PhysicalReferenceValidationCounterSnapshot,
    ) -> Self {
        Self {
            kind: PhysicalReferenceDenialKind::WrongReferenceKind,
            reference,
            counters,
            stale_reference: None,
        }
    }

    pub(crate) const fn placement_mismatch(
        reference: PhysicalReference,
        counters: PhysicalReferenceValidationCounterSnapshot,
    ) -> Self {
        Self {
            kind: PhysicalReferenceDenialKind::PlacementMismatch,
            reference,
            counters,
            stale_reference: None,
        }
    }

    pub(crate) const fn stale(stale_reference: StalePhysicalReference) -> Self {
        Self {
            kind: stale_reference.kind(),
            reference: stale_reference.reference(),
            counters: stale_reference.counters(),
            stale_reference: Some(stale_reference),
        }
    }

    pub const fn kind(self) -> PhysicalReferenceDenialKind {
        self.kind
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn counters(self) -> PhysicalReferenceValidationCounterSnapshot {
        self.counters
    }

    pub const fn stale_reference(self) -> Option<StalePhysicalReference> {
        self.stale_reference
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StalePhysicalReference {
    kind: PhysicalReferenceDenialKind,
    reference: PhysicalReference,
    counters: PhysicalReferenceValidationCounterSnapshot,
}

impl StalePhysicalReference {
    pub(crate) const fn new(
        kind: PhysicalReferenceDenialKind,
        reference: PhysicalReference,
        counters: PhysicalReferenceValidationCounterSnapshot,
    ) -> Self {
        Self {
            kind,
            reference,
            counters,
        }
    }

    pub const fn kind(self) -> PhysicalReferenceDenialKind {
        self.kind
    }

    pub const fn reference(self) -> PhysicalReference {
        self.reference
    }

    pub const fn counters(self) -> PhysicalReferenceValidationCounterSnapshot {
        self.counters
    }
}
