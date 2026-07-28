use crate::{ChunkIntegrityCounters, PhysicalScopeBasis};
use worth_store_physical_format::{PhysicalGenerationOwner, PhysicalReferenceScope};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkIntegrityDenialKind {
    WrongPhysicalFamily,
    MissingCheckedChunkWindow,
    UnboundedWholeObjectWindow,
    ProtectedWindowExceedsStreamingWindow,
    ChunkHeaderDamage,
    ChunkPayloadDamage,
    ChunkBoundaryDamage,
    ExtentBoundaryDamage,
    UnknownChunkIntegrity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkDamageLocality {
    ChunkHeader(PhysicalReferenceScope),
    ChunkPayload(PhysicalReferenceScope),
    ChunkBoundary(PhysicalReferenceScope),
    ExtentBoundary(PhysicalReferenceScope),
    Unknown(PhysicalReferenceScope),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkIntegrityDenial {
    kind: ChunkIntegrityDenialKind,
    basis: Option<PhysicalScopeBasis>,
    locality: Option<PhysicalGenerationOwner>,
    damage_locality: Option<ChunkDamageLocality>,
    counters: ChunkIntegrityCounters,
}

impl ChunkIntegrityDenial {
    pub(crate) fn new(kind: ChunkIntegrityDenialKind, counters: ChunkIntegrityCounters) -> Self {
        Self {
            kind,
            basis: None,
            locality: None,
            damage_locality: None,
            counters,
        }
    }

    pub(crate) fn with_basis(mut self, basis: PhysicalScopeBasis) -> Self {
        self.locality = Some(basis.scope().owner());
        self.basis = Some(basis);
        self
    }

    pub(crate) const fn with_damage_locality(mut self, locality: ChunkDamageLocality) -> Self {
        self.damage_locality = Some(locality);
        self
    }

    pub const fn kind(&self) -> ChunkIntegrityDenialKind {
        self.kind
    }

    pub const fn basis(&self) -> Option<&PhysicalScopeBasis> {
        self.basis.as_ref()
    }

    pub const fn locality(&self) -> Option<PhysicalGenerationOwner> {
        self.locality
    }

    pub const fn damage_locality(&self) -> Option<ChunkDamageLocality> {
        self.damage_locality
    }

    pub const fn counters(&self) -> ChunkIntegrityCounters {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkIntegrityStreamingWindowDenial {
    WindowExceedsBlobAllocation { requested: u64, allocation: u64 },
    WholeObjectWindow,
}
