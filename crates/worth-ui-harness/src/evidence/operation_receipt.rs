use super::{HarnessEvidenceBasis, HarnessEvidenceBundle, HarnessEvidenceFamily};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessOperationReceipt {
    step_index: usize,
    step_label: String,
    operation_identity: String,
    evidence: HarnessEvidenceBundle,
}

impl HarnessOperationReceipt {
    pub(crate) fn new(
        step_index: usize,
        step_label: impl Into<String>,
        operation_identity: impl Into<String>,
        evidence: HarnessEvidenceBundle,
    ) -> Self {
        Self {
            step_index,
            step_label: step_label.into(),
            operation_identity: operation_identity.into(),
            evidence,
        }
    }

    pub fn step_index(&self) -> usize {
        self.step_index
    }

    pub fn step_label(&self) -> &str {
        &self.step_label
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn evidence(&self) -> &HarnessEvidenceBundle {
        &self.evidence
    }

    pub fn basis(&self) -> Option<HarnessEvidenceBasis> {
        self.evidence.basis()
    }

    pub fn contains(&self, family: HarnessEvidenceFamily) -> bool {
        self.evidence.contains(family)
    }
}
