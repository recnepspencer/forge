use super::{BooleanEvidenceStageKind, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport};

pub trait BooleanEvidenceReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind;
    fn evidence_identity(&self) -> &str;
    fn evidence_support(&self) -> WorkloadEvidenceSupport;
    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters;
}
