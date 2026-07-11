use crate::{PhysicalSubstrateReadinessDenial, PhysicalSubstrateReadinessDenialKind};
use forge_store_physical_format::{
    PhysicalHeaderDecodeWitness, PhysicalPayloadViewAdmission, PhysicalReference,
};

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
    pub(crate) fn from_handoff_evidence(evidence: PhysicalSubstrateHandoffEvidence) -> Self {
        evidence.facts
    }

    fn from_s1_closeout_counts(
        physical_reference_count: u32,
        header_decode_witness_count: u32,
        payload_admission_witness_count: u32,
        manifest_layout_evidence_count: u32,
        no_materialization_witness_count: u32,
        counter_evidence_count: u32,
    ) -> Result<Self, PhysicalSubstrateReadinessDenial> {
        let facts = Self {
            physical_references: PhysicalSubstrateReadinessFactPosture::from_count(
                physical_reference_count,
            ),
            header_decode_witnesses: PhysicalSubstrateReadinessFactPosture::from_count(
                header_decode_witness_count,
            ),
            payload_admission_witnesses: PhysicalSubstrateReadinessFactPosture::from_count(
                payload_admission_witness_count,
            ),
            manifest_layout_evidence: PhysicalSubstrateReadinessFactPosture::from_count(
                manifest_layout_evidence_count,
            ),
            no_materialization_witness: PhysicalSubstrateReadinessFactPosture::from_count(
                no_materialization_witness_count,
            ),
            counter_evidence: PhysicalSubstrateReadinessFactPosture::from_count(
                counter_evidence_count,
            ),
        };
        facts.require_complete()?;
        Ok(facts)
    }

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

    fn require_complete(&self) -> Result<(), PhysicalSubstrateReadinessDenial> {
        require_present(
            self.physical_references,
            PhysicalSubstrateReadinessDenialKind::MissingPhysicalReferences,
        )?;
        require_present(
            self.header_decode_witnesses,
            PhysicalSubstrateReadinessDenialKind::MissingHeaderDecodeWitnesses,
        )?;
        require_present(
            self.payload_admission_witnesses,
            PhysicalSubstrateReadinessDenialKind::MissingPayloadAdmissionWitnesses,
        )?;
        require_present(
            self.manifest_layout_evidence,
            PhysicalSubstrateReadinessDenialKind::MissingManifestLayoutEvidence,
        )?;
        require_present(
            self.no_materialization_witness,
            PhysicalSubstrateReadinessDenialKind::MissingNoMaterializationWitness,
        )?;
        require_present(
            self.counter_evidence,
            PhysicalSubstrateReadinessDenialKind::MissingCounterEvidence,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSubstrateHandoffEvidence {
    facts: PhysicalSubstrateReadinessFacts,
}

impl PhysicalSubstrateHandoffEvidence {
    pub(crate) fn from_s1_physical_witnesses(
        physical_references: &[PhysicalReference],
        header_decode_witnesses: &[PhysicalHeaderDecodeWitness],
        payload_admission_witnesses: &[PhysicalPayloadViewAdmission<'_>],
        evidence_counts: PhysicalSubstrateEvidenceCounts,
    ) -> Result<Self, PhysicalSubstrateReadinessDenial> {
        let facts = PhysicalSubstrateReadinessFacts::from_s1_closeout_counts(
            physical_references.len() as u32,
            header_decode_witnesses.len() as u32,
            payload_admission_witnesses.len() as u32,
            evidence_counts.manifest_layout_evidence_count(),
            evidence_counts.no_materialization_witness_count(),
            evidence_counts.counter_evidence_count(),
        )?;
        Ok(Self { facts })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSubstrateEvidenceCounts {
    manifest_layout_evidence_count: u32,
    no_materialization_witness_count: u32,
    counter_evidence_count: u32,
}

impl PhysicalSubstrateEvidenceCounts {
    pub(crate) const fn from_s1_closeout_evidence(
        manifest_layout_evidence_count: u32,
        no_materialization_witness_count: u32,
        counter_evidence_count: u32,
    ) -> Self {
        Self {
            manifest_layout_evidence_count,
            no_materialization_witness_count,
            counter_evidence_count,
        }
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

fn require_present(
    posture: PhysicalSubstrateReadinessFactPosture,
    denial: PhysicalSubstrateReadinessDenialKind,
) -> Result<(), PhysicalSubstrateReadinessDenial> {
    if posture.is_present() {
        Ok(())
    } else {
        Err(PhysicalSubstrateReadinessDenial::new(denial))
    }
}
