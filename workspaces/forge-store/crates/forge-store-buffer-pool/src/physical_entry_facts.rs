use forge_store_contracts::S2PhysicalSubstrateReadinessSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2PhysicalEntryFacts {
    physical_reference_count: u32,
    header_decode_witness_count: u32,
    payload_admission_witness_count: u32,
    manifest_layout_evidence_count: u32,
    no_materialization_witness_count: u32,
    counter_evidence_count: u32,
}

impl S2PhysicalEntryFacts {
    pub(crate) const fn from_snapshot(snapshot: S2PhysicalSubstrateReadinessSnapshot) -> Self {
        Self {
            physical_reference_count: snapshot.physical_reference_count(),
            header_decode_witness_count: snapshot.header_decode_witness_count(),
            payload_admission_witness_count: snapshot.payload_admission_witness_count(),
            manifest_layout_evidence_count: snapshot.manifest_layout_evidence_count(),
            no_materialization_witness_count: snapshot.no_materialization_witness_count(),
            counter_evidence_count: snapshot.counter_evidence_count(),
        }
    }

    pub const fn physical_reference_count(&self) -> u32 {
        self.physical_reference_count
    }

    pub const fn header_decode_witness_count(&self) -> u32 {
        self.header_decode_witness_count
    }

    pub const fn payload_admission_witness_count(&self) -> u32 {
        self.payload_admission_witness_count
    }

    pub const fn manifest_layout_evidence_count(&self) -> u32 {
        self.manifest_layout_evidence_count
    }

    pub const fn no_materialization_witness_count(&self) -> u32 {
        self.no_materialization_witness_count
    }

    pub const fn counter_evidence_count(&self) -> u32 {
        self.counter_evidence_count
    }
}
