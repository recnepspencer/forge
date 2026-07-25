use crate::{
    ChecksumCoverageBasis, ExecutedPhysicalChecksum, LogicalDecodeGate, LogicalDecodeGateEvidence,
    LogicalDecodeGateIdentity, PreDecodeAdmissionCounters, ProtectedPhysicalByteView,
};
use worth_store_physical_format::PhysicalHeaderDecodeWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityCheckedPhysicalFormKind {
    Page,
    Frame,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityCheckedPage<'lease> {
    view: ProtectedPhysicalByteView<'lease>,
    witness: PhysicalHeaderDecodeWitness,
    checksum: ExecutedPhysicalChecksum,
    counters: PreDecodeAdmissionCounters,
    evidence: LogicalDecodeGateEvidence,
}

impl<'lease> IntegrityCheckedPage<'lease> {
    pub(crate) fn new(
        view: ProtectedPhysicalByteView<'lease>,
        witness: PhysicalHeaderDecodeWitness,
        checksum: ExecutedPhysicalChecksum,
        counters: PreDecodeAdmissionCounters,
        coverage_basis: ChecksumCoverageBasis,
    ) -> Self {
        let identity = LogicalDecodeGateIdentity::new(witness, view.len_bytes() as u64, checksum);
        let evidence = LogicalDecodeGateEvidence::new(identity, coverage_basis);
        Self {
            view,
            witness,
            checksum,
            counters,
            evidence,
        }
    }

    pub const fn kind(&self) -> IntegrityCheckedPhysicalFormKind {
        IntegrityCheckedPhysicalFormKind::Page
    }

    pub const fn checked_bytes(&self) -> ProtectedPhysicalByteView<'lease> {
        self.view
    }

    pub fn checked_payload_bytes(&self) -> &'lease [u8] {
        checked_payload_bytes(self.view, self.witness)
    }

    pub const fn physical_witness(&self) -> PhysicalHeaderDecodeWitness {
        self.witness
    }

    pub const fn checksum(&self) -> ExecutedPhysicalChecksum {
        self.checksum
    }

    pub const fn counters(&self) -> PreDecodeAdmissionCounters {
        self.counters
    }

    pub const fn gate_evidence(&self) -> &LogicalDecodeGateEvidence {
        &self.evidence
    }

    pub fn logical_decode_gate(&self) -> LogicalDecodeGate<'lease> {
        LogicalDecodeGate::new(self.checked_payload_bytes(), self.witness, self.counters)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityCheckedFrame<'lease> {
    view: ProtectedPhysicalByteView<'lease>,
    witness: PhysicalHeaderDecodeWitness,
    checksum: ExecutedPhysicalChecksum,
    counters: PreDecodeAdmissionCounters,
    evidence: LogicalDecodeGateEvidence,
}

impl<'lease> IntegrityCheckedFrame<'lease> {
    pub(crate) fn new(
        view: ProtectedPhysicalByteView<'lease>,
        witness: PhysicalHeaderDecodeWitness,
        checksum: ExecutedPhysicalChecksum,
        counters: PreDecodeAdmissionCounters,
        coverage_basis: ChecksumCoverageBasis,
    ) -> Self {
        let identity = LogicalDecodeGateIdentity::new(witness, view.len_bytes() as u64, checksum);
        let evidence = LogicalDecodeGateEvidence::new(identity, coverage_basis);
        Self {
            view,
            witness,
            checksum,
            counters,
            evidence,
        }
    }

    pub const fn kind(&self) -> IntegrityCheckedPhysicalFormKind {
        IntegrityCheckedPhysicalFormKind::Frame
    }

    pub const fn checked_bytes(&self) -> ProtectedPhysicalByteView<'lease> {
        self.view
    }

    pub fn checked_payload_bytes(&self) -> &'lease [u8] {
        checked_payload_bytes(self.view, self.witness)
    }

    pub const fn physical_witness(&self) -> PhysicalHeaderDecodeWitness {
        self.witness
    }

    pub const fn checksum(&self) -> ExecutedPhysicalChecksum {
        self.checksum
    }

    pub const fn counters(&self) -> PreDecodeAdmissionCounters {
        self.counters
    }

    pub const fn gate_evidence(&self) -> &LogicalDecodeGateEvidence {
        &self.evidence
    }

    pub fn logical_decode_gate(&self) -> LogicalDecodeGate<'lease> {
        LogicalDecodeGate::new(self.checked_payload_bytes(), self.witness, self.counters)
    }
}

fn checked_payload_bytes<'lease>(
    view: ProtectedPhysicalByteView<'lease>,
    witness: PhysicalHeaderDecodeWitness,
) -> &'lease [u8] {
    let start = witness.payload_offset();
    let end = start + witness.payload_length() as usize;
    &view.as_bytes()[start..end]
}
