use topology::derived_invalidation_milestone_ten_closeout::DerivedInvalidationMilestoneElevenSeed;

use crate::workload_platform::evidence_ledger::{
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupStageReceiptFamilyIdentity;

use super::query_support::EvidenceLookupQueryAdmissionEvidenceSet;
use super::stage_receipt_identity::EvidenceLookupStageReceiptAdmission;

pub struct EvidenceLookupInputAdmissionRequest<'a> {
    spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
    stage: Option<WorkloadEvidenceStage>,
    receipt_family: Option<EvidenceLookupStageReceiptFamilyIdentity>,
    stage_receipt_digest: Option<String>,
    stage_receipt_spatial_touch_digest: Option<String>,
    topology_seed: Option<&'a DerivedInvalidationMilestoneElevenSeed>,
    query_evidence: Option<EvidenceLookupQueryAdmissionEvidenceSet>,
}

impl<'a> EvidenceLookupInputAdmissionRequest<'a> {
    pub fn from_spatial_touch_authority(
        spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
    ) -> Self {
        Self {
            spatial_touch_authority,
            stage: None,
            receipt_family: None,
            stage_receipt_digest: None,
            stage_receipt_spatial_touch_digest: None,
            topology_seed: None,
            query_evidence: None,
        }
    }

    pub fn with_stage_receipt_family(
        mut self,
        stage: WorkloadEvidenceStage,
        receipt_family: EvidenceLookupStageReceiptFamilyIdentity,
    ) -> Self {
        self.stage = Some(stage);
        self.receipt_family = Some(receipt_family);
        self.stage_receipt_digest =
            Some(self.spatial_touch_authority.evidence_identity().to_string());
        self.stage_receipt_spatial_touch_digest =
            Some(self.spatial_touch_authority.digest().as_str().to_string());
        self
    }

    pub fn with_stage_receipt_identity(
        mut self,
        identity: EvidenceLookupStageReceiptAdmission,
    ) -> Self {
        self.stage = Some(identity.stage());
        self.receipt_family = Some(identity.receipt_family().clone());
        self.stage_receipt_digest = Some(identity.stage_receipt_digest().to_string());
        self.stage_receipt_spatial_touch_digest = Some(identity.spatial_touch_digest().to_string());
        self
    }

    pub fn with_topology_seed(mut self, seed: &'a DerivedInvalidationMilestoneElevenSeed) -> Self {
        self.topology_seed = Some(seed);
        self
    }

    pub fn with_query_import_evidence(
        mut self,
        evidence: EvidenceLookupQueryAdmissionEvidenceSet,
    ) -> Self {
        self.query_evidence = Some(evidence);
        self
    }

    pub(crate) fn spatial_touch_authority(&self) -> &'a SpatialGeometryEvidenceTouchAuthority {
        self.spatial_touch_authority
    }

    pub(crate) const fn stage(&self) -> Option<WorkloadEvidenceStage> {
        self.stage
    }

    pub(crate) fn receipt_family(&self) -> Option<EvidenceLookupStageReceiptFamilyIdentity> {
        self.receipt_family.clone()
    }

    pub(crate) fn stage_receipt_digest(&self) -> Option<&str> {
        self.stage_receipt_digest.as_deref()
    }

    pub(crate) fn stage_receipt_spatial_touch_digest(&self) -> Option<&str> {
        self.stage_receipt_spatial_touch_digest.as_deref()
    }

    pub(crate) const fn topology_seed(&self) -> Option<&'a DerivedInvalidationMilestoneElevenSeed> {
        self.topology_seed
    }

    pub(crate) const fn query_evidence(&self) -> Option<&EvidenceLookupQueryAdmissionEvidenceSet> {
        self.query_evidence.as_ref()
    }
}
