use super::replay_request::TopologyReplaySemanticGraphStageReceiptAuthority;
use super::stage_identity::TopologyReplaySemanticGraphStageIdentity;
use crate::replay_family_catalog::TopologyReplayFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyReplaySemanticGraphPreparedStageAuthority {
    family_identity: TopologyReplayFamilyIdentity,
    selected_plan_digest: String,
    touched_closure_digest: String,
    stage_identity: TopologyReplaySemanticGraphStageIdentity,
}

impl TopologyReplaySemanticGraphPreparedStageAuthority {
    pub(crate) fn prepare(
        stage_receipt_authority: TopologyReplaySemanticGraphStageReceiptAuthority<'_>,
    ) -> Self {
        Self {
            family_identity: stage_receipt_authority.family_identity(),
            selected_plan_digest: stage_receipt_authority.selected_plan_digest().to_string(),
            touched_closure_digest: stage_receipt_authority.touched_closure_digest().to_string(),
            stage_identity: TopologyReplaySemanticGraphStageIdentity::from_stage_receipt_authority(
                stage_receipt_authority,
            ),
        }
    }

    pub const fn family_identity(&self) -> TopologyReplayFamilyIdentity {
        self.family_identity
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub const fn stage_identity(&self) -> &TopologyReplaySemanticGraphStageIdentity {
        &self.stage_identity
    }
}
