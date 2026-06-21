use super::{BooleanEvidenceStageKind, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport};

pub(crate) use crate::trusted_boolean_evidence_authority::Seal as BooleanEvidenceReceiptSealed;

#[allow(private_bounds)]
pub trait BooleanEvidenceReceipt: crate::trusted_boolean_evidence_authority::Seal {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind;
    fn evidence_identity(&self) -> &str;
    fn evidence_support(&self) -> WorkloadEvidenceSupport;
    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters;
}

pub trait BooleanEvidenceRowAuthority: BooleanEvidenceReceipt + 'static {}
