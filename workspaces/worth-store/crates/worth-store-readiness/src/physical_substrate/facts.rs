#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSubstrateReadinessFacts {
    physical_references: PhysicalSubstrateReadinessFactPosture,
    header_decode_witnesses: PhysicalSubstrateReadinessFactPosture,
    payload_admission_witnesses: PhysicalSubstrateReadinessFactPosture,
    manifest_layout_evidence: PhysicalSubstrateReadinessFactPosture,
    no_materialization_witness: PhysicalSubstrateReadinessFactPosture,
    counter_evidence: PhysicalSubstrateReadinessFactPosture,
}

impl PhysicalSubstrateReadinessFacts {
    pub const fn posture(
        &self,
        fact: PhysicalSubstrateReadinessFact,
    ) -> PhysicalSubstrateReadinessFactPosture {
        match fact {
            PhysicalSubstrateReadinessFact::PhysicalReferences => self.physical_references,
            PhysicalSubstrateReadinessFact::HeaderDecodeWitnesses => self.header_decode_witnesses,
            PhysicalSubstrateReadinessFact::PayloadAdmissionWitnesses => {
                self.payload_admission_witnesses
            }
            PhysicalSubstrateReadinessFact::ManifestLayoutEvidence => self.manifest_layout_evidence,
            PhysicalSubstrateReadinessFact::NoMaterializationWitness => {
                self.no_materialization_witness
            }
            PhysicalSubstrateReadinessFact::CounterEvidence => self.counter_evidence,
        }
    }

    pub const fn physical_reference_count(&self) -> u32 {
        self.physical_references.count()
    }

    pub const fn header_decode_witness_count(&self) -> u32 {
        self.header_decode_witnesses.count()
    }

    pub const fn payload_admission_witness_count(&self) -> u32 {
        self.payload_admission_witnesses.count()
    }

    pub const fn manifest_layout_evidence_count(&self) -> u32 {
        self.manifest_layout_evidence.count()
    }

    pub const fn no_materialization_witness_count(&self) -> u32 {
        self.no_materialization_witness.count()
    }

    pub const fn counter_evidence_count(&self) -> u32 {
        self.counter_evidence.count()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSubstrateReadinessFact {
    PhysicalReferences,
    HeaderDecodeWitnesses,
    PayloadAdmissionWitnesses,
    ManifestLayoutEvidence,
    NoMaterializationWitness,
    CounterEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSubstrateReadinessFactPosture {
    count: u32,
}

impl PhysicalSubstrateReadinessFactPosture {
    pub const fn from_count(count: u32) -> Self {
        Self { count }
    }

    pub const fn count(&self) -> u32 {
        self.count
    }

    pub const fn is_present(&self) -> bool {
        self.count > 0
    }
}
