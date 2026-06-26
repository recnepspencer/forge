use crate::{ContainerIntegrityCounters, PhysicalBoundaryLocalization, PhysicalScopeBasis};
use forge_store_physical_format::PhysicalRecordSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalContainerIntegrityDenialKind {
    HeaderWitnessMismatch,
    BodyLengthMismatch,
    SlotDirectoryMalformed,
    SlotStateIntegrityFailure,
    FrameOutOfBounds,
    TornFrame,
    MalformedFrame,
    ExtentBoundaryMismatch,
    WrongPhysicalFamily,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AmbiguousBoundaryDamage {
    boundary: PhysicalBoundaryLocalization,
}

impl AmbiguousBoundaryDamage {
    pub const fn new(boundary: PhysicalBoundaryLocalization) -> Self {
        Self { boundary }
    }

    pub const fn boundary(self) -> PhysicalBoundaryLocalization {
        self.boundary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TornFrameDenial {
    expected_length: usize,
    actual_length: usize,
}

impl TornFrameDenial {
    pub const fn new(expected_length: usize, actual_length: usize) -> Self {
        Self {
            expected_length,
            actual_length,
        }
    }

    pub const fn expected_length(self) -> usize {
        self.expected_length
    }

    pub const fn actual_length(self) -> usize {
        self.actual_length
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalContainerIntegrityDenial {
    kind: PhysicalContainerIntegrityDenialKind,
    basis: Option<PhysicalScopeBasis>,
    localization: PhysicalBoundaryLocalization,
    counters: ContainerIntegrityCounters,
    slot: Option<PhysicalRecordSlot>,
    expected_length: Option<usize>,
    actual_length: Option<usize>,
    ambiguous: Option<AmbiguousBoundaryDamage>,
    torn_frame: Option<TornFrameDenial>,
}

impl PhysicalContainerIntegrityDenial {
    pub(crate) const fn new(
        kind: PhysicalContainerIntegrityDenialKind,
        localization: PhysicalBoundaryLocalization,
        counters: ContainerIntegrityCounters,
    ) -> Self {
        Self {
            kind,
            basis: None,
            localization,
            counters,
            slot: None,
            expected_length: None,
            actual_length: None,
            ambiguous: None,
            torn_frame: None,
        }
    }

    pub(crate) const fn with_slot(mut self, slot: PhysicalRecordSlot) -> Self {
        self.slot = Some(slot);
        self
    }

    pub(crate) fn with_basis(mut self, basis: PhysicalScopeBasis) -> Self {
        self.basis = Some(basis);
        self
    }

    pub(crate) const fn with_lengths(mut self, expected: usize, actual: usize) -> Self {
        self.expected_length = Some(expected);
        self.actual_length = Some(actual);
        self
    }

    pub(crate) const fn with_ambiguous(mut self, damage: AmbiguousBoundaryDamage) -> Self {
        self.ambiguous = Some(damage);
        self
    }

    pub(crate) const fn with_torn_frame(mut self, denial: TornFrameDenial) -> Self {
        self.torn_frame = Some(denial);
        self
    }

    pub const fn kind(&self) -> PhysicalContainerIntegrityDenialKind {
        self.kind
    }

    pub const fn basis(&self) -> Option<&PhysicalScopeBasis> {
        self.basis.as_ref()
    }

    pub const fn localization(&self) -> PhysicalBoundaryLocalization {
        self.localization
    }

    pub const fn counters(&self) -> ContainerIntegrityCounters {
        self.counters
    }

    pub const fn slot(&self) -> Option<PhysicalRecordSlot> {
        self.slot
    }

    pub const fn expected_length(&self) -> Option<usize> {
        self.expected_length
    }

    pub const fn actual_length(&self) -> Option<usize> {
        self.actual_length
    }

    pub const fn ambiguous_boundary_damage(&self) -> Option<AmbiguousBoundaryDamage> {
        self.ambiguous
    }

    pub const fn torn_frame(&self) -> Option<TornFrameDenial> {
        self.torn_frame
    }
}
