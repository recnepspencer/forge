use schema::facade::platform::authority::replay_undo_semantic_graph::{
    ReplayUndoSemanticGraphPriorProofIdentity, ReplayUndoSemanticGraphStageIndexIdentity,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::replay_family_catalog::SpatialReplayFamilyIdentity;
use crate::undo_family_catalog::SpatialUndoFamilyIdentity;
use crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority;
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

#[derive(Debug)]
pub struct SpatialReplaySemanticGraphAdmittedInput<'a> {
    family_identity: SpatialReplayFamilyIdentity,
    spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    lookup_consumed_workload_handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
    retained_replay_receipt: Option<&'a RetainedReplayWorkloadReceipt>,
}

impl<'a> SpatialReplaySemanticGraphAdmittedInput<'a> {
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

    pub fn semantic_graph_identity(&self) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:replay-undo-semantic-graph:admitted-input:v1".to_string(),
                format!("family:{}", self.family_identity.as_str()),
                format!(
                    "spatial-touch-digest:{}",
                    self.spatial_touch_authority.digest().as_str()
                ),
                format!("prior-proof:{}", self.prior_proof_identity.digest()),
                format!("stage-index:{}", self.stage_index_identity.digest()),
                format!(
                    "lookup-handoff:{}",
                    self.lookup_consumed_workload_handoff
                        .semantic_graph_identity()
                ),
                format!(
                    "retained-replay:{}",
                    self.retained_replay_receipt
                        .map(|receipt| receipt.identity().receipt_identity())
                        .unwrap_or_else(|| "not-required".to_string())
                ),
            ],
        )
    }
}

#[derive(Debug)]
pub struct SpatialUndoSemanticGraphAdmittedInput<'a> {
    family_identity: SpatialUndoFamilyIdentity,
    spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
    prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
    lookup_consumed_workload_handoff: Option<&'a EvidenceLookupConsumedWorkloadHandoff>,
    semantic_graph_identity: String,
}

impl<'a> SpatialUndoSemanticGraphAdmittedInput<'a> {
    pub(crate) fn new(
        family_identity: SpatialUndoFamilyIdentity,
        spatial_touch_authority: &'a SpatialGeometryEvidenceTouchAuthority,
        prior_proof_identity: ReplayUndoSemanticGraphPriorProofIdentity,
        stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
        lookup_consumed_workload_handoff: Option<&'a EvidenceLookupConsumedWorkloadHandoff>,
    ) -> Self {
        let semantic_graph_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:replay-undo-semantic-graph:undo-admitted-input:v1".to_string(),
                format!("family:{}", family_identity.as_str()),
                format!(
                    "spatial-touch-digest:{}",
                    spatial_touch_authority.digest().as_str()
                ),
                format!("prior-proof:{}", prior_proof_identity.digest()),
                format!("stage-index:{}", stage_index_identity.digest()),
                format!(
                    "lookup-handoff:{}",
                    lookup_consumed_workload_handoff
                        .map(EvidenceLookupConsumedWorkloadHandoff::semantic_graph_identity)
                        .unwrap_or_else(|| "not-required".to_string())
                ),
            ],
        );
        Self {
            family_identity,
            spatial_touch_authority,
            prior_proof_identity,
            stage_index_identity,
            lookup_consumed_workload_handoff,
            semantic_graph_identity,
        }
    }

    pub const fn family_identity(&self) -> SpatialUndoFamilyIdentity {
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
    ) -> Option<&'a EvidenceLookupConsumedWorkloadHandoff> {
        self.lookup_consumed_workload_handoff
    }

    pub fn semantic_graph_identity(&self) -> &str {
        &self.semantic_graph_identity
    }
}
