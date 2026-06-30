use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayUndoSemanticGraphPriorProofIdentity, ReplayUndoSemanticGraphStageIndexIdentity,
};

use crate::replay_family_catalog::SpatialReplayFamilyIdentity;
use crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority;
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

#[derive(Clone, Copy, Debug)]
pub struct SpatialReplaySemanticGraphPreparationRequest<'a> {
    family_identity: SpatialReplayFamilyIdentity,
    spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
    evidence_lookup_receipt: &'a EvidenceLookupExecutionReceipt,
    lookup_consumed_workload_handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
    retained_replay_receipt: Option<&'a RetainedReplayWorkloadReceipt>,
}

impl<'a> SpatialReplaySemanticGraphPreparationRequest<'a> {
    pub fn new(
        family_identity: SpatialReplayFamilyIdentity,
        spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
        evidence_lookup_receipt: &'a EvidenceLookupExecutionReceipt,
        lookup_consumed_workload_handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
    ) -> Self {
        Self {
            family_identity,
            spatial_touch_authority,
            evidence_lookup_receipt,
            lookup_consumed_workload_handoff,
            retained_replay_receipt: None,
        }
    }

    pub fn with_retained_replay_receipt(
        mut self,
        retained_replay_receipt: &'a RetainedReplayWorkloadReceipt,
    ) -> Self {
        self.retained_replay_receipt = Some(retained_replay_receipt);
        self
    }

    pub const fn family_identity(&self) -> SpatialReplayFamilyIdentity {
        self.family_identity
    }

    pub const fn spatial_touch_authority(&self) -> &'a SpatialGeometryEvidenceTouchAuthority {
        self.spatial_touch_authority
    }

    pub const fn evidence_lookup_receipt(&self) -> &'a EvidenceLookupExecutionReceipt {
        self.evidence_lookup_receipt
    }

    pub const fn lookup_consumed_workload_handoff(
        &self,
    ) -> &'a EvidenceLookupConsumedWorkloadHandoff {
        self.lookup_consumed_workload_handoff
    }

    pub const fn retained_replay_receipt(&self) -> Option<&'a RetainedReplayWorkloadReceipt> {
        self.retained_replay_receipt
    }
}

pub type SpatialReplaySemanticGraphAdmissionRequest<'a> =
    SpatialReplaySemanticGraphPreparationRequest<'a>;

#[derive(Clone, Debug)]
pub struct SpatialReplaySemanticGraphPreparedRequest<'a> {
    family_identity: SpatialReplayFamilyIdentity,
    spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    lookup_consumed_workload_handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
    retained_replay_receipt: Option<&'a RetainedReplayWorkloadReceipt>,
}

impl<'a> SpatialReplaySemanticGraphPreparedRequest<'a> {
    pub(crate) fn new(
        family_identity: SpatialReplayFamilyIdentity,
        spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
        prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
        lookup_consumed_workload_handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
        retained_replay_receipt: Option<&'a RetainedReplayWorkloadReceipt>,
    ) -> Self {
        Self {
            family_identity,
            spatial_touch_authority,
            prior_proof_identity,
            stage_index_identity,
            lookup_consumed_workload_handoff,
            retained_replay_receipt,
        }
    }

    pub const fn family_identity(&self) -> SpatialReplayFamilyIdentity {
        self.family_identity
    }

    pub const fn spatial_touch_authority(&self) -> &'a SpatialGeometryEvidenceTouchAuthority {
        self.spatial_touch_authority
    }

    pub const fn prior_proof_identity(&self) -> &ReplayUndoSemanticGraphPriorProofIdentity {
        &self.prior_proof_identity
    }

    pub const fn stage_index_identity(&self) -> &ReplayUndoSemanticGraphStageIndexIdentity {
        &self.stage_index_identity
    }

    pub const fn lookup_consumed_workload_handoff(
        &self,
    ) -> &'a EvidenceLookupConsumedWorkloadHandoff {
        self.lookup_consumed_workload_handoff
    }

    pub const fn retained_replay_receipt(&self) -> Option<&'a RetainedReplayWorkloadReceipt> {
        self.retained_replay_receipt
    }
}
