#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceRow {
    stage: WorkloadEvidenceStage,
    evidence_identity: String,
    backing: WorkloadEvidenceBacking,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
}

impl WorkloadEvidenceRow {
    pub fn new(stage: WorkloadEvidenceStage, evidence_identity: impl Into<String>) -> Self {
        Self {
            stage,
            evidence_identity: evidence_identity.into(),
            backing: WorkloadEvidenceBacking::Manual,
            support: WorkloadEvidenceSupport::Manual,
            counters: WorkloadEvidenceStageCounters::default(),
        }
    }

    pub(crate) fn receipt_backed(
        stage: WorkloadEvidenceStage,
        evidence_identity: impl Into<String>,
        counters: WorkloadEvidenceStageCounters,
    ) -> Self {
        Self {
            stage,
            evidence_identity: evidence_identity.into(),
            backing: WorkloadEvidenceBacking::Receipt,
            support: WorkloadEvidenceSupport::Admitted,
            counters,
        }
    }

    pub(crate) fn receipt_backed_with_support(
        stage: WorkloadEvidenceStage,
        evidence_identity: impl Into<String>,
        support: WorkloadEvidenceSupport,
        counters: WorkloadEvidenceStageCounters,
    ) -> Self {
        Self {
            stage,
            evidence_identity: evidence_identity.into(),
            backing: WorkloadEvidenceBacking::Receipt,
            support,
            counters,
        }
    }

    pub fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn backing(&self) -> WorkloadEvidenceBacking {
        self.backing
    }

    pub fn counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }

    pub fn support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    pub fn is_receipt_backed(&self) -> bool {
        self.backing == WorkloadEvidenceBacking::Receipt
    }

    pub fn is_admitted(&self) -> bool {
        self.support == WorkloadEvidenceSupport::Admitted
    }

    pub fn from_boolean_evidence_receipt(receipt: &impl BooleanEvidenceReceipt) -> Self {
        Self::receipt_backed_with_support(
            receipt.boolean_stage().evidence_stage(),
            receipt.evidence_identity(),
            receipt.evidence_support(),
            receipt.evidence_counters(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadEvidenceBacking {
    Receipt,
    Manual,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadEvidenceSupport {
    Admitted,
    Unsupported,
    Blocked,
    Manual,
}
use super::{BooleanEvidenceReceipt, WorkloadEvidenceStage, WorkloadEvidenceStageCounters};
