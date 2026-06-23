use super::{
    WorthUiClassifiedRuntimeChange, WorthUiRuntimeChangeActivationPosture,
    WorthUiRuntimeChangeCounters, WorthUiRuntimeChangeEvidenceDigest,
    WorthUiRuntimeChangeFamilyRow, WorthUiRuntimeChangeFamilyStatus, WorthUiRuntimeInstanceWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedRuntimeChangeEvidence {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    posture: WorthUiRuntimeChangeActivationPosture,
    family_rows: Vec<WorthUiRuntimeChangeFamilyRow>,
    digest: WorthUiRuntimeChangeEvidenceDigest,
    counters: WorthUiRuntimeChangeCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRuntimeChangeAdmissionDenial {
    RuntimeInstanceMismatch,
    ActivatedFamilyWithoutChangedFacts,
}

impl WorthUiAdmittedRuntimeChangeEvidence {
    pub(crate) fn admit(
        classified: WorthUiClassifiedRuntimeChange,
        runtime_instance: WorthUiRuntimeInstanceWitness,
    ) -> Result<Self, WorthUiRuntimeChangeAdmissionDenial> {
        if classified.runtime_instance() != runtime_instance {
            return Err(WorthUiRuntimeChangeAdmissionDenial::RuntimeInstanceMismatch);
        }
        if classified.family_rows().iter().any(activated_without_facts) {
            return Err(WorthUiRuntimeChangeAdmissionDenial::ActivatedFamilyWithoutChangedFacts);
        }
        let family_rows = classified.family_rows().to_vec();
        let posture = classified.posture();
        let digest = WorthUiRuntimeChangeEvidenceDigest::from_rows(
            runtime_instance.raw(),
            posture,
            &family_rows,
        );
        let counters = WorthUiRuntimeChangeCounters::from_rows(&family_rows);
        Ok(Self {
            runtime_instance,
            posture,
            family_rows,
            digest,
            counters,
        })
    }

    pub fn runtime_instance(&self) -> WorthUiRuntimeInstanceWitness {
        self.runtime_instance
    }

    pub fn posture(&self) -> WorthUiRuntimeChangeActivationPosture {
        self.posture
    }

    pub fn family_rows(&self) -> &[WorthUiRuntimeChangeFamilyRow] {
        &self.family_rows
    }

    pub fn digest(&self) -> WorthUiRuntimeChangeEvidenceDigest {
        self.digest
    }

    pub fn counters(&self) -> WorthUiRuntimeChangeCounters {
        self.counters
    }
}

fn activated_without_facts(row: &WorthUiRuntimeChangeFamilyRow) -> bool {
    row.status() == WorthUiRuntimeChangeFamilyStatus::Activated && row.changed_facts().is_empty()
}
