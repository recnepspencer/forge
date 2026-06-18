use std::any::TypeId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceRow {
    stage: WorkloadEvidenceStage,
    evidence_identity: String,
    backing: WorkloadEvidenceBacking,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
    receipt_type_id: Option<TypeId>,
    upstream_stage_binding: Option<WorkloadEvidenceStageBinding>,
}

impl WorkloadEvidenceRow {
    pub fn new(stage: WorkloadEvidenceStage, evidence_identity: impl Into<String>) -> Self {
        Self {
            stage,
            evidence_identity: evidence_identity.into(),
            backing: WorkloadEvidenceBacking::Manual,
            support: WorkloadEvidenceSupport::Manual,
            counters: WorkloadEvidenceStageCounters::default(),
            receipt_type_id: None,
            upstream_stage_binding: None,
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
            receipt_type_id: None,
            upstream_stage_binding: None,
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
            receipt_type_id: None,
            upstream_stage_binding: None,
        }
    }

    pub(crate) fn receipt_backed_with_receipt_type<T: 'static>(
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
            receipt_type_id: Some(TypeId::of::<T>()),
            upstream_stage_binding: None,
        }
    }

    pub(crate) fn receipt_backed_with_stage_binding(
        stage: WorkloadEvidenceStage,
        evidence_identity: impl Into<String>,
        counters: WorkloadEvidenceStageCounters,
        upstream_stage_binding: WorkloadEvidenceStageBinding,
    ) -> Self {
        Self {
            stage,
            evidence_identity: evidence_identity.into(),
            backing: WorkloadEvidenceBacking::Receipt,
            support: WorkloadEvidenceSupport::Admitted,
            counters,
            receipt_type_id: None,
            upstream_stage_binding: Some(upstream_stage_binding),
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

    pub fn upstream_stage_binding(&self) -> Option<&WorkloadEvidenceStageBinding> {
        self.upstream_stage_binding.as_ref()
    }

    pub(crate) fn matches_receipt_type<T: 'static>(&self) -> bool {
        self.receipt_type_id == Some(TypeId::of::<T>())
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

    pub fn from_boolean_evidence_receipt<T: BooleanEvidenceRowAuthority>(receipt: &T) -> Self {
        Self::receipt_backed_with_receipt_type::<T>(
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceStageBinding {
    upstream_stage: WorkloadEvidenceStage,
    upstream_evidence_identity: String,
}

impl WorkloadEvidenceStageBinding {
    pub(crate) fn new(
        upstream_stage: WorkloadEvidenceStage,
        upstream_evidence_identity: impl Into<String>,
    ) -> Self {
        Self {
            upstream_stage,
            upstream_evidence_identity: upstream_evidence_identity.into(),
        }
    }

    pub fn upstream_stage(&self) -> WorkloadEvidenceStage {
        self.upstream_stage
    }

    pub fn upstream_evidence_identity(&self) -> &str {
        &self.upstream_evidence_identity
    }
}
use super::{BooleanEvidenceRowAuthority, WorkloadEvidenceStage, WorkloadEvidenceStageCounters};
