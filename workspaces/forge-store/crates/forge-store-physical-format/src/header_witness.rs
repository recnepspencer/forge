use crate::{
    PhysicalDecodedHeader, PhysicalGenerationOwner, PhysicalHeaderDecodeCounterSnapshot,
    PhysicalHeaderKind, PhysicalPublicationState, PHYSICAL_HEADER_LENGTH,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalHeaderDecodeWitness {
    header: PhysicalDecodedHeader,
    owner: PhysicalGenerationOwner,
    counters: PhysicalHeaderDecodeCounterSnapshot,
}

impl PhysicalHeaderDecodeWitness {
    pub(crate) const fn new(
        header: PhysicalDecodedHeader,
        owner: PhysicalGenerationOwner,
        counters: PhysicalHeaderDecodeCounterSnapshot,
    ) -> Self {
        Self {
            header,
            owner,
            counters,
        }
    }

    pub const fn header(self) -> PhysicalDecodedHeader {
        self.header
    }

    pub const fn kind(self) -> PhysicalHeaderKind {
        self.header.kind()
    }

    pub const fn owner(self) -> PhysicalGenerationOwner {
        self.owner
    }

    pub const fn payload_offset(self) -> usize {
        PHYSICAL_HEADER_LENGTH as usize
    }

    pub const fn payload_length(self) -> u32 {
        self.header.payload_length()
    }

    pub const fn publication(self) -> PhysicalPublicationState {
        self.header.publication()
    }

    pub const fn counters(self) -> PhysicalHeaderDecodeCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalHeaderDecodeReport {
    witness: PhysicalHeaderDecodeWitness,
}

impl PhysicalHeaderDecodeReport {
    pub(crate) const fn new(witness: PhysicalHeaderDecodeWitness) -> Self {
        Self { witness }
    }

    pub const fn witness(self) -> PhysicalHeaderDecodeWitness {
        self.witness
    }

    pub const fn counters(self) -> PhysicalHeaderDecodeCounterSnapshot {
        self.witness.counters()
    }
}
