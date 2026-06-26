use crate::{S2ReadinessDenial, S2ReadinessDenialKind};
use forge_store_physical_format::{
    PhysicalHeaderDecodeWitness, PhysicalPayloadViewAdmission, PhysicalReference,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2PhysicalReadinessFacts {
    physical_references: S2ReadinessFactPosture,
    header_decode_witnesses: S2ReadinessFactPosture,
    payload_admission_witnesses: S2ReadinessFactPosture,
    manifest_layout_evidence: S2ReadinessFactPosture,
    no_materialization_witness: S2ReadinessFactPosture,
    counter_evidence: S2ReadinessFactPosture,
}

impl S2PhysicalReadinessFacts {
    pub(crate) fn from_handoff_evidence(evidence: S2PhysicalSubstrateHandoffEvidence) -> Self {
        evidence.facts
    }

    fn from_s1_closeout_counts(
        physical_reference_count: u32,
        header_decode_witness_count: u32,
        payload_admission_witness_count: u32,
        manifest_layout_evidence_count: u32,
        no_materialization_witness_count: u32,
        counter_evidence_count: u32,
    ) -> Result<Self, S2ReadinessDenial> {
        let facts = Self {
            physical_references: S2ReadinessFactPosture::from_count(physical_reference_count),
            header_decode_witnesses: S2ReadinessFactPosture::from_count(
                header_decode_witness_count,
            ),
            payload_admission_witnesses: S2ReadinessFactPosture::from_count(
                payload_admission_witness_count,
            ),
            manifest_layout_evidence: S2ReadinessFactPosture::from_count(
                manifest_layout_evidence_count,
            ),
            no_materialization_witness: S2ReadinessFactPosture::from_count(
                no_materialization_witness_count,
            ),
            counter_evidence: S2ReadinessFactPosture::from_count(counter_evidence_count),
        };
        facts.require_complete()?;
        Ok(facts)
    }

    pub const fn posture(&self, fact: S2PhysicalReadinessFact) -> S2ReadinessFactPosture {
        match fact {
            S2PhysicalReadinessFact::PhysicalReferences => self.physical_references,
            S2PhysicalReadinessFact::HeaderDecodeWitnesses => self.header_decode_witnesses,
            S2PhysicalReadinessFact::PayloadAdmissionWitnesses => self.payload_admission_witnesses,
            S2PhysicalReadinessFact::ManifestLayoutEvidence => self.manifest_layout_evidence,
            S2PhysicalReadinessFact::NoMaterializationWitness => self.no_materialization_witness,
            S2PhysicalReadinessFact::CounterEvidence => self.counter_evidence,
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

    fn require_complete(&self) -> Result<(), S2ReadinessDenial> {
        require_present(
            self.physical_references,
            S2ReadinessDenialKind::MissingPhysicalReferences,
        )?;
        require_present(
            self.header_decode_witnesses,
            S2ReadinessDenialKind::MissingHeaderDecodeWitnesses,
        )?;
        require_present(
            self.payload_admission_witnesses,
            S2ReadinessDenialKind::MissingPayloadAdmissionWitnesses,
        )?;
        require_present(
            self.manifest_layout_evidence,
            S2ReadinessDenialKind::MissingManifestLayoutEvidence,
        )?;
        require_present(
            self.no_materialization_witness,
            S2ReadinessDenialKind::MissingNoMaterializationWitness,
        )?;
        require_present(
            self.counter_evidence,
            S2ReadinessDenialKind::MissingCounterEvidence,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2PhysicalSubstrateHandoffEvidence {
    facts: S2PhysicalReadinessFacts,
}

impl S2PhysicalSubstrateHandoffEvidence {
    pub(crate) fn from_s1_physical_witnesses(
        physical_references: &[PhysicalReference],
        header_decode_witnesses: &[PhysicalHeaderDecodeWitness],
        payload_admission_witnesses: &[PhysicalPayloadViewAdmission<'_>],
        evidence_counts: S2PhysicalSubstrateEvidenceCounts,
    ) -> Result<Self, S2ReadinessDenial> {
        let facts = S2PhysicalReadinessFacts::from_s1_closeout_counts(
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
pub struct S2PhysicalSubstrateEvidenceCounts {
    manifest_layout_evidence_count: u32,
    no_materialization_witness_count: u32,
    counter_evidence_count: u32,
}

impl S2PhysicalSubstrateEvidenceCounts {
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
pub enum S2PhysicalReadinessFact {
    PhysicalReferences,
    HeaderDecodeWitnesses,
    PayloadAdmissionWitnesses,
    ManifestLayoutEvidence,
    NoMaterializationWitness,
    CounterEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S2ReadinessFactPosture {
    count: u32,
}

impl S2ReadinessFactPosture {
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
    posture: S2ReadinessFactPosture,
    denial: S2ReadinessDenialKind,
) -> Result<(), S2ReadinessDenial> {
    if posture.is_present() {
        Ok(())
    } else {
        Err(S2ReadinessDenial::new(denial))
    }
}
