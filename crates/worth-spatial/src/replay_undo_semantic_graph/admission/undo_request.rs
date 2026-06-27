use crate::undo_family_catalog::SpatialUndoFamilyIdentity;
use crate::workload_platform::evidence_ledger::{
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStageIndexProduct,
};
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;

#[derive(Clone, Copy, Debug)]
pub struct SpatialUndoSemanticGraphAdmissionRequest<'a> {
    family_identity: SpatialUndoFamilyIdentity,
    spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
    evidence_lookup_receipt: &'a EvidenceLookupExecutionReceipt,
    stage_index_product: &'a WorkloadEvidenceStageIndexProduct,
    lookup_consumed_workload_handoff: Option<&'a EvidenceLookupConsumedWorkloadHandoff>,
}

impl<'a> SpatialUndoSemanticGraphAdmissionRequest<'a> {
    pub fn new(
        family_identity: SpatialUndoFamilyIdentity,
        spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
        evidence_lookup_receipt: &'a EvidenceLookupExecutionReceipt,
        stage_index_product: &'a WorkloadEvidenceStageIndexProduct,
    ) -> Self {
        Self {
            family_identity,
            spatial_touch_authority,
            evidence_lookup_receipt,
            stage_index_product,
            lookup_consumed_workload_handoff: None,
        }
    }

    pub fn with_lookup_consumed_workload_handoff(
        mut self,
        lookup_consumed_workload_handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
    ) -> Self {
        self.lookup_consumed_workload_handoff = Some(lookup_consumed_workload_handoff);
        self
    }

    pub const fn family_identity(&self) -> SpatialUndoFamilyIdentity {
        self.family_identity
    }

    pub const fn spatial_touch_authority(&self) -> &'a SpatialGeometryEvidenceTouchAuthority {
        self.spatial_touch_authority
    }

    pub const fn evidence_lookup_receipt(&self) -> &'a EvidenceLookupExecutionReceipt {
        self.evidence_lookup_receipt
    }

    pub const fn stage_index_product(&self) -> &'a WorkloadEvidenceStageIndexProduct {
        self.stage_index_product
    }

    pub const fn lookup_consumed_workload_handoff(
        &self,
    ) -> Option<&'a EvidenceLookupConsumedWorkloadHandoff> {
        self.lookup_consumed_workload_handoff
    }
}
