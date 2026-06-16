use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadEvidenceStageLink {
    stage: WorkloadEvidenceStage,
    evidence_identity: String,
    link_identity: String,
    counters: WorkloadEvidenceStageCounters,
}

impl WorkloadEvidenceStageLink {
    pub(crate) fn new(
        stage: WorkloadEvidenceStage,
        evidence_identity: String,
        link_identity: String,
        counters: WorkloadEvidenceStageCounters,
    ) -> Self {
        Self {
            stage,
            evidence_identity,
            link_identity,
            counters,
        }
    }

    pub fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn link_identity(&self) -> &str {
        &self.link_identity
    }

    pub fn counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }
}
