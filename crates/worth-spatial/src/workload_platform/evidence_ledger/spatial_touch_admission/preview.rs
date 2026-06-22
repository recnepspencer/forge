use super::denial::SpatialGeometryEvidenceTouchDenial;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceStageCounters,
    WorkloadEvidenceSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialGeometryEvidenceTouchReceiptOnlyPreview {
    boolean_stage: BooleanEvidenceStageKind,
    evidence_identity: String,
    support: WorkloadEvidenceSupport,
    counters: WorkloadEvidenceStageCounters,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialGeometryEvidenceTouchDiagnosticStatus {
    ReceiptOnly,
}

impl SpatialGeometryEvidenceTouchReceiptOnlyPreview {
    pub(crate) fn from_receipt<T: BooleanEvidenceReceipt + 'static>(receipt: &T) -> Self {
        Self {
            boolean_stage: receipt.boolean_stage(),
            evidence_identity: receipt.evidence_identity().to_string(),
            support: receipt.evidence_support(),
            counters: receipt.evidence_counters(),
        }
    }

    pub fn status(&self) -> SpatialGeometryEvidenceTouchDiagnosticStatus {
        SpatialGeometryEvidenceTouchDiagnosticStatus::ReceiptOnly
    }

    pub fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
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

    pub fn lower_to_query(&self) -> Result<(), SpatialGeometryEvidenceTouchDenial> {
        Err(SpatialGeometryEvidenceTouchDenial::diagnostic_only(
            "receipt-only preview cannot lower to Query",
        ))
    }

    pub fn build_lookup_authority(&self) -> Result<(), SpatialGeometryEvidenceTouchDenial> {
        Err(SpatialGeometryEvidenceTouchDenial::diagnostic_only(
            "receipt-only preview cannot build lookup authority",
        ))
    }

    pub fn satisfy_replay(&self) -> Result<(), SpatialGeometryEvidenceTouchDenial> {
        Err(SpatialGeometryEvidenceTouchDenial::diagnostic_only(
            "receipt-only preview cannot satisfy replay",
        ))
    }

    pub fn pass_closeout(&self) -> Result<(), SpatialGeometryEvidenceTouchDenial> {
        Err(SpatialGeometryEvidenceTouchDenial::diagnostic_only(
            "receipt-only preview cannot pass closeout",
        ))
    }
}
