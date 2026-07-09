#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2PhysicalSubstrateReadinessSnapshot {
    sealed: bool,
    physical_reference_count: u32,
    header_decode_witness_count: u32,
    payload_admission_witness_count: u32,
    manifest_layout_evidence_count: u32,
    no_materialization_witness_count: u32,
    counter_evidence_count: u32,
}

impl S2PhysicalSubstrateReadinessSnapshot {
    pub const fn from_exact_counts(
        sealed: bool,
        physical_reference_count: u32,
        header_decode_witness_count: u32,
        payload_admission_witness_count: u32,
        manifest_layout_evidence_count: u32,
        no_materialization_witness_count: u32,
        counter_evidence_count: u32,
    ) -> Self {
        Self {
            sealed,
            physical_reference_count,
            header_decode_witness_count,
            payload_admission_witness_count,
            manifest_layout_evidence_count,
            no_materialization_witness_count,
            counter_evidence_count,
        }
    }

    pub const fn is_sealed(&self) -> bool {
        self.sealed
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
