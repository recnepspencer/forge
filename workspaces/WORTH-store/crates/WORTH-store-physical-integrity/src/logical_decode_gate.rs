use crate::{ChecksumCoverageBasis, ExecutedPhysicalChecksum, PreDecodeAdmissionCounters};
use worth_store_physical_format::{
    PhysicalGenerationOwner, PhysicalHeaderDecodeWitness, PhysicalHeaderKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalDecodeGateIdentity {
    header_kind: PhysicalHeaderKind,
    locality: PhysicalGenerationOwner,
    checked_byte_count: u64,
    checksum_value: u64,
    checksum_algorithm: &'static str,
}

impl LogicalDecodeGateIdentity {
    pub(crate) fn new(
        witness: PhysicalHeaderDecodeWitness,
        checked_byte_count: u64,
        checksum: ExecutedPhysicalChecksum,
    ) -> Self {
        Self {
            header_kind: witness.kind(),
            locality: witness.owner(),
            checked_byte_count,
            checksum_value: checksum.value(),
            checksum_algorithm: checksum.algorithm().as_str(),
        }
    }

    pub const fn header_kind(&self) -> PhysicalHeaderKind {
        self.header_kind
    }

    pub const fn locality(&self) -> PhysicalGenerationOwner {
        self.locality
    }

    pub const fn checked_byte_count(&self) -> u64 {
        self.checked_byte_count
    }

    pub const fn checksum_value(&self) -> u64 {
        self.checksum_value
    }

    pub const fn checksum_algorithm(&self) -> &'static str {
        self.checksum_algorithm
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalDecodeGate<'lease> {
    bytes: &'lease [u8],
    witness: PhysicalHeaderDecodeWitness,
    counters: PreDecodeAdmissionCounters,
}

impl<'lease> LogicalDecodeGate<'lease> {
    pub(crate) const fn new(
        bytes: &'lease [u8],
        witness: PhysicalHeaderDecodeWitness,
        counters: PreDecodeAdmissionCounters,
    ) -> Self {
        Self {
            bytes,
            witness,
            counters,
        }
    }

    pub const fn checked_bytes(self) -> &'lease [u8] {
        self.bytes
    }

    pub const fn physical_witness(self) -> PhysicalHeaderDecodeWitness {
        self.witness
    }

    pub const fn counters(self) -> PreDecodeAdmissionCounters {
        self.counters
    }
}

pub trait S3LogicalDecoder<'lease> {
    type Output;

    fn decode(&mut self, gate: LogicalDecodeGate<'lease>) -> Self::Output;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalDecodeGateEvidence {
    identity: LogicalDecodeGateIdentity,
    coverage_basis: ChecksumCoverageBasis,
}

impl LogicalDecodeGateEvidence {
    pub(crate) const fn new(
        identity: LogicalDecodeGateIdentity,
        coverage_basis: ChecksumCoverageBasis,
    ) -> Self {
        Self {
            identity,
            coverage_basis,
        }
    }

    pub const fn identity(&self) -> &LogicalDecodeGateIdentity {
        &self.identity
    }

    pub const fn coverage_basis(&self) -> &ChecksumCoverageBasis {
        &self.coverage_basis
    }
}
