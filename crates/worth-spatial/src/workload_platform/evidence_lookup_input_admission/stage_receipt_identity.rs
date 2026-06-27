use crate::workload_platform::evidence_ledger::{
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupStageReceiptFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupStageReceiptAdmission {
    stage: WorkloadEvidenceStage,
    receipt_family: EvidenceLookupStageReceiptFamilyIdentity,
    stage_receipt_digest: String,
    spatial_touch_digest: String,
}

impl EvidenceLookupStageReceiptAdmission {
    pub fn from_spatial_touch_authority(
        authority: &SpatialGeometryEvidenceTouchAuthority,
        receipt_family: EvidenceLookupStageReceiptFamilyIdentity,
    ) -> Self {
        Self {
            stage: authority.evidence_stage(),
            receipt_family,
            stage_receipt_digest: authority.evidence_identity().to_string(),
            spatial_touch_digest: authority.digest().as_str().to_string(),
        }
    }

    pub const fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub const fn receipt_family(&self) -> &EvidenceLookupStageReceiptFamilyIdentity {
        &self.receipt_family
    }

    pub fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }
}
