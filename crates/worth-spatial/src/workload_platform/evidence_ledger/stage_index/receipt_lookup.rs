use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceStageKind, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
    WorkloadEvidenceSupport,
};

use super::lookup_counters::WorkloadEvidenceStageLookupCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceBooleanReceiptLookupProduct {
    boolean_stage: BooleanEvidenceStageKind,
    evidence_stage: WorkloadEvidenceStage,
    evidence_identity: String,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
    lookup_counters: WorkloadEvidenceStageLookupCounters,
    stage_index_identity: String,
}

impl WorkloadEvidenceBooleanReceiptLookupProduct {
    pub(crate) fn new(
        boolean_stage: BooleanEvidenceStageKind,
        evidence_stage: WorkloadEvidenceStage,
        evidence_identity: impl Into<String>,
        support: WorkloadEvidenceSupport,
        counters: WorkloadEvidenceStageCounters,
        stage_index_identity: impl Into<String>,
    ) -> Self {
        Self {
            boolean_stage,
            evidence_stage,
            evidence_identity: evidence_identity.into(),
            support,
            counters,
            lookup_counters: WorkloadEvidenceStageLookupCounters::indexed(1),
            stage_index_identity: stage_index_identity.into(),
        }
    }

    pub fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
    }

    pub fn evidence_stage(&self) -> WorkloadEvidenceStage {
        self.evidence_stage
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    pub fn counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }

    pub fn lookup_counters(&self) -> WorkloadEvidenceStageLookupCounters {
        self.lookup_counters
    }

    pub fn stage_index_identity(&self) -> &str {
        &self.stage_index_identity
    }
}
