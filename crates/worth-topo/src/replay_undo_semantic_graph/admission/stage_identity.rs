use crate::derived_topology::invalidation_plan::catalog::catalog_digest;
use crate::replay_undo_semantic_graph::TopologyReplaySemanticGraphStageReceiptAuthority;
use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphStageIndexIdentity;
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_replay_undo_stage_index_identity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyReplaySemanticGraphStageIdentity {
    digest: String,
    stage_index_identity: ReplayUndoSemanticGraphStageIndexIdentity,
}

impl TopologyReplaySemanticGraphStageIdentity {
    pub(crate) fn from_stage_receipt_authority(
        stage_receipt_authority: TopologyReplaySemanticGraphStageReceiptAuthority<'_>,
    ) -> Self {
        let digest = catalog_digest([
            "worth-topo:replay-undo-semantic-graph:topology-stage-identity:v1".to_string(),
            format!(
                "family:{}",
                stage_receipt_authority.family_identity().as_str()
            ),
            format!(
                "selected-plan:{}",
                stage_receipt_authority.selected_plan_digest()
            ),
            format!(
                "touched-closure:{}",
                stage_receipt_authority.touched_closure_digest()
            ),
            format!(
                "native-query-read:{}",
                stage_receipt_authority.native_query_read_receipt_digest()
            ),
        ]);
        Self {
            stage_index_identity: admit_replay_undo_stage_index_identity(&digest),
            digest,
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub const fn stage_index_identity(&self) -> &ReplayUndoSemanticGraphStageIndexIdentity {
        &self.stage_index_identity
    }
}
